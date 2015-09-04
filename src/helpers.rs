use libc::{c_ushort, c_int, STDOUT_FILENO};
use libc::funcs::bsd44::ioctl;
use std::iter::repeat;

// NOTE: system-dependent constant (not available in libc yet, so you'd have to get it from your system)
const TIOCGWINSZ: c_int = 21523;

#[repr(C)]
struct WindowSize {
    row: c_ushort,
    col: c_ushort,
}

fn window_size() -> (usize, usize) {            // get the current size of the terminal window
    let wsize = WindowSize { row: 0, col: 0 };
    let val = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &wsize) };
    match val {
        0 => (wsize.row as usize, wsize.col as usize),
        _ => {
            println!("\n\tERROR: Can't get terminal window size!\n\r");
            panic!("getting terminal window size")
        },
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FallArea {   // necessary type which describes where the game objects should be drawn
    pub width: (usize, usize),      // (width restricted for the game, remaining width)
    pub height: (usize, usize),     // (height restricted for the game, remaining height)
}

impl FallArea {
    pub fn new(width: usize, height: usize) -> FallArea {
        let (rows, cols) = window_size();
        if (width < 50) | (height < 30) | (rows < 30) | (cols < 50) {   // for a smoother gameplay
            println!("\n\tERROR: Minimum window size is 30 rows and 50 columns!\n\r");
            panic!("setting window size")
        } else if (cols - 2 < width) | (rows - 2 < height) {    // the extra "2" is for drawing the dashed box
            println!("\n\tERROR: Requested window size is less than what's available!\n\r");
            panic!("setting window size")
        } else {
            FallArea {
                width: (width, cols - width),
                height: (height, rows - height),
            }
        }
    }
}

pub fn multiply(ch: &str, length: usize) -> String {    // I don't wanna write this every time! (DRY)
    repeat(ch).take(length).collect()
}
