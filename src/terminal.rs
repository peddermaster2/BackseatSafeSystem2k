// featuring Tom Hanks

use crate::{address_constants, memory::Memory, Address, Size, Word};
use raylib::prelude::*;

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 25;

pub fn render(
    memory: &Memory,
    draw_handle: &mut RaylibDrawHandle,
    position: Vector2,
    font: &Font,
    font_height: f32,
) {
    for row in 0..HEIGHT {
        // let words = &memory[row * WIDTH..][..WIDTH];
        let string: String = (0..WIDTH)
            .map(|i| {
                memory.read_data(
                    address_constants::TERMINAL_BUFFER_START
                        + (row * WIDTH * Word::SIZE + i * Word::SIZE) as Address,
                )
            })
            .map(|word| {
                if !(32..=255).contains(&word) {
                    b' '
                } else {
                    word as u8
                }
            })
            .map(|c| c as char)
            .collect();
        let text = string.as_str();

        draw_handle.draw_text_ex(
            font,
            text,
            Vector2::new(position.x, position.y + row as f32 * font_height as f32),
            font_height,
            5.0,
            Color::WHITE,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{Size, Word};

    use super::*;

    #[test]
    fn terminal_character_width_divisible_by_word_size() {
        assert_eq!(WIDTH % Word::SIZE, 0);
    }
}
