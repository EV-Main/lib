import * as React from "react";
import { Container } from "../atomic/container";
import * as copy from "../../generated/copy";
import type { FooterLink, FooterLinkGroup, FooterOffice } from "../../generated/footer";
import { FOOTER_BRAND, FOOTER_COPYRIGHT, FOOTER_DESCRIPTION, FOOTER_GRID, FOOTER_HEADING, FOOTER_LEGAL_LINK, FOOTER_LEGAL_LINKS, FOOTER_LEGAL_SEPARATOR, FOOTER_LOCKUP, FOOTER_LOCKUP_TEXT, FOOTER_NAV_COLUMN, FOOTER_NAV_LINK, FOOTER_NAV_LIST, FOOTER_NEWSLETTER_BLURB, FOOTER_NEWSLETTER_COLUMN, FOOTER_OFFICES_LIST, FOOTER_OFFICE_NAME, FOOTER_ROOT, FOOTER_TAGLINE, FOOTER_VERSION, FOOTER_WIDE_COLUMN } from "../../generated/footer";

export type { FooterLink, FooterLinkGroup, FooterOffice };

export interface FooterProps {
  /** The whole lock-up; replaces the one `mark` + `brand` + `tagline` compose. */
  lockup?: React.ReactNode;
  /** The wordmark beside the mark, set in the display family. */
  brand?: string;
  /** The mark beside the brand name. Sized by the caller. */
  mark?: React.ReactNode;
  tagline?: string;
  description?: React.ReactNode;
  copyright?: React.ReactNode;
  nav?: readonly FooterLinkGroup[];
  offices?: readonly FooterOffice[];
  officesHeading?: string;
  legalLinks?: readonly FooterLink[];
  /** The newsletter form; the newsletter column renders only when present. */
  newsletter?: React.ReactNode;
  newsletterHeading?: string;
  newsletterBlurb?: string;
  version?: string;
  commitHref?: string;
  linkComponent?: React.ElementType;
  children?: React.ReactNode;
}

const HEADING = FOOTER_HEADING;

/**
 * A 12-col footer grid — brand 3 | sitemap groups 2 each | offices 3 |
 * newsletter 2 — with the legal links and copyright line. On mobile the sitemap
 * columns sit side by side. The kit brings no content of its own: every string,
 * the mark and the lock-up are the caller's, and each column renders only when
 * it has something in it. `children` render right after the `<footer>` tag —
 * the slot for app-side extras like a build-version easter egg.
 *
 * The lock-up is either composed from `mark` + `brand` + `tagline`, or handed
 * over whole as `lockup` when a brand's arrangement is not that one.
 *
 * `linkComponent` lets Next hosts pass `next/link`; everyone else gets `<a>`.
 */
export function Footer({
  lockup,
  brand,
  mark,
  tagline,
  description,
  copyright,
  nav = [],
  offices = [],
  officesHeading = copy.FOOTER_OFFICES,
  legalLinks = [],
  newsletter,
  newsletterHeading = copy.FOOTER_NEWSLETTER,
  newsletterBlurb = "",
  version,
  commitHref,
  linkComponent,
  children,
}: FooterProps) {
  const L = linkComponent ?? "a";
  const composed = brand !== undefined || mark !== undefined;
  const hasBrandColumn = lockup !== undefined || composed || description !== undefined || legalLinks.length > 0;

  return (
    <footer data-slot="footer" className={FOOTER_ROOT}>
      {children}
      <Container>
        <div className={FOOTER_GRID}>
          {hasBrandColumn && (
            <div className={FOOTER_WIDE_COLUMN}>
              {lockup !== undefined ? (
                <div data-slot="footer-lockup" className="mb-6">{lockup}</div>
              ) : (
                composed && <FooterLockup brand={brand} mark={mark} tagline={tagline} />
              )}
              {description !== undefined && (
                <p className={FOOTER_DESCRIPTION}>
                  {description}
                </p>
              )}
              {legalLinks.length > 0 && (
                <div className={FOOTER_LEGAL_LINKS}>
                  {legalLinks.map((link, i) => (
                    <React.Fragment key={link.label}>
                      {i > 0 && <span className={FOOTER_LEGAL_SEPARATOR}>|</span>}
                      <L href={link.href} className={FOOTER_LEGAL_LINK}>
                        {link.label}
                      </L>
                    </React.Fragment>
                  ))}
                </div>
              )}
            </div>
          )}

          {nav.map((group) => (
            <nav key={group.heading} aria-label={`Footer ${group.heading} links`} className={FOOTER_NAV_COLUMN}>
              <h4 className={HEADING}>{group.heading}</h4>
              <ul className={FOOTER_NAV_LIST}>
                {group.links.map((link) => (
                  <li key={link.href}>
                    <L
                      href={link.href}
                      className={FOOTER_NAV_LINK}
                    >
                      {link.label}
                    </L>
                  </li>
                ))}
              </ul>
            </nav>
          ))}

          {offices.length > 0 && (
            <div className={FOOTER_WIDE_COLUMN}>
              <h4 className={HEADING}>{officesHeading}</h4>
              <ul className={FOOTER_OFFICES_LIST}>
                {offices.map((office) => (
                  <li key={office.name}>
                    <strong className={FOOTER_OFFICE_NAME}>
                      {office.name}
                    </strong>
                    {office.address}
                  </li>
                ))}
              </ul>
            </div>
          )}

          {newsletter && (
            <div className={FOOTER_NEWSLETTER_COLUMN}>
              <h4 className={HEADING}>{newsletterHeading}</h4>
              {/* What the source's <Tier tier="alt"><Text> emits: alt body size
                  + the info variant, then the mb-4 the caller adds. */}
              <p className={FOOTER_NEWSLETTER_BLURB}>
                {newsletterBlurb}
              </p>
              {newsletter}
            </div>
          )}
        </div>

        {(copyright !== undefined || version !== undefined) && (
          <div className={FOOTER_COPYRIGHT}>
            <p>
              {copyright}
              {version && (
                <>
                  {copyright !== undefined && " "}
                  <a href={commitHref} className={FOOTER_VERSION}>
                    {version}
                  </a>
                </>
              )}
            </p>
          </div>
        )}
      </Container>
    </footer>
  );
}

function FooterLockup({
  brand,
  mark,
  tagline,
}: {
  brand: string | undefined;
  mark: React.ReactNode;
  tagline: string | undefined;
}) {
  return (
    <div data-slot="footer-lockup" className={FOOTER_LOCKUP}>
      {mark}
      {(brand !== undefined || tagline !== undefined) && (
        <div className={FOOTER_LOCKUP_TEXT}>
          {brand !== undefined && (
            <span className={FOOTER_BRAND}>{brand}</span>
          )}
          {tagline !== undefined && (
            <span className={FOOTER_TAGLINE}>
              {tagline}
            </span>
          )}
        </div>
      )}
    </div>
  );
}
