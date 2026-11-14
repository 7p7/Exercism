pub fn is_armstrong_number(num: u32) -> bool {
    let len = num.to_string().len();
    let mut sum: u32 = 0;
    let mut n = num;

    while n > 0 {
        let r = n % 10;
        sum += r.pow(len as u32);
        if sum > num {
            return false;
        }
        n = n / 10;
    }
    sum == num
}
