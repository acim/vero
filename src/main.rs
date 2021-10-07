//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

// use mysql_async::prelude::*;

mod storage;

fn main() {
    let pool = mysql_async::Pool::new("");
    println!("Hello, world!");
}
