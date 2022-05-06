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
    pub a: Vec<(usize, usize)>, // item width, height
}

fn parse_input() -> Input {
    input! {
        n: usize,
        w: usize,
		a: [(usize,usize); n]
	}
    Input { n, w, a }
}

fn NF_solve(input: &Input) -> Vec<(usize, usize)> {
    let N: usize = input.n;
    let W: usize = input.w;
    let mut level_y = (0, 0);
    let mut last_x = 0usize;
    let mut res = vec![];

    for &(w, h) in input.a.iter() {
        if w > W {
            res.push((!0, !0));
            continue;
        }
        if level_y.1 - level_y.0 < h || last_x + w > W {
            level_y = (level_y.1, level_y.1 + h);
            last_x = 0;
        }
        res.push((last_x, level_y.0));
        last_x += w;
    }
    res
}


pub fn solve(input: &Input) -> Vec<(usize, usize)> {
    return NF_solve(&input);
}


pub fn main() {
    // Logger::init();
    Timer::get_time();
    let input = parse_input();
    let res = solve(&input);
    println!("{:?}", res);
}

