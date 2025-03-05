"use strict";

import { WriteStream, createWriteStream } from "fs";
process.stdin.resume();
process.stdin.setEncoding("utf-8");

let inputString: string = "";
let inputLines: string[] = [];
let currentLine: number = 0;

process.stdin.on("data", function (inputStdin: string): void {
  inputString += inputStdin;
});

process.stdin.on("end", function (): void {
  inputLines = inputString.split("\n");
  inputString = "";

  main();
});

function readLine(): string {
  return inputLines[currentLine++];
}

/*
 * Complete the 'palindromeIndex' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

function palindromeIndex(s: string): number {
  // Write your code here
  let i: number = 0;
  let j: number = s.length - 1;
  let result: number = -1;
  while (i < j) {
    if (s[i] !== s[j]) {
      if (s[i + 1] === s[j] && s[i + 2] === s[j - 1]) {
        result = i;
        break;
      } else if (s[i] === s[j - 1] && s[i + 1] === s[j - 2]) {
        result = j;
        break;
      }
    }
    i++;
    j--;
  }
  return result;
}

function main() {
  const ws: WriteStream = createWriteStream(process.env["OUTPUT_PATH"]);

  const q: number = parseInt(readLine().trim(), 10);

  for (let qItr: number = 0; qItr < q; qItr++) {
    const s: string = readLine();

    const result: number = palindromeIndex(s);

    ws.write(result + "\n");
  }

  ws.end();
}
