/**
index   S       ops[index]  explanation
-----   ------  ----------  -----------
0       abcde   1 fg        append fg
1       abcdefg 3 6         print the 6th letter - f
2       abcdefg 2 5         delete the last 5 letters
3       ab      4           undo the last operation, index 2
4       abcdefg 3 7         print the 7th characgter - g
5       abcdefg 4           undo the last operation, index 0
6       abcde   3 4         print the 4th character - d} input
*/

function processData(input) {
  let [n, ...ops] = input.split("\n");

  let s = "";

  let history = [];

  for (let i = 0; i < n; i++) {
    let [op, arg] = ops[i].split(" ");

    switch (op) {
      case "1":
        history.push(s);
        s += arg;
        break;
      case "2":
        history.push(s);
        s = s.slice(0, -arg);
        break;
      case "3":
        console.log(s[arg - 1]);
        break;
      case "4":
        s = history.pop() || "";
        break;
    }
  }
}

process.stdin.resume();
process.stdin.setEncoding("ascii");
_input = "";
process.stdin.on("data", function (input) {
  _input += input;
});

process.stdin.on("end", function () {
  processData(_input);
});
