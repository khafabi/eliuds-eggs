pub fn egg_count(display_value: u32) -> usize {
    fn count_ones(values: u32, acc: usize) -> usize {
        match values {
            0 => acc,
            _ => count_ones(values >> 1, acc + (values & 1) as usize),
        }
    }

    count_ones(display_value, 0)
}
