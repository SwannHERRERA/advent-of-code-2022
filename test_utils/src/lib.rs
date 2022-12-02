use core::fmt::Debug;

pub fn vec_eq<T: Eq + Debug>(vec1: Vec<T>, vec2: Vec<T>) -> bool {
    for (a, b) in vec1.iter().zip(vec2.iter()) {
        if a != b {
            return false;
        }
    }
    true
}
