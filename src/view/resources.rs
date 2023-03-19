use std::fs::File;

use ggez::{
    audio::{self, SoundSource},
    Context, graphics::Image,
};
use ron::de::from_reader;
use serde::Deserialize;
use structopt::lazy_static::lazy_static;

use geometry::Size;

use crate::models::{AtlasData, Images, Jukebox};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub music_volume: f32,
    pub sound_volume: f32,
}

pub struct Resources {
    pub images: Images,
    pub atlas_data: AtlasData,
    pub jukebox: Jukebox,
}

pub static mut RESOURCES: Option<Resources> = None;

impl Resources {
    /// Initialize and define the `Resources` instance.
    pub fn init(ctx: &mut Context) {
        let config: Config =
            from_reader(File::open("resources/config.ron").unwrap()).unwrap();

        let jukebox = Jukebox {
            music: new_audio(ctx, "/audio/music.mp3", config.music_volume, true),
            lose: new_audio(ctx, "/audio/lose.wav", config.sound_volume, false),
            win: new_audio(ctx, "/audio/win.wav", config.sound_volume, false),
            hit: new_audio(ctx, "/audio/hit.wav", config.sound_volume, false),
            defend: new_audio(ctx, "/audio/defend.wav", config.sound_volume, false),
        };

        let images = Images {
            hero: Image::new(ctx, "/images/hero.png").unwrap(),
            monster: Image::new(ctx, "/images/monster.png").unwrap(),
            claw: Image::new(ctx, "/images/claw.png").unwrap(),
            slash: Image::new(ctx, "/images/slash.png").unwrap(),
        };

        let atlas_data = AtlasData::parse_file(ctx, "resources/atlas.ron");

        println!("Resources loaded.");
        unsafe {
            RESOURCES = Some(Resources {
                images,
                atlas_data,
                jukebox,
            });
        }
    }

    /// Get the `Resources` instance.
    /// If the instance is not initialized, the method will panic.
    pub fn instance() -> &'static mut Resources {
        unsafe {
            match RESOURCES {
                Some(ref mut resources) => resources,
                None => panic!("Resources not initialized."),
            }
        }
    }
}

fn new_audio(ctx: &mut Context, path: &str, volume: f32, repeat: bool) -> audio::Source {
    let mut sound = audio::Source::new(ctx, path).unwrap();
    sound.set_volume(volume);
    sound.set_repeat(repeat);
    sound
}
