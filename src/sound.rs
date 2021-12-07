#![allow(dead_code)]
use sdl2::mixer::{Channel, Chunk, Music};
use std::collections::HashMap;

pub struct SoundSystem<'a> {
    music: HashMap<u8, Music<'a>>,
    effect: HashMap<u8, Chunk>,
    volume_effect: i32,
    volume_music: i32,
    enable_effect: bool,
    enable_music: bool,
}

// TODO: redesign audio system
impl<'a> SoundSystem<'a> {
    pub fn new() -> Self {
        SoundSystem {
            music: HashMap::new(),
            effect: HashMap::new(),
            enable_effect: true,
            enable_music: true,
            volume_music: 0,
            volume_effect: 0,
        }
    }

    pub fn set_effect_status(&mut self, status: bool) {
        self.enable_effect = status;
    }

    pub fn set_music_status(&mut self, status: bool) {
        self.enable_music = status;
    }

    pub fn set_effect_volume(&mut self, volume: u8) {
        self.volume_effect = volume.min(128) as i32;
    }

    pub fn set_music_volume(&mut self, volume: u8) {
        self.volume_music = volume.min(128) as i32;
    }

    pub fn load_effect(&mut self, id: u8, file: &str) -> bool {
        if !self.enable_effect {
            return true;
        }
        match Chunk::from_file(file) {
            Ok(track) => {
                self.effect.insert(id, track);
                true
            }
            Err(err) => {
                eprintln!("[effect::warning] problem with load `{}`: {}", file, err);
                false
            }
        }
    }

    pub fn load_music(&mut self, id: u8, file: &str) -> bool {
        if !self.enable_music {
            return true;
        }
        match Music::from_file(file) {
            Ok(track) => {
                self.music.insert(id, track);
                true
            }
            Err(err) => {
                eprintln!("[music::warning] problem with load `{}`: {}", file, err);
                false
            }
        }
    }

    pub fn batch_load_effect<I>(&mut self, batch: I) -> bool
    where
        I: IntoIterator<Item = (u8, &'static str)>,
    {
        if !self.enable_effect {
            return true;
        }
        let mut state = true;
        for (id, file) in batch {
            state &= self.load_effect(id, file);
        }
        state
    }

    pub fn batch_load_music<I>(&mut self, batch: I) -> bool
    where
        I: IntoIterator<Item = (u8, &'static str)>,
    {
        if !self.enable_music {
            return true;
        }
        let mut state = true;
        for (id, file) in batch {
            state &= self.load_music(id, file);
        }
        state
    }

    pub fn play_effect(&self, id: u8) {
        if !self.enable_effect {
            return;
        }
        if let Some(track) = self.effect.get(&id) {
            let channel = Channel::all();
            channel.set_volume(self.volume_effect);
            match channel.play(track, 0) {
                Ok(_) => (),
                Err(err) => eprintln!("[effect::warning] cannot play audio `{}`: {}", id, err),
            }
        }
    }

    pub fn play_music(&self, id: u8) {
        if !self.enable_music {
            return;
        }
        if let Some(track) = self.music.get(&id) {
            Music::set_volume(self.volume_music);
            match track.play(-1) {
                Ok(_) => (),
                Err(err) => eprintln!("[music::warning] cannot play audio `{}`: {}", id, err),
            }
        }
    }
}
