mod sprites;
mod helpers;

use helpers::*;
use sprites::*;

fn main() {
    let fall_area = FallArea { width: 100, height: 50 };
    let user_env = (0..fall_area.height).map(|_| {
        fill_up(" ", " ", " ", fall_area.width - 2)
    }).collect::<Vec<String>>();
    let jumper = Jumper { area: fall_area };
    loop {
        let frame = jumper.draw(&user_env, 44);
        for line in frame {
            println!("\t| {} |", line);
        }
    }
}
