#![allow(dead_code)]
use sdl2::mixer::Music;
use std::collections::HashMap;

pub struct SoundSystem<'a> {
    track: HashMap<u8, Music<'a>>,
}

impl<'a> SoundSystem<'a> {
    pub fn new() -> Self {
        SoundSystem { track: HashMap::new() }
    }

    pub fn load(&mut self, id: u8, file: &str) -> bool {
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
        let mut state = true;
        for (id, file) in batch {
            state &= self.load(id, file);
        }
        state
    }

    pub fn play(&self, id: u8) {
        match self.track.get(&id) {
            Some(track) => match track.play(1) {
                Ok(_) => (),
                Err(err) => eprintln!("[warning] cannot play track `{}`: {}", id, err),
            },
            None => (),
        }
    }
}
