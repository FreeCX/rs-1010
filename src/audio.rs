use std::collections::HashMap;

use sdl2::mixer::{Channel, Chunk, Music};

pub enum MusicLoop {
    Once,
    Repeat,
    #[allow(dead_code)]
    Count(i32),
}

pub struct AudioSystem<'a> {
    music: HashMap<u8, Music<'a>>,
    audio: HashMap<u8, Chunk>,
    volume_sfx: i32,
    volume_music: i32,
    enable_sfx: bool,
    enable_music: bool,
}

impl<'a> AudioSystem<'a> {
    pub fn new() -> Self {
        AudioSystem {
            music: HashMap::new(),
            audio: HashMap::new(),
            enable_sfx: true,
            enable_music: true,
            volume_music: 0,
            volume_sfx: 0,
        }
    }

    pub fn set_sfx_status(&mut self, status: bool) {
        self.enable_sfx = status;
    }

    pub fn set_music_status(&mut self, status: bool) {
        self.enable_music = status;
    }

    pub fn set_sfx_volume(&mut self, volume: u8) {
        self.volume_sfx = volume.min(128) as i32;
    }

    pub fn set_music_volume(&mut self, volume: u8) {
        self.volume_music = volume.min(128) as i32;
    }

    pub fn load_sfx(&mut self, id: u8, file: &str) -> bool {
        if !self.enable_sfx {
            return true;
        }
        match Chunk::from_file(file) {
            Ok(track) => {
                self.audio.insert(id, track);
                true
            }
            Err(err) => {
                eprintln!("[audio::warning] problem with load `{}`: {}", file, err);
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

    pub fn batch_load_sfx<I>(&mut self, batch: I) -> bool
    where
        I: IntoIterator<Item = (u8, &'static str)>,
    {
        if !self.enable_sfx {
            return true;
        }
        let mut state = true;
        for (id, file) in batch {
            state &= self.load_sfx(id, file);
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

    pub fn play_sfx(&self, id: u8) {
        if !self.enable_sfx {
            return;
        }
        if let Some(track) = self.audio.get(&id) {
            let channel = Channel::all();
            channel.set_volume(self.volume_sfx);
            match channel.play(track, 0) {
                Ok(_) => (),
                Err(err) => eprintln!("[audio::warning] cannot play audio `{}`: {}", id, err),
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
