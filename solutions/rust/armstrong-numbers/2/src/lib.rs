pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let len = s.len();

    s.chars()
        .map(|x| (x as u32 - '0' as u32).pow(len as u32))
        .sum::<u32>()
        == num
}
