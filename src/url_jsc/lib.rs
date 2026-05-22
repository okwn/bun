//! JSC bridges for `url/url.zig` `URL`. The struct + parser stay in `url/`.

#![warn(unused_must_use)]

use bun_core::{String as BunString, Tag};
use bun_url::OwnedURL;

// ── bun_jsc surface ──────────────────────────────────────────────────────
// bun_jsc is green now; re-export the real opaque handles so downstream
// callers see the same types the rest of the JSC layer uses.
pub use bun_jsc::{JSGlobalObject, JSValue};

pub fn url_from_js(
    js_value: JSValue,
    global: &JSGlobalObject,
) -> Result<OwnedURL, bun_core::Error> {
    let href: BunString = bun_jsc::URL::href_from_js(js_value, global)
        .map_err(|_| bun_core::err!(JSError))?;
    if href.tag() == Tag::Dead {
        return Err(bun_core::err!(InvalidURL));
    }
    let owned = href.to_owned_slice().into_boxed_slice();
    href.deref();
    Ok(OwnedURL::from_href(owned))
}

// ported from: src/url_jsc/url_jsc.zig
