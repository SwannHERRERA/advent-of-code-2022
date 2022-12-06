use core::fmt::Debug;

pub fn vec_eq<T: Eq + Debug>(vec1: Vec<T>, vec2: Vec<T>) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }
    for (a, b) in vec1.iter().zip(vec2.iter()) {
        if a != b {
            return false;
        }
    }
    true
}
