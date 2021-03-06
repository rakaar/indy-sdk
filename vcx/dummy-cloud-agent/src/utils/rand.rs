use rand::{Rng, thread_rng};

pub fn rand_string(len: usize) -> String {
    thread_rng().gen_ascii_chars().take(len).collect()
}
