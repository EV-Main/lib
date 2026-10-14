// Hostnames that never leave the machine. Plain http is tolerated only there:
// a dev stand has no certificate, while in production a bundle, link or return
// address over http can be rewritten by anyone on the path.
const LOOPBACK = /^(?:localhost|127(?:\.\d{1,3}){3}|\[::1\])$/;

/**
 * Whether an absolute `url` may carry a bundle, a link or a return address:
 * `https:`, or `http:` to a loopback host (a dev stand). Everything else —
 * http to a real host, `javascript:`, `data:`, `ftp:` — is refused.
 */
export function isSecureTransport(url: URL): boolean {
  if (url.protocol === 'https:') return true;
  return url.protocol === 'http:' && LOOPBACK.test(url.hostname);
}
