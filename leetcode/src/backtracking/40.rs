fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut candidates = candidates.clone();
    candidates.sort();

    fn backtrack(
        result: &mut Vec<Vec<i32>>,
        candidates: &Vec<i32>,
        target: i32,
        path: &mut Vec<i32>,
        start: usize,
    ) {
        if target == 0 {
            result.push(path.clone());
            return;
        }

        for i in start..candidates.len() {
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }

            if candidates[i] > target {
                break;
            }

            path.push(candidates[i]);
            backtrack(result, candidates, target - candidates[i], path, i + 1);
            path.pop();
        }
    }

    backtrack(&mut result, &candidates, target, &mut Vec::new(), 0);
    result
}

fn test_combination_sum2() {
    let mut candidates;
    let mut target;
    let mut result;

    candidates = vec![10, 1, 2, 7, 6, 1, 5];
    target = 8;
    result = combination_sum2(candidates, target);
    assert_eq!(
        result,
        vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
        "Test 1 Failed"
    );

    candidates = vec![2, 5, 2, 1, 2];
    target = 5;
    result = combination_sum2(candidates, target);
    assert_eq!(result, vec![vec![1, 2, 2], vec![5]], "Test 2 Failed");
}

fn main() {
    test_combination_sum2();
}
