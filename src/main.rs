extern crate libc;
extern crate rand;

mod helpers;
mod keyevents;
mod sprites;

use helpers::{FallArea, multiply};
use keyevents::set_raw_mode;
use sprites::*;
use std::thread::sleep_ms;

fn main() {
    let _raw = set_raw_mode();      // has the old termios attributes (which is restored on drop)
    let fall_area = FallArea::new(65, 35);
    let mut frame: Vec<String> = Vec::new();    // initial garbage vector to be used as the base frame
    let jumper = Jumper::new(fall_area);

    let top_indent = fall_area.height.1 / 2;
    let bottom_indent = fall_area.height.1 - top_indent;
    let left_shift = multiply(" ", fall_area.width.1 / 2);
    let box_width = fall_area.width.0;

    // draw the dashed box for the frames to be drawn inside
    let mut user_env_top = (0..top_indent - 1).map(|_| {        // say, box's lid
        multiply(" ", box_width)
    }).collect::<Vec<String>>();
    let dashes = String::from("-") + &multiply("-", box_width) + "---";
    user_env_top.push(dashes.clone());
    let mut user_env_bottom: Vec<String> = vec![dashes];        // say, box's base
    for _ in 0..bottom_indent - 1 {
        user_env_bottom.push(multiply(" ", box_width));
    }

    loop {
        sleep_ms(500);                   // testing at 2 fps
        frame = jumper.draw(&frame);
        let cliff = Cliff::new(jumper);
        frame = cliff.draw(&frame);
        for line in &user_env_top {
            println!("\r{}{}", left_shift, line);
        }
        for line in &frame {
            println!("\r{}| {} |", left_shift, line);       // contents of the box (gameplay)
        }
        for line in &user_env_bottom {
            println!("\r{}{}", left_shift, line);
        }
    }
}
