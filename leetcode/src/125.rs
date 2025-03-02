pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut new_s = String::new();
        let chars: Vec<char> = s.trim().to_lowercase().chars().collect();

        for c in chars {
            if Solution::is_digit(c) || Solution::is_letter(c) {
                new_s.push(c);
            }
        }

        let rev_s: String = new_s.chars().rev().collect();

        new_s.eq(&rev_s)
    }

    pub fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    pub fn is_letter(c: char) -> bool {
        c >= 'a' && c <= 'z'
    }
}

fn main() {
    let s = "A man, a plan, a canal: Panama".to_string();
    let expected = true;
    let result = Solution::is_palindrome(s);

    assert_eq!(expected, result);
}
