mod sprites;
mod helpers;

use helpers::*;

fn main() {
    let fall_area = FallArea { width: 100, height: 50 };
    let env = (1..fall_area.height).map(|_| {
        fill_up("\t|", "|", " ", (fall_area.width + 2) as usize)
    }).collect::<Vec<String>>();
    for line in env {
        print!("{}", line);
    }
}
