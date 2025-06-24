// TODO: src
//! Pluggable compression algorithms.

pub trait Compressor { fn compress(&self, data:&[u8]) -> Vec<u8>; }

pub struct Zip;  impl Compressor for Zip  { fn compress(&self,d: &[u8])->Vec<u8>{d.to_vec()} }
pub struct Rar;  impl Compressor for Rar  { fn compress(&self,d: &[u8])->Vec<u8>{d.iter().rev().cloned().collect()} }

pub struct Context<C:Compressor> { alg: C }
impl<C:Compressor> Context<C> {
    pub fn new(alg:C)->Self{Self{alg}}
    pub fn run(&self, data:&[u8])->Vec<u8>{ self.alg.compress(data) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn choose() {
        let d=b"abc";
        assert_eq!(Context::new(Zip).run(d), b"abc");
        assert_eq!(Context::new(Rar).run(d), b"cba");
    }
}
