use helpers::*;
use rand::{thread_rng, Rng};

pub trait Sprite {      // should be implemented by all the objects in the game
    fn draw(&self, frame: &Vec<String>) -> Vec<String>;
}

#[derive(Debug, Copy, Clone)]
pub struct Jumper {
    pub area: FallArea,
    pub x_pos: usize,       // for now, he's got only one DOF
}

impl Jumper {
    pub fn new(fall_area: FallArea) -> Jumper {
        Jumper {
            area: fall_area,
            x_pos: (fall_area.width.0 / 2),     // initial position of the jumper
        }
    }
}

impl Sprite for Jumper {
    fn draw(&self, _: &Vec<String>) -> Vec<String> {
        let x_pos = (self.x_pos - 1) % self.area.width.0;   // limit the jumper to the screen's width
        let body = ["  \\\\ //  ",
                    "===[O]==="]    // assume that it's the front view of a falling jumper
                    .iter()
                    .map(|&string| string.to_owned())
                    .collect::<Vec<String>>();
        base_draw(self.area, body, x_pos)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cliff {
    area: FallArea,
    x_pos: usize,   // used for initial random positioning of the cliff (restricted to window's width)
    y_pos: usize,   // though it has only one DOF, the cliffs should move upward over consecutive frames
    size: usize,    // size is random and restricted to half of the window's width
}

impl Cliff {
    pub fn new(jumper: Jumper) -> Cliff {       // jumper's position is necessary to throw cliffs at him!
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
            size: size,         // this is just the length, as the cliff's height is a constant "4" (for now)
        }
    }

    pub fn shift(&self, frame: &Vec<String>) -> Vec<String> {       // will be used for moving the cliff upwards
        Vec::new()
    }
}

impl Sprite for Cliff {
    fn draw(&self, frame: &Vec<String>) -> Vec<String> {
        let size = self.size;
        let body: Vec<String> = (1..5).map(|part| {
            match part {                                                // Cliff of size "2"
                1 => String::from(" ") + &multiply("_", size) + " ",    //      __
                2 => String::from("/") + &multiply("O", size) + "\\",   //     /OO\
                3 => String::from("\\") + &multiply("O", size) + "/",   //     \OO/
                4 => String::from(" ") + &multiply("-", size) + " ",    //      --
                _ => panic!("Unexpected value!"),
            }
        }).collect();
        merge_draw(self.area, &frame, body, self.x_pos, self.y_pos)
    }
}
