use libc::{c_int, c_uint, c_short, c_uchar, STDIN_FILENO};
use std::cmp::Ordering;

const NCHARS: usize = 32;
const POLLIN: i16 = 1;
const TCSANOW: i32 = 0;

// Implementation based on <termios.h> (I did peek into a lot of stuff for getting the structure)
// NOTE: All the fields are needed for proper communication with the foreign library!
#[repr(C)]
#[derive(Clone)]
struct Termios {
    c_iflag: c_uint,              // input mode flags
    c_oflag: c_uint,              // output mode flags
    c_cflag: c_uint,              // control mode flags
    c_lflag: c_uint,              // local mode flags
    c_line: c_uint,               // line discipline
    c_cc: [c_uchar; NCHARS],      // special characters
    c_ispeed: c_uint,             // input speed
    c_ospeed: c_uint,             // output speed
}

#[repr(C)]
struct PollFD {
    fd: c_int,                  // file descriptor
    events: c_short,            // requested events
    revents: c_short,           // returned events
}

pub enum Poll {
    Start,
    Wait,
}

pub enum Key {
    Up,
    Down,
    Right,
    Left,
    Esc,
    Other,
}

extern "C" {
    // termios-related functions (http://linux.die.net/man/3/termios)
    fn tcgetattr(fd_num: c_int, termios_ptr: &mut Termios) -> c_int;
    fn tcsetattr(fd_num: c_int, optional_actions: c_int, termios_ptr: &mut Termios) -> c_int;
    fn cfmakeraw(termios_ptr: &mut Termios);
    // polling function (http://linux.die.net/man/2/poll)
    fn poll(file_desc: &mut PollFD, num_file_desc: c_int, timeout_ms: c_uint) -> c_int;
    // reading function (http://linux.die.net/man/2/read)
    fn read(fd_num: c_int, buffer: &mut c_uint, count: c_uint) -> c_int;
}

pub struct TermiosAttribs {     // wrapper struct for the C-like struct
    term: Termios               // created only for later drop
}

impl Drop for TermiosAttribs {
    fn drop(&mut self) {    // override `drop` to set back the old termios attributes on drop
        let _ = unsafe { tcsetattr(STDIN_FILENO, TCSANOW, &mut self.term) };
    }
}

pub fn set_raw_mode() -> TermiosAttribs {
    let mut new_termios = Termios {     // stupid initial values for termios
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; NCHARS],
        c_ispeed: 0,
        c_ospeed: 0,
    };

    unsafe {
        let old_termios = match tcgetattr(STDIN_FILENO, &mut new_termios) {     // try getting the old termios
            0 => TermiosAttribs { term: new_termios.clone() },  // put it into the wrapper
            _ => {
                println!("\n\tERROR: Can't get terminal attributes!\n");
                panic!("getting terminal attributes")
            },
        };

        cfmakeraw(&mut new_termios);        // get the attributes for raw termios into our termios
        match tcsetattr(STDIN_FILENO, TCSANOW, &mut new_termios) {    // try setting the newly obtained attributes
            0 => old_termios,       // Yay! switched to raw mode! Now, return the wrapper (for later drop)
            _ => {
                println!("\n\tERROR: Can't switch to raw mode!\n");
                panic!("switching to raw mode")
            },
        }
    }
}

pub fn poll_keypress(timeout_ms: c_uint) -> Poll {
    let mut poll_fd = PollFD {
        fd: STDIN_FILENO,
        events: POLLIN,
        revents: 0,     // will be filled by the kernel denoting the events occurred
    };

    unsafe {
        match poll(&mut poll_fd, 1, timeout_ms).cmp(&0) {
            Ordering::Greater => Poll::Start,
            Ordering::Equal => Poll::Wait,
            Ordering::Less => {
                println!("\n\tERROR: Can't poll the input!\n");
                panic!("polling input")
            },
        }
    }
}

pub fn read_keypress() -> Key {
    let mut buffer: u32 = 0;
    unsafe {
        if read(STDIN_FILENO, &mut buffer, 8) < 0 {
            println!("\n\tERROR: Can't read the input!\n");
            panic!("reading input")
        } else {
            match buffer {      // the values were found by initially printing `buffer`
                27 => Key::Esc,
                4283163 => Key::Up,
                4348699 => Key::Down,
                4414235 => Key::Right,
                4479771 => Key::Left,
                _ => Key::Other,
            }
        }
    }
}
