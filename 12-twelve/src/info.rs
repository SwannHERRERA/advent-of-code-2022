#[derive(Debug)]
pub struct Info {
    pub line_length: usize,
    pub start_pos: usize,
    pub end_pos: usize,
}

pub fn read_info(bytes: &[u8]) -> Info {
    let mut line_length = None;
    let mut start_pos = None;
    let mut end_pos = None;

    for (i, c) in bytes.iter().enumerate() {
        match c {
            b'S' => start_pos = Some(i),
            b'E' => end_pos = Some(i),
            b'\n' => {
                if line_length.is_none() {
                    line_length = Some(i + 1);
                }
            }
            _ => (),
        }
    }

    Info {
        line_length: line_length.unwrap(),
        start_pos: start_pos.unwrap(),
        end_pos: end_pos.unwrap(),
    }
}
