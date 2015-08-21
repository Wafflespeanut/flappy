use std::iter::repeat;

pub fn jumper() -> Vec<String> {
    vec!["  \\\\   //  ",
         "   \\\\ //   ",
         "====[o]====",
         "    (O)    "]
        .iter()
        .map(|&part| String::from(part))
        .collect()
}

fn fill_up(start: &str, end: &str, ch: &str, length: usize) -> String {
    let mut right = repeat(ch).take(length).collect::<String>();
    right.push_str(end);
    start.to_owned() + &right
}

pub fn clouds(size: usize) -> Vec<String> {
    (1..5).map(|part| {
         match part {
             1 => fill_up(" ", " ", "_", size),
             2 => fill_up("/", "\\", "O", size),
             3 => fill_up("\\", "/", "O", size),
             4 => fill_up(" ", " ", "-", size),
             _ => panic!("Unexpected value!"),
         }
     }).collect()
}
