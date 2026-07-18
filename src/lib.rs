//! Official engram capability `random`. Provides the guest with randomness from
//! the host. Exports the `engram:cap-random/provider` interface; imports are the
//! allowlisted WASI interfaces declared in `wit/world.wit`.

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
