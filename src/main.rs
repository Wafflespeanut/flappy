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

    let initial_timeout_ms: c_uint = 100;           // would be 10 fps
    let mut poll_timeout_ms = initial_timeout_ms;
    let mut time_since_last_ns: u64 = 0;

    let game = Game::new(50, 30);
    let mut jumper = game.jumper;
    let mut cliff = Cliff::new(&jumper);

    loop {
        let start_time = precise_time_ns();
        match poll_keypress(poll_timeout_ms) {      // wait for the given time to capture the input
            Poll::Start => {
                let key = read_keypress();
                match key {         // proceeds immediately on input
                    Key::Quit => {
                        println!("\rGoodbye...\n\r");
                        break
                    },
                    _ => {
                        time_since_last_ns += precise_time_ns() - start_time;
                        poll_timeout_ms = initial_timeout_ms - ((time_since_last_ns / 1000000) as c_uint);
                        jumper.shift(3, Some(key))
                    },
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
        // gameplay inside an outlined box
        println!("{}", game.top);
        print!("\r{}|{}", &game.side, frame.join(&("|\n\r".to_owned() + &game.side + "|")));
        println!("\r{}", game.bottom);

        if cliff.erase_body() {
            cliff = Cliff::new(&jumper);
        }
    }
}
