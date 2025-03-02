function exist(board: string[][], word: string): boolean {
  const dfs = (board: string[][], word: string, i: number, j: number, k: number, visited: boolean[][]): boolean => {
    if (
      i < 0 || i >= board.length || // i가 범위를 벗어나는지 확인
      j < 0 || j >= board[0].length || // j가 범위를 벗어나는지 확인
      visited[i][j] || // 셀이 방문되었는지 확인
      board[i][j] !== word[k] // 셀의 값이 현재 단어의 문자와 일치하지 않는지 확인
    ) {
      return false;
    }

    if (k === word.length - 1) { // 단어의 마지막 문자에 도달했는지 확인
      return true;
    }

    visited[i][j] = true;
    if (dfs(board, word, i - 1, j, k + 1, visited) ||
        dfs(board, word, i + 1, j, k + 1, visited) ||
        dfs(board, word, i, j - 1, k + 1, visited) ||
        dfs(board, word, i, j + 1, k + 1, visited)) {
      return true;
    }
    visited[i][j] = false;

    return false;
  };

  let visited = Array.from({ length: board.length }, () => Array(board[0].length).fill(false));

  for (let i = 0; i < board.length; i++) {
    for (let j = 0; j < board[0].length; j++) {
      if (dfs(board, word, i, j, 0, visited)) {
        return true;
      }
    }
  }

  return false;
};

function existWithStack(board: string[][], word: string): boolean {
  const directions = [
    [-1, 0], [1, 0], [0, -1], [0, 1]
  ];

  for (let i = 0; i < board.length; i++) {
    for (let j = 0; j < board[0].length; j++) {
      if (board[i][j] === word[0]) {
        const stack = [[i, j, 0]];
        const visited = Array.from({ length: board.length }, () => Array(board[0].length).fill(false));
        visited[i][j] = true;

        while (stack.length > 0) {
          const [x, y, k] = stack.pop()!;
          if (k === word.length - 1) { // 단어의 마지막 문자에 도달했는지 확인
            return true;
          }

          for (const [dx, dy] of directions) {
            const nx = x + dx;
            const ny = y + dy;
            if (
              nx >= 0 && nx < board.length && // nx가 범위를 벗어나는지 확인
              ny >= 0 && ny < board[0].length && // ny가 범위를 벗어나는지 확인
              !visited[nx][ny] && // 셀이 방문되었는지 확인
              board[nx][ny] === word[k + 1] // 셀의 값이 다음 단어의 문자와 일치하는지 확인
            ) {
              stack.push([nx, ny, k + 1]); // 다음 셀을 스택에 추가
              visited[nx][ny] = true; // 다음 셀을 방문했다고 표시
            }
          }

          visited[x][y] = false; // 현재 셀을 방문하지 않았다고 표시
        }
      }
    }
  }

  return false;
}

// 알기 쉬운 솔루션
function exist2(board: string[][], word: string): boolean {
  let ans = false

  const m = board.length
  const n = board[0].length
  const total = word.length

  const track = (i, j, l) => {
    if (l === total) {
      ans = true
      return
    }

    if (i >= m || j >= n || i < 0 || j < 0) return
    if (board[i][j] !== word[l]) return

    const pre = board[i][j]
    board[i][j] = '*'

    track(i + 1, j, l + 1)
    track(i, j + 1, l + 1)
    track(i - 1, j, l + 1)
    track(i, j - 1, l + 1)

    board[i][j] = pre
  }

  for (let i = 0; i < m; i++) {
    for (let j = 0; j < n; j++) {
      track(i, j, 0)
    }
  }

  return ans
}

function testExist() {
  const board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]];
  const word = "ABCCED";
  console.log(exist(board, word)); // 예상 출력: true

  const board2 = [["A","B","C","E"],["S","F","E","S"],["A","D","E","E"]];
  const word2 = "ABCESEEEFS";
  console.log(exist(board2, word2)); // 예상 출력: true

  const board3 = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]];
  const word3 = "ABCB";
  console.log(exist(board3, word3)); // 예상 출력: false
}

testExist();

function testExistWithStack() {
  const board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]];
  const word = "ABCCED";
  console.log(existWithStack(board, word)); // 예상 출력: true

  const board2 = [["A","B","C","E"],["S","F","E","S"],["A","D","E","E"]];
  const word2 = "ABCESEEEFS";
  console.log(existWithStack(board2, word2)); // 예상 출력: true

  const board3 = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]];
  const word3 = "ABCB";
  console.log(existWithStack(board3, word3)); // 예상 출력: false
}

testExistWithStack();

