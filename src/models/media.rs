use ggez::audio::Source;
use ggez::graphics::Image;

/// A struct to hold all the audio used in the game
pub struct Jukebox {
    pub music: Source,
    pub lose: Source,
    pub win: Source,
    pub hit: Source,
    pub defend: Source,
}

/// A struct to hold all the images used in the game
pub struct Images {
    pub hero: Image,
    pub monster: Image,
    pub claw: Image,
    pub slash: Image,
}