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
    pub height: usize,
    pub pos_list: Vec<(usize, usize, usize, usize, usize)>, // no, x,y,width,height
}

#[derive(Serialize, Deserialize)]
pub struct SolverInp {
    pub width: usize,
    pub squares: Vec<(usize, usize, usize)>, // no, width,height
}

pub fn solve<F: Fn(&Input) -> (usize, Vec<(usize, usize, usize)>)>(jsVal: &JsValue, solver: F) -> JsValue {
    console_error_panic_hook::set_once(); // エラーがあった場合にログ出力
    let inp: SolverInp = jsVal.into_serde().unwrap();
    let mut input = Input {
        n: inp.squares.len(),
        w: inp.width,
        a: inp.squares.clone(),
    };

    let (max_height,pos) = solver(&input);
    let mut res = SolverRes {
        height: max_height,
        pos_list: vec![(0, 0, 0, 0, 0); input.n],
    };
    for a in input.a.into_iter() {
        res.pos_list[a.0].0 = a.0;
        res.pos_list[a.0].3 = a.1;
        res.pos_list[a.0].4 = a.2;
    }
    for p in pos {
        res.pos_list[p.0].1 = p.1;
        res.pos_list[p.0].2 = p.2;
    }
    JsValue::from_serde(&res).unwrap()
}

#[wasm_bindgen]
pub fn NF_solve(jsVal: &JsValue) -> JsValue {
    console_error_panic_hook::set_once(); // エラーがあった場合にログ出力
    return solve(jsVal, bl_solve::NF_solve);
}

#[wasm_bindgen]
pub fn NFDH_solve(jsVal: &JsValue) -> JsValue {
    console_error_panic_hook::set_once(); // エラーがあった場合にログ出力
    return solve(jsVal, bl_solve::NFDH_solve);
}
