extern crate libc;
extern crate rand;
extern crate time;

mod helpers;
mod keyevents;
mod sprites;

use helpers::print_msg;
use keyevents::*;
use libc::c_uint;
use sprites::Game;
use time::precise_time_ns;

// width & height for game
const WIDTH: usize = 50;
const HEIGHT: usize = 30;
// initial difficulty setting (game speed & cliff rate)
const TIMEOUT_MS: c_uint = 80;
const CLIFF_SEPARATION: usize = 8;
// shift the jumper or cliff by X chars (change this if you change the width & height)
const JUMPER_X: usize = 3;
const JUMPER_Y: usize = 1;       // 2-DOF won't be realistic and so, let's abandon it!
const CLIFF_Y: usize = 1;

fn main() {
    let _raw = match set_raw_mode() {   // old termios attributes (which will be restored on drop)
        Ok(term_attrib) => term_attrib,
        Err(err) => {
            print_msg(err, None);
            return;
        }
    };

    let mut game = match Game::new(TIMEOUT_MS, CLIFF_SEPARATION) {
        Ok(stuff) => stuff,
        Err(err) => {
            print_msg(&err, None);
            return;
        }
    };
    let mut poll_timeout_ms = game.poll_timeout;
    let mut time_since_last_ns: u64 = 0;

    while game.is_running() {
        let start_time = precise_time_ns();
        match poll_keypress(poll_timeout_ms) {      // wait for the given time to capture the input
            Ok(poll) => match poll {
                Poll::Start => {
                    let keypress = read_keypress();
                    match keypress {        // proceeds immediately on input
                        Ok(key) => match key {
                            Key::Quit => {
                                print_msg("\r\tGoodbye!\n\r", Some("B"));
                                break
                            },
                            _ => {
                                time_since_last_ns += precise_time_ns() - start_time;
                                poll_timeout_ms = game.poll_timeout - ((time_since_last_ns / 1000000) as c_uint);
                                game.jumper_shift(key)
                            },
                        },
                        Err(err) => {
                            print_msg(err, None);
                            break
                        }
                    }
                },
                Poll::Wait => {
                    time_since_last_ns = 0;
                    poll_timeout_ms = game.poll_timeout;
                    game.cliffs_shift();
                },
            },
            Err(err) => {
                print_msg(err, None);
                break
            }
        }
    }
}
