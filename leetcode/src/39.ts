function combinationSum(candidates: number[], target: number): number[][] {
  const result: number[][] = [];

  const backtrack = (target: number, path: number[], start: number) => {
    if (target < 0) {
      return;
    }

    if (target === 0) {
      result.push([...path]);
      return;
    }

    for (let i = start; i < candidates.length; i++) {
      path.push(candidates[i]);
      backtrack(target - candidates[i], path, i);
      path.pop();
    }
  };

  backtrack(target, [], 0);

  return result;
}

function testCombinationSum() {
  const testCases = [
    { candidates: [2, 3, 6, 7], target: 7, expected: [[2, 2, 3], [7]] },
    {
      candidates: [2, 3, 5],
      target: 8,
      expected: [
        [2, 2, 2, 2],
        [2, 3, 3],
        [3, 5],
      ],
    },
    { candidates: [2], target: 1, expected: [] },
  ];

  testCases.forEach(({ candidates, target, expected }, index) => {
    const result = combinationSum(candidates, target);
    console.assert(
      JSON.stringify(result) === JSON.stringify(expected),
      `Test Case ${index + 1} Failed`
    );
  });
}

testCombinationSum();
