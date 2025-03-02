function threeSumClosest(nums: number[], target: number): number {
  nums.sort((a, b) => a - b);
  let closest = nums[0] + nums[1] + nums[2];

  for (let i = 0; i < nums.length - 2; i++) {
    let left = i + 1;
    let right = nums.length - 1;

    while (left < right) {
      let sum = nums[i] + nums[left] + nums[right];

      if (Math.abs(target - sum) < Math.abs(target - closest)) {
        closest = sum;
      }

      if (sum < target) {
        left++;
      } else {
        right--;
      }
    }
  }

  return closest;
}

// Test function
function testThreeSumClosest() {
  let nums1 = [-1, 2, 1, -4];
  let target1 = 1;
  console.log(threeSumClosest(nums1, target1)); // Expected output: 2

  let nums2 = [0, 0, 0];
  let target2 = 1;
  console.log(threeSumClosest(nums2, target2)); // Expected output: 0
}

// Run tests
testThreeSumClosest();

// nums = [-1,2,1,-4], target = 1
// nums = [0,0,0], target = 1
