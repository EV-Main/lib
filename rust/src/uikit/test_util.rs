use std::rc::Rc;

use dioxus::{
	dioxus_core::{ElementId, Mutation, Mutations},
	html::{
		PlatformEventData, SerializedFormData,
		input_data::keyboard_types::{Code, Location, Modifiers},
	},
	prelude::*,
};

/// Upper bound for the event sweeps — larger than any component tree the uikit
/// tests build.
const MAX_ELEMENTS: usize = 32;
/// Renders a component to a static HTML string via `dioxus-ssr`, so tests can
/// assert on the emitted classes and structure without a browser.
pub fn render(app: fn() -> Element) -> String {
	let mut dom = VirtualDom::new(app);
	dom.rebuild_in_place();
	dioxus_ssr::render(&dom)
}

/// Renders once effects have run. `rebuild_in_place` leaves them queued, so a
/// component that registers itself through `use_effect` is invisible to plain
/// [`render`]; `render_immediate` runs them and re-renders whoever they dirtied.
pub fn render_with_effects(app: fn() -> Element) -> String {
	let mut dom = VirtualDom::new(app);
	dom.rebuild_in_place();
	// Twice: the first pass runs the effects, the second lets whatever they
	// dirtied settle.
	for _ in 0..2 {
		dom.render_immediate(&mut dioxus::dioxus_core::NoOpMutations);
	}
	dioxus_ssr::render(&dom)
}

/// Fires a `click` at every mounted element, so a test can assert that a
/// handler ran without hardcoding a brittle `ElementId`. The component's own
/// handlers record the hits.
pub fn click_every_element(app: fn() -> Element) {
	let mut dom = mount(app);
	sweep(&mut dom, "click", || Box::new(dioxus::html::SerializedMouseData::default()));
}

/// Fires a `click` at every mounted element in mount order, then renders — so a
/// test can assert the state a sequence of presses left behind. No render runs
/// between clicks; a handler that reads its state at click time (a signal, not
/// a value captured at render) still sees the earlier presses.
pub fn render_after_click(app: fn() -> Element) -> String {
	let mut dom = mount(app);
	sweep(&mut dom, "click", || Box::new(dioxus::html::SerializedMouseData::default()));
	dom.render_immediate(&mut dioxus::dioxus_core::NoOpMutations);
	dioxus_ssr::render(&dom)
}

/// Fires a `focus` at every mounted element, then renders — so a test can
/// assert on the markup a component shows only while focused.
pub fn render_focused(app: fn() -> Element) -> String {
	let mut dom = mount(app);
	sweep(&mut dom, "focus", || Box::new(dioxus::html::SerializedFocusData::default()));
	dom.render_immediate(&mut dioxus::dioxus_core::NoOpMutations);
	dioxus_ssr::render(&dom)
}

/// Fires a `keydown` for `key` at every mounted element, then renders — so a
/// test can assert what a component's key handling actually did, rather than
/// re-asserting the markup around it. Only elements listening for `keydown`
/// react; the event does not bubble, so a handler sees exactly one press.
pub fn render_after_keydown(app: fn() -> Element, key: Key) -> String {
	render_after_keys(app, &[key])
}

/// [`render_after_keydown`] for a sequence: each key is swept and rendered
/// before the next, so the second press meets whatever the first one opened.
pub fn render_after_keys(app: fn() -> Element, keys: &[Key]) -> String {
	let mut dom = mount(app);
	for key in keys {
		sweep(&mut dom, "keydown", || {
			Box::new(dioxus::html::SerializedKeyboardData::new(
				key.clone(),
				Code::Unidentified,
				Location::Standard,
				false,
				Modifiers::empty(),
				false,
			))
		});
		dom.render_immediate(&mut dioxus::dioxus_core::NoOpMutations);
	}
	dioxus_ssr::render(&dom)
}

/// One sweep of [`render_after_steps`].
pub enum Step {
	Key(Key),
	Focus,
}

/// Sweeps each step at every mounted element and renders after it — for a
/// sequence that mixes keys with focus moves.
pub fn render_after_steps(app: fn() -> Element, steps: &[Step]) -> String {
	let mut dom = mount(app);
	for step in steps {
		match step {
			Step::Key(key) => sweep(&mut dom, "keydown", || {
				Box::new(dioxus::html::SerializedKeyboardData::new(
					key.clone(),
					Code::Unidentified,
					Location::Standard,
					false,
					Modifiers::empty(),
					false,
				))
			}),
			Step::Focus => sweep(&mut dom, "focus", || Box::new(dioxus::html::SerializedFocusData::default())),
		}
		dom.render_immediate(&mut dioxus::dioxus_core::NoOpMutations);
	}
	dioxus_ssr::render(&dom)
}

/// Fires an `input` carrying `value` at every mounted element, then renders and
/// returns the mutations that render produced — so a test can assert what a
/// controlled field writes back to the DOM, which the SSR string cannot show
/// (the string is the same whether or not the DOM was patched).
pub fn mutations_after_input(app: fn() -> Element, value: &str) -> Vec<Mutation> {
	let mut dom = mount(app);
	let value = value.to_owned();
	sweep(&mut dom, "input", move || Box::new(SerializedFormData::new(value.clone(), Vec::new())));
	let mut mutations = Mutations::default();
	dom.render_immediate(&mut mutations);
	mutations.edits
}

/// Fires each of `keys` as a `keydown` at every mounted element, like
/// [`render_after_keydown`], but only once effects have settled — before the
/// first key and after each — for a component whose parts register through
/// `use_effect` and would otherwise meet the keys unregistered.
pub fn render_after_settled_keys(app: fn() -> Element, keys: &[Key]) -> String {
	keys_settling(app, keys, true)
}
/// [`render_after_settled_keys`] without settling before the first key: it
/// meets the first render's effects still queued — for a test of what happens
/// to them when that key unmounts their component.
pub fn render_after_keys_on_queued_effects(app: fn() -> Element, keys: &[Key]) -> String {
	keys_settling(app, keys, false)
}
/// A mounted app that fires one event at one element, for a contract that a
/// sweep at every element would blur: a scrim, a cancel and a confirm all
/// close a dialog, so only a click on one of them says which one did it.
///
/// Elements are found by what the mutations reveal about them: the events they
/// listen for, and their dynamic attributes. Effects settle after every event.
pub struct Harness {
	dom: VirtualDom,
	listeners: Vec<(String, ElementId)>,
	attributes: Vec<(&'static str, ElementId)>,
}
impl Harness {
	pub fn new(app: fn() -> Element) -> Self {
		dioxus::html::set_event_converter(Box::new(dioxus::html::SerializedHtmlEventConverter));
		let mut harness = Self {
			dom: VirtualDom::new(app),
			listeners: Vec::new(),
			attributes: Vec::new(),
		};
		let mut mutations = Mutations::default();
		harness.dom.rebuild(&mut mutations);
		harness.record(mutations);
		harness.settle();
		harness
	}

	/// The elements that listen for `event`, in the order they were created.
	pub fn listening(&self, event: &str) -> Vec<ElementId> {
		dedup(self.listeners.iter().filter(|(name, _)| name == event).map(|(_, id)| *id))
	}

	/// The elements that carry the dynamic attribute `name`, in the order they
	/// were created.
	pub fn with_attribute(&self, name: &str) -> Vec<ElementId> {
		dedup(self.attributes.iter().filter(|(n, _)| *n == name).map(|(_, id)| *id))
	}

	pub fn click(&mut self, id: ElementId) {
		self.fire("click", id, Box::new(dioxus::html::SerializedMouseData::default()));
	}

	pub fn key(&mut self, id: ElementId, key: Key) {
		let data = dioxus::html::SerializedKeyboardData::new(key, Code::Unidentified, Location::Standard, false, Modifiers::empty(), false);
		self.fire("keydown", id, Box::new(data));
	}

	/// Fires an `input` that leaves `value` in the field.
	pub fn input(&mut self, id: ElementId, value: &str) {
		self.fire("input", id, Box::new(SerializedFormData::new(value.to_owned(), Vec::new())));
	}

	/// Fires a form event (`submit`, `reset`) with no field values.
	pub fn form(&mut self, event: &str, id: ElementId) {
		self.fire(event, id, Box::new(SerializedFormData::new(String::new(), Vec::new())));
	}

	pub fn html(&self) -> String {
		dioxus_ssr::render(&self.dom)
	}

	fn fire(&mut self, name: &str, id: ElementId, data: Box<dyn std::any::Any>) {
		let event = Rc::new(PlatformEventData::new(data));
		self.dom.runtime().handle_event(name, Event::new(event, false), id);
		self.settle();
	}

	fn settle(&mut self) {
		// Three passes, one more than `render_with_effects`: an effect that a
		// prop change re-runs is woken by the render, queued on the next pass,
		// and what it dirtied renders on the one after.
		for _ in 0..3 {
			let mut mutations = Mutations::default();
			self.dom.render_immediate(&mut mutations);
			self.record(mutations);
		}
	}

	fn record(&mut self, mutations: Mutations) {
		for edit in mutations.edits {
			match edit {
				Mutation::NewEventListener { name, id } => self.listeners.push((name, id)),
				Mutation::SetAttribute { name, id, .. } => self.attributes.push((name, id)),
				_ => {}
			}
		}
	}
}
fn dedup(ids: impl Iterator<Item = ElementId>) -> Vec<ElementId> {
	let mut out: Vec<ElementId> = Vec::new();
	for id in ids {
		if !out.contains(&id) {
			out.push(id);
		}
	}
	out
}

fn mount(app: fn() -> Element) -> VirtualDom {
	// Listeners receive `PlatformEventData` and a converter turns it back into
	// the concrete data. The web/desktop platforms install one; under
	// `dioxus-ssr` there is none, hence the serialized converter.
	dioxus::html::set_event_converter(Box::new(dioxus::html::SerializedHtmlEventConverter));
	let mut dom = VirtualDom::new(app);
	dom.rebuild_in_place();
	dom
}

/// Dispatches `name` at every element id. The dom is not re-rendered mid-sweep,
/// so every element stays mounted throughout.
fn sweep(dom: &mut VirtualDom, name: &str, data: impl Fn() -> Box<dyn std::any::Any>) {
	let runtime = dom.runtime();
	for id in 1..MAX_ELEMENTS {
		let event = Rc::new(PlatformEventData::new(data()));
		runtime.handle_event(name, Event::new(event, false), dioxus::dioxus_core::ElementId(id));
	}
}

fn keys_settling(app: fn() -> Element, keys: &[Key], settle_first: bool) -> String {
	let mut dom = mount(app);
	let settle = |dom: &mut VirtualDom| {
		// Twice, as in `render_with_effects`: run the effects, then let what they dirtied render.
		for _ in 0..2 {
			dom.render_immediate(&mut dioxus::dioxus_core::NoOpMutations);
		}
	};
	if settle_first {
		settle(&mut dom);
	}
	for key in keys {
		sweep(&mut dom, "keydown", || {
			Box::new(dioxus::html::SerializedKeyboardData::new(
				key.clone(),
				Code::Unidentified,
				Location::Standard,
				false,
				Modifiers::empty(),
				false,
			))
		});
		settle(&mut dom);
	}
	dioxus_ssr::render(&dom)
}
