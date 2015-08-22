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
        _ => panic!("Couldn't get terminal window size!"),
    }
}

pub struct FallArea {
    pub width: (usize, usize),      // (width, extra)
    pub height: (usize, usize),     // (height, extra)
}

impl FallArea {
    pub fn new(width: usize, height: usize) -> FallArea {
        let (rows, cols) = window_size();
        if (width < 60) | (height < 30) | (rows < 30) | (cols < 60) {
            panic!("Minimum window size is 30 rows and 60 columns!")
        } else if (cols < width) | (rows < height) {
            panic!("Requested window size is less than the available size. Please resize your terminal window!")
        } else {
            FallArea {
                width: (width, cols - width),
                height: (height, rows - height),
            }
        }
    }
}

impl Clone for FallArea{
    fn clone(&self) -> FallArea {
        FallArea {
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}

pub fn fill_up(start: &str, end: &str, ch: &str, length: usize) -> String {
    let base = repeat(ch).take(length).collect::<String>();
    start.to_owned() + &base + end
}

pub fn new_draw(area: FallArea, body: Vec<String>, idx: usize) -> Vec<String> {
    let (body_width, body_height) = (body[0].len(), body.len());
    (0..area.height.0).map(|i| {
        if i < body_height {
            let start = fill_up(" ", " ", " ", idx - 2);
            let end = fill_up(" ", " ", " ", area.width.0 - (idx + body_width) - 2);
            let line = String::from(start);
            line + &body[i] + &end
        } else {
            fill_up(" ", " ", " ", area.width.0 - 2)
        }
    }).collect()
}

// pub fn merge_draw(frame: &Vec<String>, body: Vec<String>, idx: usize) -> Vec<String> {
//     let body_width = body[0].len();
//     (0..frame.len()).map(|i| {
//         let line = &frame[i];
//         let (start, end) = (&line[..idx], &line[idx + body_width..]);
//         let line = String::from(start);
//         line + &body[i] + end
//     }).collect()
// }
