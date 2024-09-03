impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        }
        else {
            let mut r: i32 = 0;
            let mut y: i32 = x;
            while y != 0 {
                r *= 10;
                r += y % 10;
                y /= 10;
            }
            x == r
        }
    }
}
