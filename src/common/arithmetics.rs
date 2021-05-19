pub fn calculate_ones_complement_sum(numbers: Vec<u16>) -> u16 {
    let mut val: u16 = 0;
    for num in numbers {
        let (v, overflow) = val.overflowing_add(!num);
        val = if overflow { v + 1 } else { v }
    }
    val
}