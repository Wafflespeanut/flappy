extern crate libc;
extern crate rand;

mod helpers;
mod keyevents;
mod sprites;

use keyevents::*;
use libc::c_uint;
use sprites::*;

fn main() {
    let _raw = set_raw_mode();  // has the old termios attributes (which will be restored on drop)
    let poll_timeout_ms: c_uint = 1000;    // wait for the given time to capture the input
    let mut frame = Vec::new();    // initial vector to be used as the base frame

    let game = Game::new(65, 35);
    let mut jumper = game.jumper;
    let (left_indent, top_indent, bottom_indent) = (game.side, game.top, game.bottom);

    loop {
        frame = jumper.draw(&frame);
        let cliff = Cliff::new(&jumper);
        frame = cliff.draw(&frame);

        match poll_keypress(poll_timeout_ms) {      // waits a second for input
            Poll::Start => {
                match read_keypress() {         // proceeds immediately on input
                    Key::Quit => {
                        println!("\rGoodbye...\n\r");
                        break
                    },
                    _ => {  }, // do nothing for now
                }
            },
            Poll::Wait => {
                // won't be of use (for now)
            },
        }

        for line in &top_indent {
            println!("\r{}{}", left_indent, line);
        }
        for line in &frame {
            println!("\r{}| {} |", left_indent, line);       // contents of the box (gameplay)
        }
        for line in &bottom_indent {
            println!("\r{}{}", left_indent, line);
        }
    }
}
