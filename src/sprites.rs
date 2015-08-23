use helpers::*;
use rand::{thread_rng, Rng};

pub trait Sprite {
    fn draw(&self, frame: &Vec<String>) -> Vec<String>;
}

pub struct Jumper {
    pub area: FallArea,
    pub x_pos: usize,
}

impl Jumper {
    pub fn new(fall_area: FallArea) -> Jumper {
        Jumper {
            area: fall_area,
            x_pos: (fall_area.width.0 / 2),
        }
    }
}

impl Sprite for Jumper {
    fn draw(&self, frame: &Vec<String>) -> Vec<String> {
        let idx = (self.x_pos - 1) % self.area.width.0;
        let body = ["  \\\\ //  ",
                    "===[O]==="]
                    .iter()
                    .map(|&string| string.to_owned())
                    .collect::<Vec<String>>();
        new_draw(self.area.clone(), body, idx)
    }
}

pub struct Cliff {
    x_pos: usize,
    y_pos: usize,
    size: usize,
}

impl Cliff {
    fn new(jumper: Jumper) -> Cliff {
        let mut rng = thread_rng();
        let half_width = jumper.area.width.0 / 2;
        let size: usize = rng.gen_range(0, half_width);

        let left_side = {
            let jumper_side = jumper.x_pos.lt(&half_width);
            // increase the probability of hitting the jumper (by 2/3)
            rng.choose(&[true, false, jumper_side]).unwrap().clone()
        };

        let x_pos: usize = match left_side {
            true => rng.gen_range(0, jumper.area.width.0 / 3),
            false => jumper.area.width.0 - size - rng.gen_range(0, jumper.area.width.0 / 3),
        };

        Cliff {
            x_pos: x_pos,
            y_pos: jumper.area.height.0 - 1,        // initial position of any cliff at the bottom
            size: size,
        }
    }
}

impl Sprite for Cliff {
    fn draw(&self, frame: &Vec<String>) -> Vec<String> {
        let size = self.size;
        let body: Vec<String> = (1..5).map(|part| {
            match part {
                1 => String::from(" ") + &multiply("_", size) + " ",     //      __
                2 => String::from("/") + &multiply("O", size) + "\\",    //     /OO\
                3 => String::from("\\") + &multiply("O", size) + "/",    //     \OO/
                4 => String::from(" ") + &multiply("-", size) + " ",     //      --
                _ => panic!("Unexpected value!"),
            }
        }).collect();
        merge_draw(frame, body, self.x_pos)
    }
}
