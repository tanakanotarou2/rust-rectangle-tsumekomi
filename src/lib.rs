extern crate console_error_panic_hook;
mod utils;
mod procon_utils;
pub mod bl_solve;

use wasm_bindgen::prelude::*;
use crate::bl_solve::Input;

/**
 * vector 初期化
 * e.g.)
 *  mat![1,2,3]; // [1, 2, 3]
 *  mat![false; N; M]; // false で初期化された N * M の vector
 */
#[macro_export] // declared in the crate root scope
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-tsumekomi!");
}


use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct SolverRes {
    pub field: Vec<(i32, i32)>,
}

#[wasm_bindgen]
pub fn solve() -> JsValue {
    console_error_panic_hook::set_once();
    let mut input = Input { n: 3, w: 3, a: vec![(1, 2)] };
    let res = bl_solve::solve(&input);
    let res=SolverRes {
        field: vec![(1, 2), (3, 4)]
    };

    JsValue::from_serde(&res).unwrap()

}