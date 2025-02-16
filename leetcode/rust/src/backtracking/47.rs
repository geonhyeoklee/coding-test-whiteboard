fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let path = Vec::new();
    let used = vec![false; nums.len()];

    let mut stack = Vec::new();
    stack.push((path, used));

    while let Some((path, used)) = stack.pop() {
        if path.len() == nums.len() {
            result.push(path);
            continue;
        }

        for i in 0..nums.len() {
            if used[i] {
                continue;
            }

            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }

            let mut path = path.clone();
            let mut used = used.clone();
            path.push(nums[i]);
            used[i] = true;
            stack.push((path, used));
        }
    }

    result.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        let test_cases = vec![
            (
                vec![1, 2, 3],
                vec![
                    vec![1, 2, 3],
                    vec![1, 3, 2],
                    vec![2, 1, 3],
                    vec![2, 3, 1],
                    vec![3, 1, 2],
                    vec![3, 2, 1],
                ],
            ),
            (vec![0, 1], vec![vec![0, 1], vec![1, 0]]),
            (vec![1], vec![vec![1]]),
        ];

        for (i, (input, expected)) in test_cases.into_iter().enumerate() {
            let result = permute(input.clone());
            assert_eq!(result, expected, "Test case {} failed", i + 1);
        }
    }
}

fn main() {
}
