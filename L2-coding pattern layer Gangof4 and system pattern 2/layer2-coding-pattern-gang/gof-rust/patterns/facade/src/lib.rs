//! Home-theater Facade.

struct Amplifier;
impl Amplifier {
    fn on(&self) {}
}

struct DvdPlayer;
impl DvdPlayer {
    fn play(&self, _m: &str) {}
}

struct Projector;
impl Projector {
    fn wide_screen(&self) {}
}

pub struct HomeTheater {
    amp: Amplifier,
    dvd: DvdPlayer,
    proj: Projector,
}
impl HomeTheater {
    pub fn new() -> Self {
        Self {
            amp: Amplifier,
            dvd: DvdPlayer,
            proj: Projector,
        }
    }
    pub fn watch_movie(&self, movie: &str) {
        self.amp.on();
        self.proj.wide_screen();
        self.dvd.play(movie);
    }
}
impl Default for HomeTheater {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn facade_ok() {
        HomeTheater::default().watch_movie("Rustaceans 2");
    }
}
