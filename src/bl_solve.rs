use std::time::Instant;
use std::ops::{Index, IndexMut};
use itertools::{concat, Itertools};
// use rand_pcg::Mcg128Xsl64;

use proconio::{*, marker::*};
use crate::procon_utils::SetMinMax;
/* timer
------------------------ */
struct Timer {
    since: Instant,
    duration: f64,
}

impl Timer {
    fn new(duration: f64) -> Timer {
        Timer {
            since: Instant::now(),
            duration,
        }
    }
    fn t(&self) -> f64 {
        (Instant::now() - self.since).as_secs_f64() * (1.0 / self.duration)
    }

    /*
     * 経過時間取得(sec)
     * 実行経過時間測定用
     * 実行直後に1度コールする。2回目以降は1度目のコールからの経過時間を返す
     *
     */
    fn get_time() -> f64 {
        static mut STIME: f64 = -1.0;
        let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
        let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
        unsafe {
            if STIME < 0.0 {
                STIME = ms;
            }
            ms - STIME
        }
    }
}

#[derive(Clone, Debug)]
struct Input {
    n: usize,
    // item count
    w: usize,
    // width
    a: Vec<(usize, usize)>,
}

fn parse_input() -> Input {
    input! {
        n: usize,
        w: usize,
		a: [(usize,usize); n]
	}
    Input { n, w, a }
}

pub fn main() {
    // Logger::init();
    // Timer::get_time();
    // let input=parse_input();
    // println!("{:?}",input);
    // let input = parse_input();
    // println!("{}", out.iter().join(""));
    let mut x:i32=50;
    x.chmax(100);
    println!("{}", x);
}

