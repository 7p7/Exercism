pub fn reverse_v2(input: &str) -> String {
    let mut res = String::new();
    for x in input.chars().rev() {
        res.push(x);
    }
    res
}

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect::<String>()
}
