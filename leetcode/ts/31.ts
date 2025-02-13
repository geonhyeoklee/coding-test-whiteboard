/**
 Do not return anything, modify nums in-place instead.
 */
function nextPermutation(nums: number[]): void {
  console.log(nums);

  let i = nums.length - 2; // 뒤에서 두 번째 요소부터 시작
  // 첫 번째 감소하는 요소를 찾음
  while (i >= 0 && nums[i] >= nums[i + 1]) {
    i--;
  }

  console.log(i);

  if (i >= 0) {
    let j = nums.length - 1; // 뒤에서부터 시작
    // nums[i]보다 큰 첫 번째 요소를 찾음
    while (j >= 0 && nums[j] <= nums[i]) {
      j--;
    }

    console.log(j);

    [nums[i], nums[j]] = [nums[j], nums[i]];
  }

  console.log(nums);

  const reverse = (nums: number[], start: number) => {
    let i = start;
    let j = nums.length - 1;

    while (i < j) {
      [nums[i], nums[j]] = [nums[j], nums[i]];
      i++;
      j--;
    }
  };

  reverse(nums, i + 1);

  console.log(nums);
}

// function nextPermutation(nums: number[]): void {
//   const _nums = [...nums].sort((a, b) => a - b);

//   const stack: { path: number[]; used: boolean[] }[] = [
//     {
//       path: [],
//       used: Array(nums.length).fill(false),
//     },
//   ];

//   const results: number[][] = [];

//   while (stack.length) {
//     const { path, used } = stack.pop()!;

//     if (path.length === _nums.length) {
//       results.push(path);
//       continue;
//     }

//     for (let i = 0; i < _nums.length; i++) {
//       if (used[i]) {
//         continue;
//       }

//       if (i > 0 && _nums[i] === _nums[i - 1] && !used[i - 1]) {
//         continue;
//       }

//       used[i] = true;
//       stack.push({
//         path: [...path, _nums[i]],
//         used: [...used],
//       });
//       used[i] = false;
//     }
//   }

//   const isEquals = (a: number[]) => (b: number[]) => {
//     for (let i = 0; i < a.length; i++) {
//       if (a[i] !== b[i]) {
//         return false;
//       }
//     }

//     return true;
//   };

//   const numsIndex = results.reverse().findIndex(isEquals(nums));
//   const newNums =
//     numsIndex < results.length - 1
//       ? [...results[numsIndex + 1]]
//       : [...results[0]];

//   for (let i = 0; i < nums.length; i++) {
//     nums[i] = newNums[i];
//   }

//   console.log(nums);
// }

const mock = [1, 3, 2];
nextPermutation(mock);
