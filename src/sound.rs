#![allow(dead_code)]
use sdl2::mixer::Music;
use std::collections::HashMap;

pub struct SoundSystem<'a> {
    track: HashMap<u8, Music<'a>>,
    enabled: bool,
}

impl<'a> SoundSystem<'a> {
    pub fn new() -> Self {
        SoundSystem { track: HashMap::new(), enabled: true }
    }

    pub fn set_status(&mut self, status: bool) {
        self.enabled = status;
    }

    pub fn set_volume(&self, volume: u8) {
        Music::set_volume(volume.min(128) as i32);
    }

    pub fn load(&mut self, id: u8, file: &str) -> bool {
        if !self.enabled {
            return true;
        }
        match Music::from_file(file) {
            Ok(track) => {
                self.track.insert(id, track);
                true
            }
            Err(err) => {
                eprintln!("[warning] problem with load `{}`: {}", file, err);
                false
            }
        }
    }

    pub fn batch_load<I>(&mut self, batch: I) -> bool
    where
        I: IntoIterator<Item = (u8, &'static str)>,
    {
        if !self.enabled {
            return true;
        }
        let mut state = true;
        for (id, file) in batch {
            state &= self.load(id, file);
        }
        state
    }

    pub fn play(&self, id: u8) {
        if !self.enabled {
            return;
        }
        match self.track.get(&id) {
            Some(track) => match track.play(1) {
                Ok(_) => (),
                Err(err) => eprintln!("[warning] cannot play track `{}`: {}", id, err),
            },
            None => (),
        }
    }
}
