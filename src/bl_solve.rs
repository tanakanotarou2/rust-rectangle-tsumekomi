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


pub fn BLF_solve(input: &Input) -> (usize, Vec<(usize, usize, usize)>) {
    let W: usize = input.w;
    let mut ids = (0..input.n).collect_vec();
    let a = input.a.clone();
    ids.sort_by_key(|&i| -(a[i].2 as i32));

    let mut bl_lst = vec![(0usize, 0usize)]; // BL安定点の候補のリスト

    //TODO: わかりにくいので長方形の構造体作るほうがよいかもしれません。
    // 配置済の長方形(k)
    let mut k_lst: Vec<(usize, usize, usize, usize)> = vec![]; // x, y, x+w, y+h;


    for ii in 0..input.n {
        let i = ids[ii];
        let (_, w, h) = a[i];

        // BL安定点の中から配置できる最も右下の座標を探す
        let mut pos: (usize, usize) = (!0, !0);

        // すでに配置済の長方形:k と重ならず, 長方形を配置できるBL安定点
        println!("i:{} {:?}", i, bl_lst);
        for &bp in bl_lst.iter() { // O(bl_pos)
            let i_r = bp.0 + w;
            let i_t = bp.1 + h;
            if i_r > W { continue; }

            if k_lst.iter().all(|&k| {
                // 区間重複
                let x = bp.0 < k.2 && k.0 < i_r;
                let y = bp.1 < k.3 && k.1 < i_t;
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
        // 配置するBL安定点の確定
        let mut pos = (pos.0, pos.1, pos.0 + w, pos.1 + h);
        println!("selected pos:{:?}", pos);

        // 新しいBL安定点を追加 O(N)
        //母材と長方形iのBL安定点
        bl_lst.push((pos.2, 0));
        bl_lst.push((0, pos.3));

        for &k in k_lst.iter() {
            // k.y + k.h (=k.3) の左方向へのBL安定点
            // 長方形 i が 長方形 k より上、左側にある場合、BL安定点を追加
            if pos.2 < k.2 && pos.3 > k.3 {
                bl_lst.push((pos.2, k.3))
            }

            // k.x + k.w (=k.2) の下方向へのBL安定点
            // 長方形 i が 長方形 k より下、右側にある場合、BL安定点を追加
            if pos.3 < k.3 && pos.2 > k.2 {
                bl_lst.push((k.2, pos.3))
            }
        }
        k_lst.push((pos.0, pos.1, pos.2, pos.3));
    }

    let mut max_height = 0;
    let mut res = vec![];
    for i in 0..input.n {
        let no = ids[i];
        res.push((no as usize, k_lst[i].0, k_lst[i].1));
        max_height.chmax(k_lst[i].3);
    }
    (max_height, res)
}


pub fn main() {
    Timer::get_time();
    let input = parse_input();
    // let res = NF_solve(&input);
    let res = BLF_solve(&input);
    println!("{:?}", res);
}

