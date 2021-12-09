#![allow(dead_code)]
use sdl2::mixer::{Channel, Chunk, Music};
use std::collections::HashMap;

pub enum MusicLoop {
    Once,
    Repeat,
    Count(i32),
}

pub struct SoundSystem<'a> {
    music: HashMap<u8, Music<'a>>,
    sound: HashMap<u8, Chunk>,
    volume_sound: i32,
    volume_music: i32,
    enable_sound: bool,
    enable_music: bool,
}

// TODO: redesign audio system
impl<'a> SoundSystem<'a> {
    pub fn new() -> Self {
        SoundSystem {
            music: HashMap::new(),
            sound: HashMap::new(),
            enable_sound: true,
            enable_music: true,
            volume_music: 0,
            volume_sound: 0,
        }
    }

    pub fn set_sound_status(&mut self, status: bool) {
        self.enable_sound = status;
    }

    pub fn set_music_status(&mut self, status: bool) {
        self.enable_music = status;
    }

    pub fn set_sound_volume(&mut self, volume: u8) {
        self.volume_sound = volume.min(128) as i32;
    }

    pub fn set_music_volume(&mut self, volume: u8) {
        self.volume_music = volume.min(128) as i32;
    }

    pub fn load_sound(&mut self, id: u8, file: &str) -> bool {
        if !self.enable_sound {
            return true;
        }
        match Chunk::from_file(file) {
            Ok(track) => {
                self.sound.insert(id, track);
                true
            }
            Err(err) => {
                eprintln!("[sound::warning] problem with load `{}`: {}", file, err);
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

    pub fn batch_load_sound<I>(&mut self, batch: I) -> bool
    where
        I: IntoIterator<Item = (u8, &'static str)>,
    {
        if !self.enable_sound {
            return true;
        }
        let mut state = true;
        for (id, file) in batch {
            state &= self.load_sound(id, file);
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

    pub fn play_sound(&self, id: u8) {
        if !self.enable_sound {
            return;
        }
        if let Some(track) = self.sound.get(&id) {
            let channel = Channel::all();
            channel.set_volume(self.volume_sound);
            match channel.play(track, 0) {
                Ok(_) => (),
                Err(err) => eprintln!("[sound::warning] cannot play audio `{}`: {}", id, err),
            }
        }
    }

    pub fn play_music(&self, id: u8, repeat: MusicLoop) {
        if !self.enable_music {
            return;
        }
        let repeat_count = match repeat {
            MusicLoop::Once => 0,
            MusicLoop::Repeat => -1,
            MusicLoop::Count(n) => n,
        };
        if let Some(track) = self.music.get(&id) {
            Music::set_volume(self.volume_music);
            match track.play(repeat_count) {
                Ok(_) => (),
                Err(err) => eprintln!("[music::warning] cannot play audio `{}`: {}", id, err),
            }
        }
    }

    pub fn stop_music(&self) {
        Music::halt();
    }
}
