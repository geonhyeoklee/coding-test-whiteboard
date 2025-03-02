pub struct Solution;
#[derive(PartialEq)]
enum Paren {
    Normal,
    Curly,
    Square,
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(10_000);
        let bytes = s.bytes();

        if bytes.len() % 2 != 0 {
            return false;
        }
        for byte in bytes {
            match byte {
                b'(' => stack.push(Paren::Normal),
                b'{' => stack.push(Paren::Curly),
                b'[' => stack.push(Paren::Square),
                b')' => {
                    if stack.pop() != Some(Paren::Normal) {
                        return false;
                    }
                }
                b'}' => {
                    if stack.pop() != Some(Paren::Curly) {
                        return false;
                    }
                }
                b']' => {
                    if stack.pop() != Some(Paren::Square) {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }

        stack.is_empty()
    }
}

fn main() {
    let s = "{[]}".to_string();
    let expected = true;
    let result = Solution::is_valid(s);

    assert_eq!(expected, result);
}
