/// chmin, chmax 関数
pub trait SetMinMax {
    fn chmin(&mut self, v: Self) -> bool;
    fn chmax(&mut self, v: Self) -> bool;
}

impl<T> SetMinMax for T where T: PartialOrd {
    fn chmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn chmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

