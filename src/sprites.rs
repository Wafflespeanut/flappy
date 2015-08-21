use helpers::*;

pub struct Jumper {
    position: usize,
}

impl GameObject for Jumper {
    fn draw(&self, fall_area: FallArea, lines: Vec<String>, pos: usize) -> Vec<String> {
        let idx = (pos - 1) % fall_area.width;
        let body = ["  \\\\   //  ",
                    "   \\\\ //   ",
                    "====[o]====",
                    "    (O)    "]
                    .iter()
                    .map(|&string| string.to_owned())
                    .collect::<Vec<String>>();
        draw_over(lines, body, 11, idx)
    }
}

pub struct Smokes {
    position: usize,
}

impl GameObject for Smokes {
    fn draw(&self, fall_area: FallArea, lines: Vec<String>, size: usize) -> Vec<String> {
        let body: Vec<String> = (1..5).map(|part| {
            match part {
                1 => fill_up(" ", " ", "_", size),     //      __
                2 => fill_up("/", "\\", "O", size),    //     /OO\
                3 => fill_up("\\", "/", "O", size),    //     \OO/
                4 => fill_up(" ", " ", "-", size),     //      --
                _ => panic!("Unexpected value!"),
            }
        }).collect();
        draw_over(lines, body, size + 2, 0)
    }
}
