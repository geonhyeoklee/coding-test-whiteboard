// O(n)으로는 불가능
// 시간 복잡도 개선을 위해 이진 탐색이 필요하다.
function solution(diffs, times, limit) {
  return binarySearch(diffs, times, limit);
}

function binarySearch(diffs, times, limit) {
  let top = (() => {
    let max = 0;
    for (const diff of diffs) {
      max = Math.max(max, diff);
    }
    return max;
  })();

  let down = 1;
  let middle = 0;

  while (down <= top) {
    middle = Math.floor((top + down) / 2);

    if (checkSuccess(diffs, times, limit, middle)) {
      top = middle - 1;
    } else {
      down = middle + 1;
    }
  }

  if (checkSuccess(diffs, times, limit, middle)) {
    return middle;
  } else if (checkSuccess(diffs, times, limit, middle + 1)) {
    return middle + 1;
  } else {
    return middle - 1;
  }
}

function checkSuccess(diffs, times, limit, level) {
  let totalTime = 0;

  for (let i = 0; i < diffs.length; i++) {
    const currentDiff = diffs[i];
    const currentTime = times[i];

    if (currentDiff <= level) {
      totalTime += currentTime;
    } else {
      const prevTime = times[i - 1];
      totalTime +=
        (currentTime + prevTime) * (currentDiff - level) + currentTime;
    }
  }

  return totalTime <= limit;
}

const diffs = [1, 328, 467, 209, 54];
const times = [2, 7, 1, 4, 3];
const limit = 1723;

solution(diffs, times, limit);
