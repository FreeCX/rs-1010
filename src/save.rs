use crate::game::{BasketSystem, Field, Figure};
use sdl2::pixels::Color;
use std::time::{Duration, SystemTime};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+~";
const BLOCK_SIZE: usize = 6;

fn as_int(data: &str) -> usize {
    let mut result = 0;
    for item in data.chars() {
        result <<= 1;
        if item == '1' {
            result += 1;
        }
    }
    result
}

fn encode(data: String) -> String {
    // игнорим невалидные данные
    assert_eq!(data.len() % BLOCK_SIZE, 0, "data size is not divided by {}", BLOCK_SIZE);

    let mut result = String::new();
    for index in (0..data.len()).step_by(BLOCK_SIZE) {
        let value = as_int(&data[index..index + BLOCK_SIZE]);
        let item = ALPHABET.chars().nth(value).unwrap();
        result.push(item);
    }

    result
}

fn decode(data: String) -> String {
    let mut result = String::new();
    for item in data.chars() {
        let value = ALPHABET.find(item).unwrap();
        result.push_str(&format!("{:06b}", value));
    }
    result
}

pub fn serialize(field: Field, bsystem: BasketSystem, score: u32, time: SystemTime) -> String {
    let duration = time.elapsed().unwrap_or(Duration::from_secs(0)).as_secs();

    // - Состояние игрового поля
    //   - 100 бит информации (10x10)
    let mut field_v = String::new();
    for y in 0..field.field_size.y {
        for x in 0..field.field_size.x {
            match field.is_set(&coord!(x, y)) {
                true => field_v.push('1'),
                false => field_v.push('0'),
            }
        }
    }

    // - Фигуры в корзинах
    //   - 15 бит (5 бит на фигуру)
    let mut basket_v = String::new();
    for basket in bsystem.destroy() {
        if let Some(figure) = basket.figure() {
            basket_v.push_str(&format!("{:05b}", figure.index));
        } else {
            basket_v.push_str("00000");
        }
    }

    // - Текущий счёт
    //   - 32 бита (u32)
    // - Текущее время
    //   - 64 бита (u64)
    // - Паддинг
    //   - 5 бит
    encode(format!("{}{}{:032b}{:064b}00000", field_v, basket_v, score, duration))
}

pub fn deserialize(
    data: String, default: &Color, figures: &[Figure], field: &mut Field, basket: &mut BasketSystem, score: &mut u32,
    time: &mut SystemTime,
) {
    // будем пока поддерживать только сохранение по дефолтным размерам все игровые элементы
    if data.len() != 36 {
        return;
    }

    // распаковываем наши данные
    let data = decode(data);

    // размер игрового поля
    let field_size = field.field_size.x as usize * field.field_size.y as usize;
    // 5 бит одна фигура
    let figure_size = 5;
    let basket_size = figure_size * basket.figures().len();
    let basket_range_right = field_size + basket_size;
    // 32 бита на весь score
    let score_range_right = basket_range_right + 32;

    // диапазоны для вытаскивания результатов
    let field_range = 0..field_size;
    let basket_range = field_size..basket_range_right;
    let score_range = basket_range_right..score_range_right;
    // 64 бита на весь duration
    let time_range = score_range_right..score_range_right + 64;

    // восстанавливаем игровое поле
    for (index, item) in (&data[field_range]).chars().enumerate() {
        let x = index as i16 % field.field_size.y;
        let y = index as i16 / field.field_size.y;
        if item == '1' {
            field.set(coord!(x, y), *default);
        }
    }

    // восстанавливаем фигуры в корзинах
    for (index, v) in basket_range.step_by(figure_size).enumerate() {
        let findex = as_int(&data[v..v + figure_size]);
        if findex > 0 {
            basket.set(index, figures[findex - 1].clone());
        } else {
            basket.pop(index);
        }
    }

    // восстанавливаем игровой счёт
    *score = as_int(&data[score_range]) as u32;

    // восстанавливаем игровое время
    let duration = Duration::from_secs(as_int(&data[time_range]) as u64);
    let current = SystemTime::now();
    *time = current.checked_sub(duration).unwrap_or(current);
}
