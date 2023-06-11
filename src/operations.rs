pub fn load(state: &mut Vec<i32>, value: i32) {
    state[0] = value;
}

pub fn swap(state: &mut Vec<i32>, memory1: usize, memory2: usize) {
    let temp = state[memory1];
    state[memory1] = state[memory2];
    state[memory2] = temp;
}

pub fn xor(state: &mut Vec<i32>, memory1: usize, memory2: usize) {
    state[memory1] ^= state[memory2];
}

pub fn inc(state: &mut Vec<i32>, memory: usize) {
    state[memory] += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load() {
        let mut state = vec![0; 4];
        load(&mut state, 1);
        assert_eq!(state, vec![1, 0, 0, 0]);
    }

    #[test]
    fn can_swap() {
        let mut state = vec![0, 1, 0, 0, 0];
        swap(&mut state, 1, 2);
        assert_eq!(state, vec![0, 0, 1, 0, 0]);
    }

    #[test]
    fn can_xor() {
        let mut state = vec![0, 1, 0, 1, 0];
        xor(&mut state, 1, 3);
        assert_eq!(state, vec![0, 0, 0, 1, 0]);
    }

    #[test]
    fn can_inc() {
        let mut state = vec![0; 4];
        inc(&mut state, 3);
        assert_eq!(state, vec![0, 0, 0, 1]);
    }
}
