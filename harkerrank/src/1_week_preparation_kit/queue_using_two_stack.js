// Queue using pointer
function processData(input) {
  let queries = input.split(/\s/).map((i) => +i);

  let start = 0;
  let queue = [];

  for (let i = 1; i < queries.length; i++) {
    switch (queries[i]) {
      case 1:
        queue.push(queries[++i]);
        break;
      case 2:
        start++;
        break;
      case 3:
        console.log(queue[start]);
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
