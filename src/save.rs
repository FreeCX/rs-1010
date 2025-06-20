use std::time::{Duration, SystemTime};

use sdl2::pixels::Color;

use crate::codec::{Decoder, Encoder};
use crate::consts::*;
use crate::game::{BasketSystem, Field, Figure};

pub fn serialize(field: Field, bsystem: BasketSystem, score: u32, time: SystemTime) -> String {
    let duration = time.elapsed().unwrap_or_else(|_| Duration::from_secs(0)).as_secs();
    let mut encoder = Encoder::new();

    // Game field state
    for y in 0..field.field_size.y {
        for x in 0..field.field_size.x {
            encoder.push(field.is_set(&coord!(x, y)), SERDE_FIELD_SIZE);
        }
    }

    // Figures in basket
    for basket in bsystem.destroy() {
        let value = match basket.figure() {
            Some(figure) => figure.index,
            None => 0,
        };
        encoder.push(value, SERDE_FIGURE_SIZE);
    }

    // Current score
    encoder.push(score, SERDE_SCORE_SIZE);
    // Current game time
    encoder.push(duration as i64, SERDE_TIME_SIZE);
    // Padding
    encoder.push(0, SERDE_PADDING_SIZE);

    encoder.result()
}

pub fn deserialize(
    data: String, default: &Color, figures: &[Figure], field: &mut Field, basket: &mut BasketSystem, score: &mut u32,
    time: &mut SystemTime,
) -> Option<()> {
    // we only support fixed game state size
    if data.len() != SERDE_TOTAL_SIZE {
        return None;
    }

    let mut decoder = Decoder::decode(&data)?;

    // restore field
    for y in 0..field.field_size.y {
        for x in 0..field.field_size.x {
            if decoder.take::<u8>(1)? == 1 {
                field.set(coord!(x, y), *default);
            }
        }
    }

    // restore figures
    for index in 0..BASKET_COUNT as usize {
        let fig_num = decoder.take::<usize>(SERDE_FIGURE_SIZE)?;
        if index > 0 {
            basket.set(index, figures[fig_num - 1].clone());
        } else {
            basket.pop(index);
        }
    }

    // restore game score
    *score = decoder.take(SERDE_SCORE_SIZE)?;

    // restore elapsed time
    let raw_time = decoder.take(SERDE_TIME_SIZE)?;
    let duration = Duration::from_secs(raw_time);
    let current = SystemTime::now();
    *time = current.checked_sub(duration).unwrap_or(current);

    Some(())
}
