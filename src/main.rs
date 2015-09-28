extern crate libc;
extern crate rand;
extern crate time;

mod helpers;
mod keyevents;
mod sprites;

use keyevents::*;
use libc::c_uint;
use sprites::*;
use time::precise_time_ns;

fn main() {
    let _raw = set_raw_mode();  // has the old termios attributes (which will be restored on drop)
    let mut frame: Vec<String>;

    let initial_timeout_ms: c_uint = 100;  // wait for the given time to capture the input
    let mut poll_timeout_ms: c_uint = initial_timeout_ms;
    let mut time_since_last_ns: u64 = 0;

    let game = Game::new(50, 30);
    let mut jumper = game.jumper;
    let mut cliff = Cliff::new(&jumper);
    let (left_indent, top_indent, bottom_indent) = (&game.side, &game.top, &game.bottom);

    loop {
        let start_time = precise_time_ns();

        match poll_keypress(poll_timeout_ms) {
            Poll::Start => {
                let key = read_keypress();
                cliff.shift(1, None);
                match key {         // proceeds immediately on input
                    Key::Quit => {
                        println!("\rGoodbye...\n\r");
                        break
                    },
                    Key::Right | Key::Left => {
                        jumper.shift(3, Some(key));
                        time_since_last_ns += precise_time_ns() - start_time;
                        poll_timeout_ms = (time_since_last_ns / 1000000) as c_uint;
                    },
                    _ => (),
                }
            },
            Poll::Wait => {
                time_since_last_ns = 0;
                poll_timeout_ms = initial_timeout_ms;
                cliff.shift(1, None);
            },
        }

        frame = jumper.draw(None);
        frame = cliff.draw(Some(&frame));

        for line in top_indent {
            println!("\r{}{}", left_indent, line);
        }
        for line in &frame {
            println!("\r{}|{}|", left_indent, line);       // contents of the box (gameplay)
        }
        for line in bottom_indent {
            println!("\r{}{}", left_indent, line);
        }
    }
}
