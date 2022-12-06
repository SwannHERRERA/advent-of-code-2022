pub fn find_marker(input: &str, marker_size: usize) -> usize {
    let len = input.len();
    let mut i = 0;
    while i < len {
        let slice = input[i..(i + marker_size)].to_string();
        let index_of_jump = find_same_char(slice);
        if index_of_jump == 0 {
            return i + marker_size;
        }
        i += index_of_jump as usize;
    }
    0
}

fn find_same_char(slice: String) -> i32 {
    let mut copy = slice.clone();
    for (idx, c) in copy.drain(..).enumerate() {
        if slice[(idx + 1)..].contains(c) {
            return (idx + 1) as i32;
        }
    }
    0
}
