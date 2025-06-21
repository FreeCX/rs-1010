use std::time::{Duration, SystemTime};

use sdl2::pixels::Color;

use crate::codec::{Decoder, Encoder};
use crate::consts::*;
use crate::game::{BasketSystem, Field, Figure};

pub fn serialize(palette: &[Color], field: Field, bsystem: BasketSystem, score: u32, time: SystemTime) -> String {
    let duration = time.elapsed().unwrap_or_else(|_| Duration::from_secs(0)).as_secs();
    let mut encoder = Encoder::new();

    let mut color_data = Vec::new();

    // Game field state
    for y in 0..field.field_size.y {
        for x in 0..field.field_size.x {
            let pos = coord!(x, y);
            let is_set = field.is_set(&pos);
            encoder.push(is_set, SERDE_FIELD_SIZE);
            if is_set {
                if let Some(color) = field.get_color(&pos) {
                    let color_index = palette.iter().position(|i| i == color).unwrap_or(0) as u8;
                    color_data.push((pos, color_index));
                }
            }
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
    encoder.push(SERDE_V2_SUPPORT, SERDE_PADDING_SIZE);

    // Field color data
    encoder.push(color_data.len() as u8, SERDE_COLOR);
    for (pos, color) in color_data {
        encoder.push(pos.x, SERDE_POS);
        encoder.push(pos.y, SERDE_POS);
        encoder.push(color, SERDE_COLOR);
    }

    encoder.result()
}

pub fn deserialize(
    data: String, palette: &[Color], figures: &[Figure], field: &mut Field, basket: &mut BasketSystem, score: &mut u32,
    time: &mut SystemTime,
) -> Option<()> {
    let mut decoder = Decoder::decode(&data)?;

    // restore field
    for y in 0..field.field_size.y {
        for x in 0..field.field_size.x {
            if decoder.take::<u8>(1)? == 1 {
                field.set(coord!(x, y), palette[0]);
            }
        }
    }

    // restore figures
    for index in 0..BASKET_COUNT as usize {
        let fig_num = decoder.take::<usize>(SERDE_FIGURE_SIZE)?;
        if fig_num > 0 {
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

    let padding = decoder.take::<u8>(SERDE_PADDING_SIZE)?;
    // load extra info about field colors
    if padding == SERDE_V2_SUPPORT {
        for _ in 0..decoder.take::<u8>(SERDE_COLOR)? {
            let x = decoder.take::<i16>(SERDE_POS)?;
            let y = decoder.take::<i16>(SERDE_POS)?;
            let color = decoder.take::<usize>(SERDE_COLOR)?;
            field.set(coord!(x, y), palette[color]);
        }
    }

    Some(())
}
