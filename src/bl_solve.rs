use std::ops::{Index, IndexMut};
use itertools::{concat, Itertools};
// use rand_pcg::Mcg128Xsl64;

use proconio::{*};
use crate::mat;
use crate::procon_utils::SetMinMax;
use crate::procon_utils::Timer;


#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    // item count
    pub w: usize,
    // width
    pub a: Vec<(usize, usize)>,
}

fn parse_input() -> Input {
    input! {
        n: usize,
        w: usize,
		a: [(usize,usize); n]
	}
    Input { n, w, a }
}

pub fn solve(input: &Input) -> Vec<i32> {
    Timer::get_time();
    while Timer::get_time() < 1.0 {
        // wait
    }
    return mat![0i32;3];
}


pub fn main() {
    // Logger::init();
    Timer::get_time();
    // let input=parse_input();
    // println!("{:?}",input);
    // let input = parse_input();
    // println!("{}", out.iter().join(""));
    let mut x: i32 = 50;
    x.chmax(100);
    println!("{}", x);
}

