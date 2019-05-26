#[macro_use(assert_matches)]
extern crate assert_matches;
#[macro_use(bson, doc)]
extern crate bson;
extern crate byteorder;
extern crate chrono;
// #[cfg(not(target_arch = "wasm32"))]
// extern crate decimal;
// #[cfg(target_arch = "wasm32")]
extern crate decimal128 as dec128;
extern crate hex;
extern crate serde_json;

mod modules;
