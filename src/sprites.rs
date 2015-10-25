use helpers::*;
use libc::c_uint;
use keyevents::Key;
use rand::{thread_rng, Rng};
use {CLIFF_SEPARATION, CLIFF_Y, HEIGHT, JUMPER_X, JUMPER_Y, TIMEOUT_MS, WIDTH};

#[derive(Clone, Debug)]
struct Jumper {
    area: FallArea,
    x_pos: usize,
    y_pos: usize,
    body: Vec<String>,
    size: (usize, usize),
}

impl Jumper {
    fn new(fall_area: FallArea) -> Jumper {
        let body = [" \\\\ // ",
                    "==[O]=="];   // assume that it's the front view of a falling jumper
        let size = (body[0].len(), body.len());
        Jumper {
            area: fall_area,
            x_pos: (fall_area.width.0 / 2),
            y_pos: (fall_area.height.0 / 4),
            body: body
                  .iter()
                  .map(|&string| string.to_owned())
                  .collect(),
            size: size
        }
    }

    // base frame over which subsequent frames are drawn
    fn draw(&self) -> Vec<String> {
        let (y_pos, fall_area) = (self.y_pos, self.area);
        let (body_width, body_height) = (self.size.0, self.size.1);
        (0..fall_area.height.0).map(|i| {
            if i < y_pos || i >= y_pos + body_height {
                multiply(" ", fall_area.width.0)
            } else {
                multiply(" ", self.x_pos) +
                &self.body[i - y_pos] +
                &multiply(" ", fall_area.width.0 - (self.x_pos + body_width))
            }
        }).collect()
    }

    fn shift(&mut self, key: Key) {
        match key {
            Key::Right if (self.x_pos + JUMPER_X + self.size.0) < self.area.width.0 => {
                self.x_pos += JUMPER_X;
            },
            Key::Left if (self.x_pos as isize - JUMPER_X as isize) > 0 => {
                self.x_pos -= JUMPER_X;
            },
            Key::Up if (self.y_pos as isize - JUMPER_Y as isize) > 0 => {
                // self.y_pos -= JUMPER_Y;
            },
            Key::Down if (self.y_pos + JUMPER_Y + self.size.1) < self.area.height.0 => {
                // self.y_pos += JUMPER_Y;
            },
            _ => (),
        }
    }
}

#[derive(Clone, Debug)]
struct Cliff {
    x_pos: usize,   // used for initial random positioning of the cliff (restricted to window's width)
    y_pos: usize,   // though it has only one DOF, the cliffs should move upward over consecutive frames
    body: Vec<String>,
    size: (usize, usize),   // cliff size is random and restricted to half the window's width (and a height of "4")
}

impl Cliff {
    fn new(jumper: &Jumper) -> Cliff {   // jumper's position is necessary to throw cliffs at him!
        let mut rng = thread_rng();
        let (full_width, full_height) = (jumper.area.width.0, jumper.area.height.0);
        let half_width = full_width / 2;
        let x_size: usize = rng.gen_range(half_width / 5, half_width - half_width / 5);
        let y_size: usize = rng.gen_range(4, full_height / 5);    // minimum y_size is 4

        let left_side = {
            let jumper_side = jumper.x_pos.le(&half_width);
            rng.choose(&[true, false, jumper_side]).unwrap().clone()    // increase the chance of hitting the jumper
        };

        let x_pos: usize = match left_side {
            true => rng.gen_range(1, full_width / 3) - 1,
            false => full_width - x_size - rng.gen_range(1, full_width / 3) - 1,
        };

        Cliff {
            x_pos: x_pos,
            y_pos: full_height,     // initial position of any cliff is at the bottom
            body: (0..y_size)
                  .map(|part| {
                      if part == 0 {                                        // cliff of size (2, 5)
                          " ".to_owned() + &multiply("_", x_size) + " "     //      __
                      } else if part == 1 {                                 //     /OO\
                          "/".to_owned() + &multiply("O", x_size) + "\\"    //     |OO|
                      } else if part == y_size - 2 {                        //     \OO/
                          "\\".to_owned() + &multiply("O", x_size) + "/"    //      --
                      } else if part == y_size - 1 {                        //
                          " ".to_owned() + &multiply("-", x_size) + " "
                      } else {
                          "|".to_owned() + &multiply("O", x_size) + "|"
                      }
                  }).collect(),
            size: (x_size + 2, y_size),
        }
    }

    fn shift(&mut self) {
        let diff = self.y_pos as isize - CLIFF_Y as isize;
        if diff >= 0 {
            self.y_pos -= CLIFF_Y;
        } else {
            self.body = self.body[1..].to_vec();
            self.size = (self.size.0, self.size.1 - 1);
        }
    }

    fn erase_body(&self) -> bool {
        self.body.len() == 1
    }
}

pub struct Game {   // struct to hold all the objects required for a new game
    // difficulty parameters (inversely proportional to difficulty)
    pub poll_timeout: c_uint,       // how fast the cliffs come & try to bang at you!
    cliff_separation: usize,    // how long before a cliff appears!
    jumper: Jumper,     // jumper is always necessary to draw the picture
    cliffs: Vec<Cliff>,     // vector of all cliffs on the current frame
    num_cliffs: usize,      // just to stop finding the length every time we update the cliffs
    line_since_last: usize,     // line since the last cliff was thrown (required because they're equally spaced)
    collision: Option<&'static str>,    // collision message which is triggered when the game ends
    score: usize,   // score that you see on the lower left corner
    // FIXME: yuck! these should be replaced with cursor controllers "ASAP!"
    side: String,
    top: String,
    bottom: String,
}

impl Game {
    pub fn new(poll_timeout: c_uint, cliff_sep: usize) -> Result<Game, &'static str> {
        let fall_area = match FallArea::new(WIDTH, HEIGHT) {
            Ok(area) => area,
            Err(err) => return Err(err),
        };

        let jumper = Jumper::new(fall_area);
        let cliff = Cliff::new(&jumper);

        let top_indent = fall_area.height.1 / 2;
        let bottom_indent = fall_area.height.1 - top_indent;
        let box_width = fall_area.width.0;
        let left_indent = fall_area.width.1 / 2;

        // draw the dashed box for the frames to be drawn inside
        let dashes = multiply(" ", left_indent) + &multiply("-", box_width) + "--";
        // base & lid of the box
        let user_env_top = multiply("\r\n", top_indent) + &dashes;
        let user_env_bottom = dashes + &multiply("\r\n", bottom_indent);

        Ok(Game {
            poll_timeout: poll_timeout,
            cliff_separation: cliff_sep,
            jumper: jumper,
            cliffs: vec![cliff],
            num_cliffs: 1,
            line_since_last: 0,
            collision: None,
            score: 0,
            // YUCK!!!
            side: multiply(" ", left_indent),
            top: user_env_top,
            bottom: user_env_bottom,
        })
    }

    pub fn is_running(&mut self) -> bool {
        let mut frame = self.jumper.draw();
        // decrement the poll timeout for every 8 units of score
        self.poll_timeout = TIMEOUT_MS - (self.score / 8) as c_uint;
        // decrement the cliff separation for every 100 units of score (I don't think people can make it that far)
        self.cliff_separation = CLIFF_SEPARATION - self.score / 100;
        self.draw_cliffs(&mut frame, false);    // `true` means GOD mode! (ehm, debug mode)

        match self.collision {
            Some(msg) => {
                self.collision = None;      // workaround for printing the last frame while quitting
                self.draw_cliffs(&mut frame, true);
                self.print_frame(&frame);
                print_msg(msg, Some("Y"));
                false
            },
            None => {
                self.print_frame(&frame);
                true
            },
        }
    }

    pub fn print_frame(&self, frame: &[String]) {       // gameplay inside an outlined box
        println!("{}", self.top);
        print!("\r{}|{}", &self.side, frame.join(&("|\n\r".to_owned() + &self.side + "|")));
        print!("\r{}", self.bottom);
        print_msg(&format!("SCORE: {}\tSPEED: {}", self.score, TIMEOUT_MS - self.poll_timeout), Some("G"));
    }

    pub fn jumper_shift(&mut self, key: Key) {
        self.jumper.shift(key)
    }

    pub fn cliffs_shift(&mut self) {
        self.line_since_last += 1;
        let mut i = 0;
        while i < self.num_cliffs {     // shift the cliffs first (i.e., check & update the fields)
            if self.cliffs[i].erase_body() {
                self.cliffs.remove(i);
                self.num_cliffs -= 1;
                self.score += 1;
            }
            self.cliffs[i].shift();
            i += 1;
        }

        let last_cliff_size = self.cliffs[self.num_cliffs - 1].size.1;
        if self.line_since_last > last_cliff_size &&
        self.line_since_last - last_cliff_size == self.cliff_separation {
            self.line_since_last = 0;
            self.cliffs.push(Cliff::new(&self.jumper));
            self.num_cliffs += 1;
        }
    }

    pub fn draw_cliffs(&mut self, frame: &mut [String], ignore_collision: bool) {
        let area = self.jumper.area;
        fn collision(string: &str) -> Option<&'static str> {
            for j in string.chars() {
                match j {
                    '=' => return Some("You tore your arm off!"),
                    '\\' | '/' => return Some("There goes your leg!"),
                    '[' | ']' | 'o' => return Some("You're now headless!"),
                    _ => continue,
                }
            } None
        }

        for cliff in &self.cliffs {
            let (x_pos, y_pos) = (cliff.x_pos, cliff.y_pos);
            let (body_width, body_height) = (cliff.size.0, cliff.size.1);
            for i in y_pos..(y_pos + body_height) {
                if i < area.height.0 {
                    let line = frame[i].clone();
                    let (start, end) = (&line[..x_pos], &line[x_pos + body_width..]);
                    frame[i] = start.to_owned() + &cliff.body[i - y_pos] + end;
                    // we ignore the collision thing only for printing the last frame while quitting
                    // well, this will also be useful for tests
                    self.collision = match ignore_collision {
                        true => None,
                        false => collision(&line[x_pos..x_pos + body_width]),
                    };
                }

                if self.collision.is_some() {
                    return
                }
            }
        }
    }
}
