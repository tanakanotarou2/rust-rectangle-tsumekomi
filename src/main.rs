use rust_tsumekomi::bl_solve;
use rust_tsumekomi::bl_solve::Input;

fn main() {
    let mut input = Input { n: 3, w: 3, a: vec![(1, 2)] };
    let res = bl_solve::solve(&input);
    println!("{:?}",res);
}
