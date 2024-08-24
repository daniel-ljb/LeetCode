impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let number: u64 = n.parse::<u64>().unwrap();
        let isEvenLength: bool = n.len() % 2 == 0;

        if number <= 10 {
            (number - 1).to_string()
        }
        else if number == 11 {
            String::from("9")
        }
        else {
            fn generate_palindrome(mut leftHalf: u64, isEvenLength: bool) -> u64 {
                let mut palindrome: u64 = leftHalf;
                if !isEvenLength {
                    leftHalf /= 10;
                }
                while leftHalf > 0 {
                    palindrome *= 10;
                    palindrome += leftHalf % 10;
                    leftHalf /= 10;
                }
                palindrome
            }

            let leftHalf: u64 = n.chars()
                .take((n.len()+1)/2)
                .collect::<String>()
                .parse::<u64>()
                .unwrap();

            let mut candidates: [u64; 5] = [
                generate_palindrome(leftHalf - 1, isEvenLength),
                generate_palindrome(leftHalf, isEvenLength),
                generate_palindrome(leftHalf + 1, isEvenLength),
                10_u64.pow(n.len() as u32 - 1) - 1,
                10_u64.pow(n.len() as u32) + 1
            ];

            let mut closest: u64 = candidates[0];
            let mut min_diff: i64 = (candidates[0] as i64 - number as i64).abs();

            for &candidate in &candidates[1..] {
                if candidate == number {
                    continue
                }
                let diff: i64 = (candidate as i64 - number as i64).abs();

                if diff < min_diff || diff == min_diff && candidate < closest {
                    closest = candidate;
                    min_diff = diff;
                }
            }
            
            closest.to_string()
        }
    }   
}