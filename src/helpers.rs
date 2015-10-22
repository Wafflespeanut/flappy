use libc::{c_ushort, c_int, STDOUT_FILENO};
use libc::funcs::bsd44::ioctl;
use std::iter::repeat;

// NOTE: system-dependent constant (not available in libc yet, so you'd have to get it from your system)
const TIOCGWINSZ: c_int = 21523;
// minimum width & height (for a smoother gameplay)
const WIN_COLS: usize = 40;
const WIN_ROWS: usize = 30;

#[repr(C)]
struct WindowSize {
    row: c_ushort,
    col: c_ushort,
}

fn window_size <'a>() -> Result<(usize, usize), &'a str> {      // get the current size of the terminal window
    let wsize = WindowSize { row: 0, col: 0 };
    let val = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &wsize) };
    match val {
        0 => Ok((wsize.row as usize, wsize.col as usize)),
        _ => Err("Can't get terminal window size!"),
    }
}

#[derive(Copy, Clone)]
pub struct FallArea {   // necessary type which describes where the game objects should be drawn
    pub width: (usize, usize),      // (width restricted for the game, remaining width)
    pub height: (usize, usize),     // (height restricted for the game, remaining height)
}

impl FallArea {
    pub fn new <'a>(width: usize, height: usize) -> Result<FallArea, &'a str> {
        let size_result = window_size();
        if let Ok((rows, cols)) = size_result {
            if (width < WIN_COLS) | (height < WIN_ROWS) | (rows < WIN_ROWS) | (cols < WIN_COLS) {
                return Err("Minimum window size is 30 rows and 40 columns!")
            } else if (cols - 2 < width) | (rows - 2 < height) {
                // the extra "2" is for drawing the dashed box
                return Err("Requested window size is less than what's available!")
            } else {
                return Ok(FallArea {
                    width: (width, cols - width),
                    height: (height, rows - height),
                })
            }
        } else {
            Err(size_result.unwrap_err())
        }
    }
}

pub fn multiply(ch: &str, length: usize) -> String {    // I don't wanna write this every time! (DRY)
    repeat(ch).take(length).collect()
}

pub fn print_error(err: &str) {
    print!("\r\n\t{}", err);
}
