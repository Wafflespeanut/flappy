extern crate libc;
extern crate rand;

mod sprites;
mod helpers;

use helpers::{FallArea, multiply};
use sprites::{Sprite, Jumper};
use std::iter::repeat;
use std::thread::sleep_ms;

fn main() {
    let fall_area = FallArea::new(75, 40);
    let mut frame: Vec<String> = Vec::new();
    let jumper = Jumper::new(fall_area);

    let top_indent = fall_area.height.1 / 2;
    let bottom_indent = fall_area.height.1 - top_indent;
    let left_shift = multiply(" ", fall_area.width.1 / 2);
    let box_width = fall_area.width.0;

    let mut user_env_top = (0..top_indent - 1).map(|_| {
        multiply(" ", box_width)
    }).collect::<Vec<String>>();
    let dashes = String::from("-") + &multiply("-", box_width) + "---";
    user_env_top.push(dashes.clone());
    let mut user_env_bottom: Vec<String> = vec![dashes];
    for i in 0..bottom_indent - 1 {
        user_env_bottom.push(multiply(" ", box_width));
    }

    loop {
        sleep_ms(50);                   // 20 fps
        frame = jumper.draw(&frame);
        for line in &user_env_top {
            println!("{}{}", left_shift, line);
        }
        for line in &frame {
            println!("{}| {} |", left_shift, line);
        }
        for line in &user_env_bottom {
            println!("{}{}", left_shift, line);
        }
    }
}
