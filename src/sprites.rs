use helpers::*;
use keyevents::Key;
use rand::{thread_rng, Rng};
use {WIDTH, HEIGHT, CLIFF_SEPARATION};

#[derive(Clone, Debug)]
struct Jumper {
    area: FallArea,
    x_pos: usize,       // for now, he's got only one DOF
    body: Vec<String>,
    size: (usize, usize),
}

impl Jumper {
    fn new(fall_area: FallArea) -> Jumper {
        let body = ["  \\\\ //  ",
                    "===[O]==="];   // assume that it's the front view of a falling jumper
        Jumper {
            area: fall_area,
            x_pos: (fall_area.width.0 / 2),     // initial x-position of the jumper
            body: body
                  .iter()
                  .map(|&string| string.to_owned())
                  .collect(),
            size: (body[0].len(), body.len())
        }
    }

    // base frame over which subsequent frames are drawn
    fn draw(&self) -> Vec<String> {
        let fall_area = self.area;
        let (body_width, body_height) = (self.size.0, self.size.1);
        (0..fall_area.height.0).map(|i| {
            match i < body_height {
                true => multiply(" ", self.x_pos) + &self.body[i]
                               + &multiply(" ", fall_area.width.0 - (self.x_pos + body_width)),
                false => multiply(" ", fall_area.width.0),
            }
        }).collect()
    }

    fn shift(&mut self, x_pos: usize, key: Key) {
        match key {
            Key::Right if (self.x_pos + x_pos + self.body[0].len()) < self.area.width.0 => {
                self.x_pos += x_pos;
            },
            Key::Left if (self.x_pos as isize - x_pos as isize) > 0 => {
                self.x_pos -= x_pos;
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
        let full_width = jumper.area.width.0;
        let half_width = full_width / 2;
        let x_size: usize = rng.gen_range(2, half_width);
        let y_size: usize = rng.gen_range(4, 6);

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
            y_pos: jumper.area.height.0,    // initial position of any cliff is at the bottom
            body: (0..y_size)
                  .map(|part| {
                      if part == 0 {                                        // cliff of size (4, 5)
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

    fn shift(&mut self, y_pos: usize) {
        let diff = self.y_pos as isize - y_pos as isize;
        if diff >= 0 {
            self.y_pos -= y_pos;
        } else {
            self.body = self.body[1..].to_vec();
            self.size = (self.size.0, self.size.1 - 1);
        }
    }

    fn erase_body(&self) -> bool {
        self.body.len() == 1
    }
}

pub struct Game {           // struct to hold the global attributes of a new game
    jumper: Jumper,
    cliffs: Vec<Cliff>,
    collision: Option<&'static str>,
    // about-to-deprecate fields
    side: String,
    top: String,
    bottom: String,
}

impl Game {
    pub fn new() -> Result<Game, &'static str> {
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
        let user_env_top = multiply("\r\n", top_indent - 1) + &dashes;
        let user_env_bottom = dashes + &multiply("\r\n", bottom_indent - 1);

        Ok(Game {
            jumper: jumper,
            cliffs: vec![cliff],
            collision: None,
            side: multiply(" ", left_indent),
            top: user_env_top,
            bottom: user_env_bottom,
        })
    }

    pub fn is_running(&mut self) -> bool {
        let mut frame = self.jumper.draw();
        self.draw_cliffs(&mut frame, false);

        match self.collision {
            Some(msg) => {
                self.collision = None;      // workaround for printing the last frame while quitting
                self.draw_cliffs(&mut frame, true);
                self.print_frame(&frame);
                print_msg(msg);
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
        println!("\r{}", self.bottom);
    }

    pub fn jumper_shift(&mut self, x_pos: usize, key: Key) {
        self.jumper.shift(x_pos, key)
    }

    pub fn cliffs_shift(&mut self, y_pos: usize) {
        for cliff in &mut self.cliffs {     // shift the cliffs first (i.e., check & update the fields)
            if cliff.erase_body() {
                *cliff = Cliff::new(&self.jumper);
            }
            cliff.shift(y_pos);
        }
    }

    pub fn draw_cliffs(&mut self, frame: &mut [String], ignore_once: bool) {
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
                    self.collision = collision(&line[x_pos..x_pos + body_width]);
                }
                // we ignore the collision thing only for printing the last frame while quitting
                if !ignore_once && self.collision.is_some() {
                    return
                }
            }
        }
    }
}
