import { readFileSync } from "fs";

function getNumericDigits(line: string): number[] {
  return Array.from(
    line.matchAll(/\d/g),
    (match: Array<string>) => match[0]
  ).map(Number);
}

const WRITTEN_TO_NUMERIC = {
  "one":   1,
  "two":   2,
  "three": 3,
  "four":  4,
  "five":  5,
  "six":   6,
  "seven": 7,
  "eight": 8,
  "nine":  9,
}
  

function getNumericAndWrittenDigits(line: string): number[] {
  let digits = [];

  for (let i = 0; i < line.length; i++) {
    let digit = line
      .substring(i)
      .match(/^(\d|one|two|three|four|five|six|seven|eight|nine)/)?.[0];

    if (digit) {
      digit = Number(digit) || WRITTEN_TO_NUMERIC[digit];
      digits.push(digit);
    }
  }

  return digits;
}

const input = readFileSync('./input.txt').toString().split('\n');


function compute(calibrationFn: (line: string) => number[]): number {
  let sum = 0;

  for (const line of input) {
    const digits = calibrationFn(line);
    const calibrationValue = digits[0] * 10 + digits[digits.length - 1];
    sum += calibrationValue;
    console.log({ line, digits, calibrationValue, sum });
  }

  return sum;
}


const part1 = compute(getNumericDigits);
console.log(`Part 1: ${part1}`);
const part2 = compute(getNumericAndWrittenDigits);
console.log(`Part 2: ${part2}`);
