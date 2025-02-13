// O(log n)으로 풀어야 함
// Binary search
function search(nums: number[], target: number): number {
  const stack = [
    {
      left: 0,
      right: nums.length - 1,
    },
  ];

  while (stack.length > 0) {
    const { left, right } = stack.pop()!;

    if (left > right) {
      continue;
    }

    const mid = Math.floor((left + right) / 2);

    if (nums[mid] === target) {
      return mid;
    }

    if (nums[left] <= nums[mid]) {
      if (nums[left] <= target && target <= nums[mid]) {
        stack.push({ left, right: mid - 1 });
      } else {
        stack.push({ left: mid + 1, right });
      }
      continue;
    }

    if (nums[mid] <= nums[right]) {
      if (nums[mid] <= target && target <= nums[right]) {
        stack.push({ left: mid + 1, right });
      } else {
        stack.push({ left, right: mid - 1 });
      }
    }
  }

  return -1;
}

function testSearch() {
  const nums = [4, 5, 6, 7, 0, 1, 2];
  const target = 0;
  const expectedOutput = 4;
  const result = search(nums, target);
  console.assert(result === expectedOutput, "Test failed");
}

testSearch();
