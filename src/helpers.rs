use libc::{c_ushort, c_int, STDOUT_FILENO};
use libc::funcs::bsd44::ioctl;
use std::iter::repeat;

// system-dependent constant (not available in libc yet, so you'd have to set it for your system)
const TIOCGWINSZ: c_int = 0x5413;

#[repr(C)]
struct WindowSize {
    row: c_ushort,
    col: c_ushort,
}

fn window_size() -> (usize, usize) {
    let wsize = WindowSize { row: 0, col: 0 };
    let val = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &wsize) };
    match val {
        0 => (wsize.row as usize, wsize.col as usize),
        _ => {
            println!("\n\tERROR: Couldn't get terminal window size!\n");
            panic!("getting terminal window size")
        },
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FallArea {
    pub width: (usize, usize),      // (width, extra)
    pub height: (usize, usize),     // (height, extra)
}

impl FallArea {
    pub fn new(width: usize, height: usize) -> FallArea {
        let (rows, cols) = window_size();
        if (width < 50) | (height < 30) | (rows < 30) | (cols < 50) {
            println!("\n\tERROR: Minimum window size is 30 rows and 50 columns!\n");
            panic!("setting window size")
        } else if (cols - 2 < width) | (rows - 2 < height) {    // the extra "2" is for drawing the box
            println!("\n\tERROR: Requested window size is less than what's available!\n");
            panic!("setting window size")
        } else {
            FallArea {
                width: (width, cols - width),
                height: (height, rows - height),
            }
        }
    }
}

pub fn multiply(ch: &str, length: usize) -> String {
    repeat(ch).take(length).collect()
}

// Always used by Jumper and it's the base frame over which the subsequent frames are drawn
pub fn base_draw(area: FallArea, body: Vec<String>, x_pos: usize) -> Vec<String> {
    let empty = multiply(" ", area.width.0);
    let (body_width, body_height) = (body[0].len(), body.len());
    (0..area.height.0).map(|i| {
        if i < body_height {
            let start = multiply(" ", x_pos);
            let end = multiply(" ", area.width.0 - (x_pos + body_width));
            String::from(start) + &body[i] + &end
        } else {
            empty.clone()
        }
    }).collect()
}

// Always used by the obstacles, and they need y-position because they're always gonna move upwards
pub fn merge_draw(area: FallArea, frame: &Vec<String>, body: Vec<String>, x_pos: usize, y_pos: usize) -> Vec<String> {
    let empty = multiply(" ", area.width.0);
    let (body_width, body_height) = (body[0].len(), body.len());
    (0..area.height.0).map(|i| {
        if i < y_pos {
            frame[i].clone()
        } else if i < (y_pos + body_height) {
            let line = &frame[i];
            let (start, end) = (&line[..x_pos], &line[x_pos + body_width..]);
            String::from(start) + &body[i - y_pos] + end
        } else {
            empty.clone()
        }
    }).collect()
}
