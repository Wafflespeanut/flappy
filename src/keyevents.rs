use libc::{c_int, c_uint, c_short, c_long, c_uchar, c_void, STDIN_FILENO};

const NCHARS: usize = 32;
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

pub enum KeyPressed {
    ArrowUp,
    ArrowDown,
    ArrowRight,
    ArrowLeft,
    Other,
    Esc,
}

extern "C" {
    // termios-related functions (http://linux.die.net/man/3/termios)
    fn tcgetattr(fd_num: c_int, termios_ptr: &mut Termios) -> c_int;
    fn tcsetattr(fd_num: c_int, optional_actions: c_int, termios_ptr: &mut Termios) -> c_int;
    fn cfmakeraw(termios_ptr: &mut Termios);
    // reading function (http://linux.die.net/man/2/read)
    fn read(fd_num: c_int, buffer: &mut usize, count: usize) -> isize;
}

pub struct TermiosAttribs {     // wrapper struct for the C-like struct
    term: Termios               // created only for later drop
}

impl Drop for TermiosAttribs {
    fn drop(&mut self) {        // override `drop` to set back the old termios attributes on drop
        let _ = unsafe { tcsetattr(STDIN_FILENO, TCSANOW, &mut self.term) };  //
    }
}

pub fn set_raw_mode() -> TermiosAttribs {
  unsafe {
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

    let old_termios = if tcgetattr(STDIN_FILENO, &mut new_termios) == 0 {
        TermiosAttribs { term: new_termios.clone() }    // try getting the old termios and put it into the wrapper
    } else {
        println!("\n\tERROR: Can't get terminal attributes!\n");
        panic!("getting terminal attributes")
    };

    cfmakeraw(&mut new_termios);        // get the attributes for raw termios into our termios
    if tcsetattr(STDIN_FILENO, TCSANOW, &mut new_termios) == 0 {    // try setting the newly obtained attributes
        old_termios         // Yay! switched to raw mode! Now, return the wrapper (for later drop)
    } else {
        println!("\n\tERROR: Can't switch to raw mode!\n");
        panic!("switching to raw mode")
    }
  }
}

pub fn read_keys() {
    unsafe {
        let mut buffer: usize = 0;
        if read(STDIN_FILENO, &mut buffer, 8) < 0 {
            println!("\n\tERROR: Can't read the input!\n");
            panic!("reading input")
        } else {
            println!("{}", buffer);     // for now, this just prints the captured keycode
        }
    }
}
