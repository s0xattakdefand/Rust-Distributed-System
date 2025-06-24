// TODO: src
//! Custom counting iterator 0..n inclusive.

pub struct Counter { cur: u8, max: u8 }
impl Counter { pub fn new(max:u8)->Self{Self{cur:0,max}} }
impl Iterator for Counter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur <= self.max {
            let v = self.cur; self.cur += 1; Some(v)
        } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn sum() {
        assert_eq!(Counter::new(4).sum::<u8>(), 10);
    }
}
