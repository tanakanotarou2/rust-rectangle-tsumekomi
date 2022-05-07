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
    pub a: Vec<(usize, usize, usize)>, // item no, width, height
}

fn parse_input() -> Input {
    input! {
        n: usize,
        w: usize,
		tmp: [(usize, usize); n]
	}
    let a = tmp.into_iter().enumerate().map(|(i, v)| (i, v.0, v.1)).collect_vec();
    Input { n, w, a }
}

/* NF法 */
pub fn NF_solve(input: &Input) -> (usize, Vec<(usize, usize, usize)>) {
    let N: usize = input.n;
    let W: usize = input.w;
    let mut level_y = (0, 0);
    let mut last_x = 0usize;
    let mut res = vec![];

    for &(no, w, h) in input.a.iter() {
        if w > W {
            res.push((no, !0, !0));
            continue;
        }
        if level_y.1 - level_y.0 < h || last_x + w > W {
            level_y = (level_y.1, level_y.1 + h);
            last_x = 0;
        }
        res.push((no, last_x, level_y.0));
        last_x += w;
    }
    (level_y.1, res)
}
/* NFDH 法 */
pub fn NFDH_solve(input: &Input) -> (usize, Vec<(usize, usize, usize)>) {
    let mut a = input.a.clone();
    a.sort_by_key(|&v| -(v.2 as i32));
    return NF_solve(&Input {
        n: input.n,
        w: input.w,
        a,
    });
}


pub fn main() {
    // Logger::init();
    Timer::get_time();
    let input = parse_input();
    // println!("{:?}",input);
    let res = NF_solve(&input);
    println!("{:?}", res);
}

