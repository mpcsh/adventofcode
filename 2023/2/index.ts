import { readFileSync } from "fs";

const INPUT = readFileSync('./input.txt').toString().split('\n');

const MAXIMA: Record<string, number> = {
  "red": 12,
  "green": 13,
  "blue": 14,
};

function compute(): number {
  let sum = 0;

  for (const line of INPUT) {
    const { id, game } = line
      .match(/^Game (?<id>\d+): (?<game>.*)$/)
      ?.groups ?? {};
    let isPossible = true;
    for (const subgame of game.split('; ')) {
      for (const subsubgame of subgame.split(', ')) {
        const { quantity, color } = subsubgame
          .match(/^(?<quantity>\d+) (?<color>\w+)$/)
          ?.groups ?? {};
        // console.log(`Game ${id}: ${quantity} ${color}`);

        if (Number(quantity) > MAXIMA[color]) {
          isPossible = false;
          // console.log(`Game ${id} impossible due to ${quantity} ${color}`);
        }
      }
    }

    if (isPossible) {
      sum += Number(id);
    }
  }

  return sum;
}

const part1 = compute();
console.log(`Part 1: ${part1}`);
