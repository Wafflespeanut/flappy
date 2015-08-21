use std::iter::repeat;

pub struct FallArea {
    pub width: usize,
    pub height: usize,
}

impl Clone for FallArea{
    fn clone(&self) -> FallArea {
        FallArea {
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}

pub trait Sprite {
    fn draw(&self, frame: &Vec<String>, size: usize) -> Vec<String>;
}

pub fn fill_up(start: &str, end: &str, ch: &str, length: usize) -> String {
    let base = repeat(ch).take(length).collect::<String>();
    start.to_owned() + &base + end
}

pub fn new_draw(area: FallArea, body: Vec<String>, idx: usize) -> Vec<String> {
    let (body_width, body_height) = (body[0].len(), body.len());
    (0..area.height).map(|i| {
        if i < body_height {
            let start = fill_up(" ", " ", " ", idx - 2);
            let end = fill_up(" ", " ", " ", area.width - (idx + body_width) - 2);
            let line = String::from(start);
            line + &body[i] + &end
        } else {
            fill_up(" ", " ", " ", area.width - 2)
        }
    }).collect()
}

pub fn merge_draw(frame: &Vec<String>, body: Vec<String>, idx: usize) -> Vec<String> {
    let body_width = body[0].len();
    (0..frame.len()).map(|i| {
        let line = &frame[i];
        let (start, end) = (&line[..idx], &line[idx + body_width..]);
        let line = String::from(start);
        line + &body[i] + end
    }).collect()
}
