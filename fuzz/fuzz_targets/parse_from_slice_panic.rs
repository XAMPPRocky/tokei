#![no_main]
use libfuzzer_sys::fuzz_target;

mod parse_from_slice;
use parse_from_slice::{parse_from_slice, FuzzInput};

fuzz_target!(|data: FuzzInput| {
    parse_from_slice(data, false);
});
