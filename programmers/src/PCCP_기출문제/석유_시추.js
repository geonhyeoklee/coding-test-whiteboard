function solution(land) {
  const n = land.length;
  const m = land[0].length;

  const directions = [
    [1, 0],
    [0, 1],
    [-1, 0],
    [0, -1],
  ];

  const countOilLand = (initY, initX, visitedX) => {
    const stack = [[initY, initX]];
    let totalOil = 0;

    while (stack.length > 0) {
      const [y, x] = stack.pop();

      // 랜드 범위를 벗어났을 때 다음 반복으로 넘어간다.
      if (y < 0 || x < 0 || y >= n || x >= m) {
        continue;
      }

      // 이미 방문한 위치이거나 원래부터 오일이 없으면 다음 반복으로 넘어간다.
      if (land[y][x] === 0) {
        continue;
      }

      visitedX.add(x);
      land[y][x] = 0; // 방문한 위치는 0으로 초기화
      totalOil++;

      // 해당 위치에서 4방향으로 탐색 확대를 위해 스택에 위치를 넣는다.
      for (const [dy, dx] of directions) {
        stack.push([y + dy, x + dx]);
      }
    }

    return totalOil;
  };

  const possibleOilInX = new Array(m).fill(0);

  for (let y = 0; y < n; y++) {
    for (let x = 0; x < m; x++) {
      const visitedX = new Set();
      const oil = countOilLand(y, x, visitedX);

      for (const x of visitedX) {
        possibleOilInX[x] += oil;
      }
    }
  }

  return Math.max(...possibleOilInX);
}

const land = [
  [0, 0, 0, 1, 1, 1, 0, 0],
  [0, 0, 0, 0, 1, 1, 0, 0],
  [1, 1, 0, 0, 0, 1, 1, 0],
  [1, 1, 1, 0, 0, 0, 0, 0],
  [1, 1, 1, 0, 0, 0, 1, 1],
];

solution(land);
