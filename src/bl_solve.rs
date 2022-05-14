use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::ops::{Index, IndexMut};
use itertools::{concat, Itertools};
// use rand_pcg::Mcg128Xsl64;
use std::collections::{BTreeSet, HashMap};
use rand_pcg::Mcg128Xsl64;
use rand::prelude::SliceRandom;
use proconio::{*};
use rand::Rng;
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

    /* 交差判定 */
    pub fn is_intersection(&self, rhs: &Rect) -> bool {
        let x = self.x < rhs.right && rhs.x < self.right;
        let y = self.y < rhs.top && rhs.y < self.top;
        x && y
    }
}

/**
 * 与えられた順に詰め込む
 */
fn BLF_pack(W: usize, a: &Vec<(usize, usize)>) -> Vec<Rect> {

    // BL安定点候補の set. タプル: (y, x) を記録
    let mut bl_lst: BTreeSet<(usize, usize)> = BTreeSet::new();
    bl_lst.insert((0, 0));

    // 配置済の長方形(k)
    let mut k_lst: Vec<Rect> = vec![]; // x, y, x+w, y+h;

    let mut v_lines: Vec<(usize, usize, usize)> = vec![]; // (x, bottom, top);
    let mut h_lines: Vec<(usize, usize, usize)> = vec![]; // (y, left, right);

    v_lines.push((0, 0, 1000_000));
    h_lines.push((0, 0, W));

    for i in 0..a.len() {
        let (w, h) = a[i];

        // BL安定点の中から配置できる最も右下の座標を探す
        let mut pos: (usize, usize) = (!0, !0); // (y, x) の順にとることに注意

        // すでに配置済の長方形:k と重ならず, 長方形を配置できるBL安定点
        for &bp in bl_lst.iter() { // O(bl_pos)
            let i_r = bp.1 + w;
            let i_t = bp.0 + h;
            if i_r > W { continue; }

            // O(N)
            if k_lst.iter().all(|k| {
                // 区間重複
                let x = bp.1 < k.right && k.x < i_r;
                let y = bp.0 < k.top && k.y < i_t;
                // x、y座標いずれか満たさなければOK
                return !x || !y;
            }) {
                if pos.1 < bp.1 { continue; }
                if pos.1 > bp.1 || pos.0 > bp.0 {
                    pos = bp;
                }
                break;
            }
        }
        assert_ne!(pos.0, !0, "{} {}", i, bl_lst.len());
        // 配置する区画
        let mut place = Rect::new(pos.1, pos.0, w, h);

        // 横line
        {
            // // line の縮小
            for v in h_lines.iter_mut().filter(|line| {
                place.y < line.0 && line.0 <= place.top &&
                    place.x < line.2 && line.1 < place.right // x座標の端点は重なるものは残して良い
            }) {
                let (y, l, r) = *v;
                let mut new_v = (y, l, r);
                if place.right >= l {
                    new_v.1 = place.right;
                }
                *v = new_v;
            }
            // 不要になった line の削除
            {
                let mut it = 0usize;
                while it < h_lines.len() {
                    if h_lines[it].1 >= h_lines[it].2 {
                        h_lines.swap_remove(it);
                    } else {
                        it += 1;
                    }
                }
            }
            // 新しいBL安定点探索用のline 追加
            if let Some(line) =
            {
                let mut line = (place.top, 0, place.right);
                for k in k_lst.iter() {
                    let (y, mut l, r) = line;
                    if k.y <= y && y <= k.top {
                        if k.x < r {
                            l.chmax(k.right);
                        }
                    }
                    line = (y, l, r);
                    if l >= r { break; }
                }
                if line.1 >= line.2 {
                    None
                } else {
                    Some(line)
                }
            } {
                // 既存のlineとの交差点を候補に追加
                for &(x, b, t) in v_lines.iter() {
                    if b <= line.0 && line.0 <= t && line.1 <= x && x < line.2 {
                        bl_lst.insert((line.0, x));
                    }
                }
                h_lines.push(line);
            };
        }

        // 縦line
        {
            // line の縮小
            for v in v_lines.iter_mut().filter(|line| {
                place.x < line.0 && line.0 <= place.right &&
                    place.y < line.2 && line.1 < place.top // y軸の端点は重なるものは残して良い
            }) {
                let (x, l, r) = *v;
                let mut new_v = (x, l, r);
                new_v.1.chmax(place.top);
                *v = new_v;
            }
            // 不要になった line の削除
            {
                let mut it = 0usize;
                while it < v_lines.len() {
                    if v_lines[it].1 >= v_lines[it].2 {
                        v_lines.swap_remove(it);
                    } else {
                        it += 1;
                    }
                }
            }
            // 新しいBL安定点探索用のline 追加
            if let Some(line) =
            {
                let mut line = (place.right, 0, place.top);
                for k in k_lst.iter() {
                    let (x, mut b, t) = line;
                    if k.x <= x && x <= k.right {
                        if k.y < t {
                            b.chmax(k.top);
                        }
                    }
                    line = (x, b, t);
                    if b >= t { break; }
                }
                if line.1 >= line.2 {
                    None
                } else {
                    Some(line)
                }
            } {
                // 既存のlineとの交差点を候補に追加
                for &(y, l, r) in h_lines.iter() {
                    if l <= line.0 && line.0 <= r && line.1 <= y && y < line.2 {
                        bl_lst.insert((y, line.0));
                    }
                }
                v_lines.push(line);
            };
        }

        // 追加した四角形と重なるBL安定点の候補は削除
        {
            let mut it = 0usize;
            let mut drops = vec![];
            let presize = bl_lst.len();
            for &v in bl_lst.iter() {
                if place.x <= v.1 && v.1 < place.right &&
                    place.y <= v.0 && v.0 < place.top {
                    drops.push(v);
                }
            }
            let mut dropsize = drops.len();
            assert_ne!(dropsize, 0);
            for v in drops {
                bl_lst.remove(&v);
            }
            assert_eq!(bl_lst.len(), presize - dropsize);
        }
        k_lst.push(place);
    }
    k_lst

}


pub fn BLF_solve(input: &Input) -> Vec<(usize, usize)> {
    let mut ids = (0..input.n).collect_vec();
    let mut a = input.a.clone();
    ids.sort_by_key(|&i| -(a[i].1 as i32));
    a.sort_by_key(|&v| -(v.1 as i32));
    let mut k_lst = BLF_pack(input.w, &a);
    let score = calc_score(&k_lst);
    eprintln!("socre: {}", score);
    k_lst.into_iter().enumerate().sorted_by_key(|(i, _)| ids[*i]).map(|(_, v)| {
        (v.x, v.y)
    }).collect_vec()
}

fn calc_score(lst: &Vec<Rect>) -> usize {
    lst.iter().map(|v| v.top).max().unwrap()
}

pub fn BLF_solve2(input: &Input) -> Vec<(usize, usize)> {
    Timer::get_time();
    const LIM: f64 = 3.0;

    let mut a = input.a.clone();
    a.sort_by_key(|&v| -(v.1 as i32));
    let mut rng = rand_pcg::Pcg64Mcg::new(48);

    let mut best = a.clone();
    let mut best_score: usize = !0;
    let mut best_res = vec![];
    let mut iter = 0;
    while Timer::get_time() < LIM {
        // if iter>1{break;}
        iter += 1;
        let mut k_lst = BLF_pack(input.w, &a);
        let score = calc_score(&k_lst);

        if best_score.chmin(score) {
            best = a.clone();
            best_res = k_lst;
            eprintln!("best score: {}", score);
        }

        loop {
            if a.len() <= 1 { break; }
            a = best.clone();
            let i = rng.gen_range(0, a.len());
            let j = rng.gen_range(0, a.len());
            if i == j { continue; }
            a.swap(i, j);
            break;
        }
    }

    let score = calc_score(&best_res);
    println!("iter:{}, score: {}", iter, score);
    // input の順に並べて返す
    let mut res = vec![];
    let mut used = vec![false; input.a.len()];
    for i in 0..input.a.len() {
        for j in 0..input.a.len() {
            if used[j] { continue; }
            let pos = (best_res[j].w, best_res[j].h);
            if input.a[i] == pos {
                used[j] = true;
                res.push((best_res[j].x, best_res[j].y));
                break;
            }
        }
    }
    return res;
}


fn validate_result(input: &Input, res: &Vec<(usize, usize)>) -> bool {
    let mut ok = true;
    for i in 0..res.len() {
        let rect_i = Rect::new(res[i].0, res[i].1, input.a[i].0, input.a[i].1);
        for j in (i + 1)..res.len() {
            let rect_j = Rect::new(res[j].0, res[j].1, input.a[j].0, input.a[j].1);
            if rect_i.is_intersection(&rect_j) {
                eprintln!("交差しています {}, {}", i, j);
                ok = false;
            }
        }
        if rect_i.right > input.w {
            eprintln!("範囲外に配置されています {}:{:?}", i, rect_i);
        }
    }
    ok
}

pub fn main() {
    let st = Timer::get_time();
    let input = parse_input();
    let res = BLF_solve(&input);
    // let res = BLF_solve2(&input);

    validate_result(&input, &res);

    // for i in 0..res.len() {
    //     println!("{} {}", res[i].0, res[i].1);
    // }
    let end = Timer::get_time();
    println!("end-st: {}", end - st);
}

