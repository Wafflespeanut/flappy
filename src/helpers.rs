use std::iter::repeat;

pub struct FallArea {
    pub width: usize,
    pub height: usize,
}

pub trait GameObject {
    fn draw(&self, fall_area: FallArea, lines: Vec<String>, pos: usize) -> Vec<String>;
}

pub fn fill_up(start: &str, end: &str, ch: &str, length: usize) -> String {
    let mut right = repeat(ch).take(length).collect::<String>();
    right.push_str(end);
    start.to_owned() + &right
}

pub fn draw_over(lines: Vec<String>, body: Vec<String>, width: usize, idx: usize) -> Vec<String> {
    let height = body.len();
    (0..height).map(|i| {
        let line = &lines[i];
        let (start, end) = (&line[..idx], &line[idx + width..]);
        let mut line = String::from(start);
        line.push_str(&body[i]);
        line.to_owned() + &end
    }).collect()
}
