function combinationSum2(candidates: number[], target: number): number[][] {
  const result: number[][] = [];
  const _candidates = candidates.sort((a, b) => a - b);

  const backtrack = (target: number, path: number[], start: number) => {
    if (target === 0) {
      result.push([...path]);
      return;
    }

    for (let i = start; i < _candidates.length; i++) {
      if (i > start && _candidates[i] === _candidates[i - 1]) {
        continue;
      }

      if (_candidates[i] > target) {
        break;
      }

      path.push(_candidates[i]);
      backtrack(target - _candidates[i], path, i + 1);
      path.pop();
    }
  };

  backtrack(target, [], 0);
  return result;
}

// 테스트 함수
function testCombinationSum2() {
  let candidates, target, result;

  candidates = [10, 1, 2, 7, 6, 1, 5];
  target = 8;
  result = combinationSum2(candidates, target);
  console.assert(
    JSON.stringify(result) ===
      JSON.stringify([
        [1, 1, 6],
        [1, 2, 5],
        [1, 7],
        [2, 6],
      ]),
    `테스트 1 실패`
  );

  // candidates = [2, 5, 2, 1, 2];
  // target = 5;
  // result = combinationSum2(candidates, target);
  // console.assert(
  //   JSON.stringify(result) === JSON.stringify([[1, 2, 2], [5]]),
  //   `테스트 2 실패`
  // );
}

testCombinationSum2();
