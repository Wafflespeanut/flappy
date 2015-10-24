use helpers::*;
use keyevents::Key;
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Jumper {
    area: FallArea,
    x_pos: usize,       // for now, he's got only one DOF
    body: Vec<String>,
}

impl Jumper {
    pub fn new(fall_area: FallArea) -> Jumper {
        Jumper {
            area: fall_area,
            x_pos: (fall_area.width.0 / 2),     // initial x-position of the jumper
            body: ["  \\\\ //  ",
                   "===[O]==="]    // assume that it's the front view of a falling jumper
                   .iter()
                   .map(|&string| string.to_owned())
                   .collect(),
        }
    }

    // base frame over which subsequent frames are drawn (especially the ones with cliffs)
    pub fn draw(&self) -> Vec<String> {
        let fall_area = self.area;
        let (body_width, body_height) = (self.body[0].len(), self.body.len());
        (0..fall_area.height.0).map(|i| {
            match i < body_height {
                true => multiply(" ", self.x_pos) + &self.body[i]
                               + &multiply(" ", fall_area.width.0 - (self.x_pos + body_width)),
                false => multiply(" ", fall_area.width.0),
            }
        }).collect()
    }

    pub fn shift(&mut self, x_pos: usize, key: Key) {
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

#[derive(Clone)]
pub struct Cliff {
    area: FallArea, // FallArea is always required by the sprites (to know about the dimensions)
    x_pos: usize,   // used for initial random positioning of the cliff (restricted to window's width)
    y_pos: usize,   // though it has only one DOF, the cliffs should move upward over consecutive frames
    body: Vec<String>,  // size of the cliff is random and restricted to half of the window's width (and a height of "4")
    collision: Option<String>,    // has the message related to the collision
}

impl Cliff {
    pub fn new(jumper: &Jumper) -> Cliff {   // jumper's position is necessary to throw cliffs at him!
        let mut rng = thread_rng();
        let full_width = jumper.area.width.0;
        let half_width = full_width / 2;
        let size: usize = rng.gen_range(3, half_width);

        let left_side = {
            let jumper_side = jumper.x_pos.le(&half_width);
            rng.choose(&[true, false, jumper_side]).unwrap().clone()    // increase the chance of hitting the jumper
        };

        let x_pos: usize = match left_side {
            true => rng.gen_range(1, full_width / 3) - 1,
            false => jumper.area.width.0 - size - rng.gen_range(1, full_width / 3) - 1,
        };

        Cliff {
            area: jumper.area,
            x_pos: x_pos,
            y_pos: jumper.area.height.0,    // initial position of any cliff is at the bottom
            body: (1..5).map(|part| {
                match part {                                             // sample cliff of size "2"
                    1 => " ".to_owned() + &multiply("_", size) + " ",    //      __
                    2 => "/".to_owned() + &multiply("O", size) + "\\",   //     /OO\
                    3 => "\\".to_owned() + &multiply("O", size) + "/",   //     \OO/
                    4 => " ".to_owned() + &multiply("-", size) + " ",    //      --
                    _ => panic!("unexpected value for cliff"),
                }
            }).collect(),
            collision: None,
        }
    }

    pub fn draw(&mut self, frame: &[String]) -> Vec<String> {
        // mutable reference because we gotta check whether we've collided and update as necessary
        fn collision(string: &str) -> Option<&str> {
            for j in string.chars() {
                match j {
                    '=' => return Some("You tore your arm off!"),
                    '\\' | '/' => return Some("There goes your leg!"),
                    '[' | ']' | 'o' => return Some("You're now headless!"),
                    _ => continue,
                }
            } None
        }

        let fall_area = self.area;
        let mut error = None;
        let (x_pos, y_pos) = (self.x_pos, self.y_pos);
        let (body_width, body_height) = (self.body[0].len(), self.body.len());
        let frame_with_cliff = (0..fall_area.height.0).map(|i| {
            if i < y_pos {
                frame[i].clone()
            } else if i < (y_pos + body_height) {
                let line = &frame[i];
                let (start, end) = (&line[..x_pos], &line[x_pos + body_width..]);
                error = collision(&line[x_pos..x_pos + body_width]);
                start.to_owned() + &self.body[i - y_pos] + end
            } else {
                frame[i].clone()
            }
        }).collect();
        self.collision = error.map(|err| err.to_owned());
        frame_with_cliff
    }

    pub fn shift(&mut self, y_pos: usize) {
        let diff = self.y_pos as isize - y_pos as isize;
        if diff >= 0 {
            self.y_pos -= y_pos;
        } else {
            self.body = self.body[1..].to_vec();
        }
    }

    pub fn collision_msg(&self) -> Option<String> {
        self.collision.clone()
    }

    pub fn erase_body(&self) -> bool {
        self.body.len() == 1
    }
}

pub struct Game {           // struct to hold the global attributes of a new game
    pub jumper: Jumper,
    pub side: String,
    pub top: String,
    pub bottom: String,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Result<Game, String> {   // let the game begin!
        let fall_area = match FallArea::new(width, height) {
            Ok(area) => area,
            Err(err) => return Err(err.to_owned()),
        };
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
            jumper: Jumper::new(fall_area),
            side: multiply(" ", left_indent),
            top: user_env_top,
            bottom: user_env_bottom,
        })
    }
}
