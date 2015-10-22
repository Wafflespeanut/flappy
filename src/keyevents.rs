use libc::{c_int, c_uint, c_short, c_uchar, STDIN_FILENO};
use std::cmp::Ordering;

const NCHARS: usize = 32;   // ASCII chars 0-31
const POLLIN: i16 = 1;      // represents the event for polling input
const TCSANOW: i32 = 0;     // for setting the termios attributes immediately

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
        unsafe { tcsetattr(STDIN_FILENO, TCSANOW, &mut self.term) };
        print!("\x1B[?25h");    // show the cursor
    }
}

pub fn set_raw_mode() -> TermiosAttribs {
    let mut new_termios = Termios {     // some initial values
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
        let old_termios = match tcgetattr(STDIN_FILENO, &mut new_termios) { // try getting the old termios
            0 => TermiosAttribs { term: new_termios.clone() },  // put it into the wrapper
            _ => {
                println!("\n\tERROR: Can't get terminal attributes!\n");
                panic!("getting terminal attributes")
            },
        };

        cfmakeraw(&mut new_termios);    // get the attributes for raw termios into our termios
        match tcsetattr(STDIN_FILENO, TCSANOW, &mut new_termios) {  // try setting the newly obtained attributes
            0 => {  // Yay! switched to raw mode! Now, return the wrapper (for later drop)
                print!("\x1B[?25l");    // hide the cursor
                println!("\n");
                old_termios
            },
            _ => {
                println!("\n\tERROR: Can't switch to raw mode!\n");
                panic!("switching to raw mode")
            },
        }
    }
}

pub enum Poll {     // we need polling to capture the keystrokes in specific time intervals
    Start,
    Wait,
}

pub fn poll_keypress(timeout_ms: c_uint) -> Poll {
    let mut poll_fd = PollFD {
        fd: STDIN_FILENO,   // since we're capturing the standard input
        events: POLLIN,
        revents: 0,     // will be filled by the kernel denoting the events occurred
    };

    unsafe {
        match poll(&mut poll_fd, 1, timeout_ms).cmp(&0) {
            Ordering::Greater => Poll::Start,   // begin blocking to capture the keystroke
            Ordering::Equal => Poll::Wait,      // indicates that the poll has timed out
            Ordering::Less => {
                println!("\n\tERROR: Can't poll the input!\n\r");
                panic!("polling input")
            },
        }
    }
}

pub enum Key {      // We'll be needing only these keys for the game
    Up,
    Down,
    Right,
    Left,
    Quit,
    Other,
}

pub fn read_keypress() -> Key {
    let mut buffer: u32 = 0;
    unsafe {
        if read(STDIN_FILENO, &mut buffer, 8) < 0 {
            println!("\n\tERROR: Can't read the input!\n\r");
            panic!("reading input")
        } else {
            match buffer {
                3 | 27 => Key::Quit,    // Ctrl-C & Esc
                4283163 => Key::Up,
                4348699 => Key::Down,
                4414235 => Key::Right,
                4479771 => Key::Left,
                _ => Key::Other,
            }
        }       // the keycodes were found initially by pressing keys and printing `buffer`
    }
}
