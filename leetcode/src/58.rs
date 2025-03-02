pub struct Solution;

impl Solution {
  pub fn length_of_last_word(s: String) -> i32 {
    const WHITE_SPACE: char = ' ';
    let mut chars: Vec<char> = s.chars().collect();

    chars.push(' ');

    let mut pin = 0;
    let mut length = match chars[0] {
      WHITE_SPACE => 0,
      _ => 1,
    };

    for i in 0..chars.len() - 1 {
      let cur = chars[i];
      let peek = chars[i + 1];

      if cur.eq(&WHITE_SPACE) {
        if !peek.eq(&WHITE_SPACE) {
          pin = i;
        }
      } else {
        if peek.eq(&WHITE_SPACE) {
          length = i - pin;
        }
      }
    }

    length as i32
  }
}

// fn main () {
//     let test = "a ";
//     let expected = 1;
//     let result = Solution::length_of_last_word(test.to_string());

//     assert_eq!(expected, result);
// }
