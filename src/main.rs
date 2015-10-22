extern crate libc;
extern crate rand;
extern crate time;

mod helpers;
mod keyevents;
mod sprites;

use helpers::print_error;
use keyevents::*;
use libc::c_uint;
use sprites::*;
use time::precise_time_ns;

fn main() {
    let _raw = match set_raw_mode() {   // old termios attributes (which will be restored on drop)
        Ok(term_attrib) => term_attrib,
        Err(err) => {
            print_error(err);
            return;
        }
    };

    let initial_timeout_ms: c_uint = 100;           // would be 10 fps
    let mut poll_timeout_ms = initial_timeout_ms;
    let mut time_since_last_ns: u64 = 0;

    let game = match Game::new(50, 30) {
        Ok(stuff) => stuff,
        Err(err) => {
            print_error(err);
            return;
        }
    };

    let mut jumper = game.jumper;
    let mut cliff = Cliff::new(&jumper);
    let mut game_frame: Vec<String>;
    let mut quit_msg: Option<&str> = None;

    loop {
        let start_time = precise_time_ns();
        if cliff.erase_body() {
            cliff = Cliff::new(&jumper);
        }

        match poll_keypress(poll_timeout_ms) {      // wait for the given time to capture the input
            Ok(poll) => match poll {
                Poll::Start => {
                    let keypress = read_keypress();
                    match keypress {        // proceeds immediately on input
                        Ok(key) => match key {
                            Key::Quit => {
                                println!("\rGoodbye...\n\r");
                                break
                            },
                            _ => {
                                time_since_last_ns += precise_time_ns() - start_time;
                                poll_timeout_ms = initial_timeout_ms - ((time_since_last_ns / 1000000) as c_uint);
                                jumper.shift(3, Some(key))
                            },
                        },
                        Err(err) => {
                            print_error(err);
                            break
                        }
                    }
                },
                Poll::Wait => {
                    time_since_last_ns = 0;
                    poll_timeout_ms = initial_timeout_ms;
                    cliff.shift(1, None);
                },
            },
            Err(err) => {
                print_error(err);
                break
            }
        }

        // gameplay inside an outlined box
        println!("{}", game.top);
        game_frame = jumper.draw(None).unwrap();    // this won't panic for the current design
        game_frame = cliff.draw(Some(&game_frame)).unwrap();
        print!("\r{}|{}", &game.side, game_frame.join(&("|\n\r".to_owned() + &game.side + "|")));
        println!("\r{}", game.bottom);

        if let Some(msg) = quit_msg {
            print_error(msg);
            break
        }
    }
}
