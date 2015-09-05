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
    let poll_timeout_ms: c_uint = 50;    // wait for the given time to capture the input
    let mut frame: Vec<String>;

    let game = Game::new(65, 35);
    let mut jumper = game.jumper;
    let (left_indent, top_indent, bottom_indent) = (&game.side, &game.top, &game.bottom);

    loop {
        frame = jumper.draw(None);
        let cliff = Cliff::new(&jumper);
        frame = cliff.draw(Some(&frame));

        match poll_keypress(poll_timeout_ms) {      // waits a second for input
            Poll::Start => {
                let key = read_keypress();
                match key {         // proceeds immediately on input
                    Key::Quit => {
                        println!("\rGoodbye...\n\r");
                        break
                    },
                    Key::Right | Key::Left => jumper.shift(3, Some(key)),
                    _ => {  }, // do nothing for now
                }
            },
            Poll::Wait => {
                // won't be of use (for now)
            },
        }

        for line in top_indent {
            println!("\r{}{}", left_indent, line);
        }
        for line in frame {
            println!("\r{}| {} |", left_indent, line);       // contents of the box (gameplay)
        }
        for line in bottom_indent {
            println!("\r{}{}", left_indent, line);
        }
    }
}
