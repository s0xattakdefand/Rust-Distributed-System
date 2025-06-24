//! Safe virtual-proxy example.

use std::cell::RefCell;

pub trait Image {
    fn display(&self) -> &'static str;
}

pub struct RealImage {
    filename: &'static str,
}
impl RealImage {
    fn load(filename: &'static str) -> Self {
        println!("Loading {filename} …");
        Self { filename }
    }
}
impl Image for RealImage {
    fn display(&self) -> &'static str {
        // read the field so Clippy knows it’s used
        let _ = self.filename;
        "high-res image displayed"
    }
}


pub struct ImageProxy {
    filename: &'static str,
    real: RefCell<Option<RealImage>>,
}
impl ImageProxy {
    pub fn new(filename: &'static str) -> Self {
        Self {
            filename,
            real: RefCell::new(None),
        }
    }
}
impl Image for ImageProxy {
    fn display(&self) -> &'static str {
        if self.real.borrow().is_none() {
            *self.real.borrow_mut() = Some(RealImage::load(self.filename));
        }
        self.real.borrow().as_ref().unwrap().display()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn proxy_lazy() {
        let img = ImageProxy::new("monster.png");
        assert_eq!(img.display(), "high-res image displayed");
        assert_eq!(img.display(), "high-res image displayed");
    }
}
