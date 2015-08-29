use libc::{c_int, c_uint, c_uchar};

const NCHARS: usize = 32;

/// Implementation based on <termios.h> (most of the fields aren't useful for us)
#[repr(C)]  // but, they're needed for foreign communication
#[derive(Clone)]
struct termios {
  c_iflag: c_uint,              // input mode flags
  c_oflag: c_uint,              // output mode flags
  c_cflag: c_uint,              // control mode flags
  c_lflag: c_uint,              // local mode flags
  c_line: c_uint,               // line discipline
  c_cc: [c_uchar; NCHARS],      // special characters
  c_ispeed: c_uint,             // input speed
  c_ospeed: c_uint,             // output speed
}

extern "C" {
  fn tcgetattr(fd_num: c_int, termios_ptr: &mut termios) -> c_int;
  fn tcsetattr(fd_num: c_int, optional_actions: c_int, termios_ptr: &mut termios) -> c_int;
  fn cfmakeraw(termios_ptr: &mut termios);
}

pub fn raw_mode() {
  unsafe {
    let mut new_termios = termios {
      c_iflag: 0,
      c_oflag: 0,
      c_cflag: 0,
      c_lflag: 0,
      c_line: 0,
      c_cc: [0; NCHARS],
      c_ispeed: 0,
      c_ospeed: 0,
    };

    if tcgetattr(0, &mut new_termios) == 0 {
        cfmakeraw(&mut new_termios);
    } else {
        println!("\n\tERROR: Couldn't get terminal attributes!\n");
        panic!("getting terminal attributes")
    }

    if tcsetattr(0, 0, &mut new_termios) != 0 {
        println!("\n\tERROR: Couldn't switch the terminal to raw mode!\n");
        panic!("switching to raw mode")
    } else {
        println!("Yay! Switched to raw mode!");
    }
  }
}
