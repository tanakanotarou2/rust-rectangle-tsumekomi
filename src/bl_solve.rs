use std::ops::{Index, IndexMut};
use itertools::{concat, Itertools};
// use rand_pcg::Mcg128Xsl64;

use rand::prelude::SliceRandom;
use proconio::{*};
use crate::mat;
use crate::procon_utils::SetMinMax;
use crate::procon_utils::Timer;


#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    // item count
    pub w: usize,
    pub a: Vec<(usize, usize)>, // width, height
}

fn parse_input() -> Input {
    input! {
        n: usize,
        w: usize,
		a: [(usize, usize); n]
	}
    Input { n, w, a }
}

/* NF法 */
pub fn NF_solve(input: &Input) -> Vec<(usize, usize)> {
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
/* NFDH 法 */
pub fn NFDH_solve(input: &Input) -> Vec<(usize, usize)> {
    let mut a = input.a.clone();
    let ids = (0..input.n).sorted_by_key(|&i| -(a[i].1 as i32)).collect_vec();
    a.sort_by_key(|&v| -(v.1 as i32));
    let mut solved = NF_solve(&Input {
        n: input.n,
        w: input.w,
        a,
    });

    solved.into_iter().enumerate().sorted_by_key(|(i, _)| ids[*i]).map(|(_, v)| v).collect_vec()
}

#[derive(Clone, Debug)]
pub struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    right: usize,
    top: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Rect {
        Self {
            x,
            y,
            w,
            h,
            right: x + w,
            top: y + h,
        }
    }
    pub fn area(&mut self) -> usize {
        self.h * self.w
    }
}


pub fn BLF_solve(input: &Input) -> Vec<(usize, usize)> {
    let W: usize = input.w;
    let mut ids = (0..input.n).collect_vec();
    let a = input.a.clone();
    ids.sort_by_key(|&i| -(a[i].1 as i32));

    let mut bl_lst = vec![(0usize, 0usize)]; // BL安定点の候補のリスト

    // 配置済の長方形(k)
    let mut k_lst: Vec<Rect> = vec![]; // x, y, x+w, y+h;


    for ii in 0..input.n {
        let i = ids[ii];
        let (w, h) = a[i];

        // BL安定点の中から配置できる最も右下の座標を探す
        let mut pos: (usize, usize) = (!0, !0);

        // すでに配置済の長方形:k と重ならず, 長方形を配置できるBL安定点
        // println!("i:{} {:?}", i, bl_lst);
        for &bp in bl_lst.iter() { // O(bl_pos)
            let i_r = bp.0 + w;
            let i_t = bp.1 + h;
            if i_r > W { continue; }

            // O(N)
            if k_lst.iter().all(|k| {
                // 区間重複
                let x = bp.0 < k.right && k.x < i_r;
                let y = bp.1 < k.top && k.y < i_t;
                // x、y座標いずれか満たさなければOK
                return !x || !y;
            }) {
                if pos.1 < bp.1 { continue; }
                if pos.1 > bp.1 || pos.0 > bp.0 {
                    pos = bp;
                }
            }
        }
        assert_ne!(pos.0, !0, "{} {}", ii, bl_lst.len());
        // 配置する区画
        let mut place = Rect::new(pos.0, pos.1, w, h);
        // println!("selected place:{:?}", place);

        // 新しいBL安定点候補を追加 O(N)
        //母材と長方形iのBL安定点
        bl_lst.push((place.right, 0));
        bl_lst.push((0, place.top));

        for k in k_lst.iter() {
            // k.y + k.h (=k.3) の左方向へのBL安定点
            // 長方形 i が 長方形 k より上、左側にある場合、BL安定点を追加
            if place.right < k.right && place.top > k.top {
                bl_lst.push((place.right, k.top))
            }

            // k.x + k.w (=k.2) の下方向へのBL安定点
            // 長方形 i が 長方形 k より下、右側にある場合、BL安定点を追加
            if place.top < k.top && place.right > k.right {
                bl_lst.push((k.right, place.top))
            }
        }
        // TODO:追加した四角形と重なるBL安定点の候補は削除

        k_lst.push(place);
    }
    k_lst.into_iter().enumerate().sorted_by_key(|(i, _)| ids[*i]).map(|(_, v)| {
        (v.x, v.y)
    }).collect_vec()
}

pub fn main() {
    Timer::get_time();
    let input = parse_input();
    println!("{:?}", input);
    // let res = NFDH_solve(&input);
    let res = BLF_solve(&input);
    println!("{:?}", res);
}

