#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(::std::fmt::Arguments::new_v1(
            &["", "\n"],
            &match (&{ [1, 2, 3][4] },) {
                (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)],
            },
        ));
    };
}
