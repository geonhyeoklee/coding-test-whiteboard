pub struct Solution;
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let s: Vec<u8> = needle
            .bytes()
            .chain("#".bytes())
            .chain(haystack.bytes())
            .collect();

        println!("{:?}", s);

        let mut pi = vec![0usize; s.len()];

        for i in 1..s.len() {
            let mut j = pi[i - 1];

            while j > 0 && s[j] != s[i] {
                j = pi[j - 1];
            }

            if s[i] == s[j] {
                j += 1;
            }

            pi[i] = j;
        }

        println!("{:?}", pi);

        for i in needle.len() + 1..s.len() {
            if pi[i] == needle.len() {
                return (i - needle.len() * 2) as i32;
            }
        }

        return -1;
    }
}
fn main() {
    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();

    let expected = 0;
    let result = Solution::str_str(haystack, needle);

    assert_eq!(expected, result);
}
