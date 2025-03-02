function permute(nums: number[]): number[][] {
  const result: number[][] = [];

  const backtrack = (path: number[], visited: boolean[]) => {
    if (path.length === nums.length) {
      result.push([...path]);
      return;
    }

    for (let i = 0; i < nums.length; i++) {
      if (visited[i]) {
        continue;
      }

      visited[i] = true;
      path.push(nums[i]);
      backtrack(path, visited);
      path.pop();
      visited[i] = false;
    }
  };

  backtrack([], new Array(nums.length).fill(false));
  return result;
}

function permuteWithStack(nums: number[]): number[][] {
  const result: number[][] = [];
  const stack: { path: number[]; visited: boolean[] }[] = [
    { path: [], visited: new Array(nums.length).fill(false) },
  ];

  while (stack.length > 0) {
    const { path, visited } = stack.pop()!;

    if (path.length === nums.length) {
      result.push([...path]);
      continue;
    }

    for (let i = 0; i < nums.length; i++) {
      if (!visited[i]) {
        const newPath = [...path, nums[i]];
        const newVisited = [...visited];
        newVisited[i] = true;
        stack.push({ path: newPath, visited: newVisited });
      }
    }
  }

  return result.reverse();
}

function testPermute() {
  const testCases = [
    {
      input: [1, 2, 3],
      expected: [
        [1, 2, 3],
        [1, 3, 2],
        [2, 1, 3],
        [2, 3, 1],
        [3, 1, 2],
        [3, 2, 1],
      ],
    },
    {
      input: [0, 1],
      expected: [
        [0, 1],
        [1, 0],
      ],
    },
    { input: [1], expected: [[1]] },
  ];

  testCases.forEach(({ input, expected }, index) => {
    const result = permuteWithStack(input);
    console.assert(
      JSON.stringify(result) === JSON.stringify(expected),
      "Test Failed!"
    );
  });
}

testPermute();
