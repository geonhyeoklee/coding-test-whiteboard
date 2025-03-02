function permuteUnique(nums: number[]): number[][] {
  const result: number[][] = [];

  nums.sort((a, b) => a - b);

  const backtrack = (path: number[], used: boolean[]) => {
    if (path.length === nums.length) {
      result.push([...path]);
      return;
    }

    for (let i = 0; i < nums.length; i++) {
      if (used[i]) {
        continue;
      }

      if (i > 0 && nums[i] === nums[i - 1] && !used[i - 1]) {
        continue;
      }

      used[i] = true;
      path.push(nums[i]);
      backtrack(path, used);
      path.pop();
      used[i] = false;
    }
  };

  backtrack([], Array(nums.length).fill(false));

  return result;
}

function permuteUniqueWithStack(nums: number[]): number[][] {
  const result: number[][] = [];

  nums.sort((a, b) => a - b);

  const stack: { path: number[]; used: boolean[] }[] = [
    { path: [], used: new Array(nums.length).fill(false) },
  ];

  while (stack.length > 0) {
    const { path, used } = stack.pop()!;

    if (path.length === nums.length) {
      result.push([...path]);
    }

    for (let i = 0; i < nums.length; i++) {
      if (used[i]) {
        continue;
      }

      if (i > 0 && nums[i] === nums[i - 1] && !used[i - 1]) {
        continue;
      }

      const newPath = [...path, nums[i]];
      const newUsed = [...used];
      newUsed[i] = true;
      stack.push({ path: newPath, used: newUsed });
    }
  }

  return result.reverse(); // Remove result.reverse()
}

// Test function
function testPermuteUnique() {
  // const result1 = permuteUniqueWithStack([1, 1, 2]);
  // console.assert(
  //   JSON.stringify(result1) ===
  //     JSON.stringify([
  //       [1, 1, 2],
  //       [1, 2, 1],
  //       [2, 1, 1],
  //     ]),
  //   `Test case 1 failed`
  // );

  const result2 = permuteUniqueWithStack([3, 3, 0, 3]);
  console.assert(
    JSON.stringify(result2) ===
      JSON.stringify([
        [0, 3, 3, 3],
        [3, 0, 3, 3],
        [3, 3, 0, 3],
        [3, 3, 3, 0],
      ]),
    `Test case 1 failed`
  );

  // const result2 = permuteUnique([1, 2, 3]);
  // console.assert(
  //   JSON.stringify(result2) ===
  //     JSON.stringify([
  //       [1, 2, 3],
  //       [1, 3, 2],
  //       [2, 1, 3],
  //       [2, 3, 1],
  //       [3, 1, 2],
  //       [3, 2, 1],
  //     ]),
  //   `Test case 2 failed`
  // );
}

testPermuteUnique();
