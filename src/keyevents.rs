use libc::{c_int, c_uint, c_uchar};

const NCHARS: usize = 32;

// Implementation based on <termios.h> (I did peak into some of the "termios" crates for getting the structure)
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

extern "C" {
  fn tcgetattr(fd_num: c_int, termios_ptr: &mut Termios) -> c_int;
  fn tcsetattr(fd_num: c_int, optional_actions: c_int, termios_ptr: &mut Termios) -> c_int;
  fn cfmakeraw(termios_ptr: &mut Termios);
}

pub struct TermiosAttribs {     // wrapper struct for the C-like struct
    term: Termios               // created only for later drop
}

impl Drop for TermiosAttribs {
    fn drop(&mut self) {        // override `drop` to set back the old termios attributes on drop
        let _ = unsafe { tcsetattr(0, 0, &mut self.term) };
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

    let old_termios = if tcgetattr(0, &mut new_termios) == 0 {
        TermiosAttribs { term: new_termios.clone() }    // get the old termios and put it into the wrapper
    } else {
        println!("\n\tERROR: Couldn't get terminal attributes!\n");
        panic!("getting terminal attributes")
    };

    cfmakeraw(&mut new_termios);                    // put the attributes for raw termios
    if tcsetattr(0, 0, &mut new_termios) != 0 {     // set the newly obtained attributes to make it raw!
        println!("\n\tERROR: Couldn't switch to raw mode!\n");
        panic!("switching to raw mode")
    } else {
        old_termios     // Yay! switched to raw mode! Now, return the wrapper (for later drop)
    }
  }
}
