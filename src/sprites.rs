use helpers::*;
use keyevents::Key;
use rand::{thread_rng, Rng};

pub trait Sprite {      // should be implemented by all the objects in the game
    fn draw(&self, frame: Option<&Vec<String>>) -> Vec<String>;
    fn shift(&mut self, pos: usize, key: Option<Key>);
}

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
            x_pos: (fall_area.width.0 / 2),     // initial position of the jumper
            body: ["  \\\\ //  ",
                   "===[O]==="]    // assume that it's the front view of a falling jumper
                   .iter()
                   .map(|&string| string.to_owned())
                   .collect::<Vec<String>>(),
        }
    }
}

impl Sprite for Jumper {
    fn draw(&self, _frame: Option<&Vec<String>>) -> Vec<String> {   // basic draw doesn't need a frame
        let fall_area = self.area;
        let empty = multiply(" ", fall_area.width.0);
        let (body_width, body_height) = (self.body[0].len(), self.body.len());
        (0..fall_area.height.0).map(|i| {
            if i < body_height {
                let start = multiply(" ", self.x_pos);
                let end = multiply(" ", fall_area.width.0 - (self.x_pos + body_width));
                String::from(start) + &self.body[i] + &end
            } else {
                empty.clone()
            }
        }).collect()
    }

    fn shift(&mut self, x_pos: usize, key: Option<Key>) {
        match key {
            Some(Key::Right) if (self.x_pos + x_pos + self.body[0].len()) < self.area.width.0 => {
                self.x_pos += x_pos;
            },
            Some(Key::Left) if self.x_pos as isize - x_pos as isize > 0 => {
                self.x_pos -= x_pos;
            },
            _ => {},
        }
    }
}

#[derive(Clone)]
pub struct Cliff {
    area: FallArea, // FallArea is always required by the sprites (to know about the dimensions)
    x_pos: usize,   // used for initial random positioning of the cliff (restricted to window's width)
    y_pos: usize,   // though it has only one DOF, the cliffs should move upward over consecutive frames
    body: Vec<String>,  // size of the cliff is random and restricted to half of the window's width (and a height of "4")
}

impl Cliff {
    pub fn new(jumper: &Jumper) -> Cliff {   // jumper's position is necessary to throw cliffs at him!
        let mut rng = thread_rng();
        let full_width = jumper.area.width.0;
        let half_width = full_width / 2;
        let size: usize = rng.gen_range(1, half_width);

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
            y_pos: jumper.area.height.0 - 4,    // initial position of any cliff is at the bottom
            body: (1..5).map(|part| {
                match part {                                                // sample cliff of size "2"
                    1 => String::from(" ") + &multiply("_", size) + " ",    //      __
                    2 => String::from("/") + &multiply("O", size) + "\\",   //     /OO\
                    3 => String::from("\\") + &multiply("O", size) + "/",   //     \OO/
                    4 => String::from(" ") + &multiply("-", size) + " ",    //      --
                    _ => panic!("unexpected value for cliff"),
                }
            }).collect()
        }
    }
}

impl Sprite for Cliff {
    fn draw(&self, frame: Option<&Vec<String>>) -> Vec<String> {
        let frame = match frame {
            Some(vec) => vec,
            None => {
                println!("\n\tERROR: A frame should be supplied for generating a cliff!\n\r");
                panic!("drawing cliff")
            },
        };

        let fall_area = self.area;
        let (x_pos, y_pos) = (self.x_pos, self.y_pos);
        let empty = multiply(" ", fall_area.width.0);
        let (body_width, body_height) = (self.body[0].len(), self.body.len());
        (0..fall_area.height.0).map(|i| {
            if i < y_pos {
                frame[i].clone()
            } else if i < (y_pos + body_height) {
                let line = &frame[i];
                let (start, end) = (&line[..x_pos], &line[x_pos + body_width..]);
                String::from(start) + &self.body[i - y_pos] + end
            } else {
                empty.clone()
            }
        }).collect()
    }

    fn shift(&mut self, y_pos: usize, _key: Option<Key>) {
        self.y_pos += y_pos;
    }
}

pub struct Game {           // struct to hold the global attributes of a new game
    pub jumper: Jumper,
    pub side: String,
    pub top: Vec<String>,
    pub bottom: Vec<String>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {       // let the game begin!
        let fall_area = FallArea::new(width, height);

        let top_indent = fall_area.height.1 / 2;
        let bottom_indent = fall_area.height.1 - top_indent;
        let left_shift = multiply(" ", fall_area.width.1 / 2);
        let box_width = fall_area.width.0;

        // draw the dashed box for the frames to be drawn inside
        let mut user_env_top = (0..top_indent - 1).map(|_| {        // lid of the box
            multiply(" ", box_width)
        }).collect::<Vec<String>>();
        let dashes = String::from("-") + &multiply("-", box_width) + "---";
        user_env_top.push(dashes.clone());
        let mut user_env_bottom = vec![dashes];        // base of the box
        for _ in 0..bottom_indent - 1 {
            user_env_bottom.push(multiply(" ", box_width));
        }

        Game {
            jumper: Jumper::new(fall_area),
            side: left_shift,
            top: user_env_top,
            bottom: user_env_bottom,
        }
    }
}
