// digits = "23"
const digitsMap = new Map<string, string[]>([
  ["2", ["a", "b", "c"]],
  ["3", ["d", "e", "f"]],
  ["4", ["g", "h", "i"]],
  ["5", ["j", "k", "l"]],
  ["6", ["m", "n", "o"]],
  ["7", ["p", "q", "r", "s"]],
  ["8", ["t", "u", "v"]],
  ["9", ["w", "x", "y", "z"]],
]);

function letterCombinations(digits: string): string[] {
  if (digits.length === 0) {
    return [];
  }

  const result: string[] = [];

  const backtrack = (index: number, path: string) => {
    if (path.length === digits.length) {
      result.push(path);
      return;
    }

    const letters = digitsMap.get(digits[index])!;
    for (const letter of letters) {
      backtrack(index + 1, path + letter);
    }
  };

  backtrack(0, "");
  return result;
}

function testLetterCombinations() {
  const testCases: { input: string; expected: string[] }[] = [
    {
      input: "23",
      expected: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
    },
    { input: "2", expected: ["a", "b", "c"] },
    { input: "9", expected: ["w", "x", "y", "z"] },
    { input: "", expected: [] },
    {
      input: "234",
      expected: [
        "adg",
        "adh",
        "adi",
        "aeg",
        "aeh",
        "aei",
        "afg",
        "afh",
        "afi",
        "bdg",
        "bdh",
        "bdi",
        "beg",
        "beh",
        "bei",
        "bfg",
        "bfh",
        "bfi",
        "cdg",
        "cdh",
        "cdi",
        "ceg",
        "ceh",
        "cei",
        "cfg",
        "cfh",
        "cfi",
      ],
    },
  ];

  testCases.forEach(({ input, expected }, index) => {
    const result = letterCombinations(input);
    console.assert(
      JSON.stringify(result) === JSON.stringify(expected),
      `Test case ${index + 1} failed: expected ${JSON.stringify(
        expected
      )}, got ${JSON.stringify(result)}`
    );
  });

  console.log("All test cases passed!");
}

testLetterCombinations();
