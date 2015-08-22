extern crate libc;

mod sprites;
mod helpers;

use helpers::{FallArea, fill_up};
use sprites::{Sprite, Jumper};

fn main() {
    let fall_area = FallArea::new(100, 50);
    let mut frame: Vec<String> = Vec::new();
    let jumper = Jumper { area: fall_area };
    let top_indent = fall_area.height.1 / 2;
    let bottom_indent = fall_area.height.1 - top;

    loop {
        frame = jumper.draw(&frame, 44);

        for line in &frame {
            println!("\t| {} |", line);
        }
    }
}
