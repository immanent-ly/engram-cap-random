//! Built-in random capability plugin. Wraps the allowlisted
//! `wasi:random/random` import and exposes a single `next-u64` provider
//! function. Deterministic replay comes from the runner journaling the
//! result; the plugin itself is a thin pass-through.

#[allow(warnings)]
mod bindings;

use bindings::exports::engram::cap_random::provider::Guest;
use bindings::wasi::random::random;

struct Component;

impl Guest for Component {
    fn next_u64() -> u64 {
        random::get_random_u64()
    }
}

bindings::export!(Component with_types_in bindings);
