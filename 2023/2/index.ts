import { readFileSync } from "fs";

const INPUT = readFileSync('./input.txt').toString().split('\n');
const EXAMPLE = readFileSync('./example1.txt').toString().split('\n');

type Game = {
  id: number,
  subGames: SubGame[],
}

type SubGame = SubSubGame[];

type SubSubGame = {
  color: string,
  quantity: number,
}

function parseSubSubGame(subSubGameStr: string): SubSubGame {
  const { quantityStr, color } = subSubGameStr
    .match(/^(?<quantityStr>\d+) (?<color>\w+)$/)
    ?.groups ?? {};
  // console.log(`Game ${id}: ${quantity} ${color}`);
  return { color, quantity: Number(quantityStr) };
}


function parseSubGames(subGamesStr: string): SubGame[] {
  let subGames = [];
  for (const subGameStr of subGamesStr.split('; ')) {
    let subGame = [];
    for (const subSubGameStr of subGameStr.split(', ')) {
      subGame.push(parseSubSubGame(subSubGameStr));
    }
    subGames.push(subGame);
  }
  return subGames;
}

function parseGame(gameStr: string): Game {
  const { idStr, subGamesStr } = gameStr
    .match(/^Game (?<idStr>\d+): (?<subGamesStr>.*)$/)
    ?.groups ?? {};

  const game = {
    id: Number(idStr),
    subGames: parseSubGames(subGamesStr),
  };

  // console.log(`Game ${game.id}: ${game.subGames}`);

  return game;
}

function parseGames(gameStrs: string[]): Game[] {
  return gameStrs.map(parseGame);
}

function part1(input: string[]): number {
  const MAXIMA: Record<string, number> = {
    red: 12,
    green: 13,
    blue: 14,
  };

  const games = parseGames(input);

  let sum = 0;
  for (const { id, subGames } of games) {
    let isPossible = true;
    for (const subGame of subGames) {
      for (const { color, quantity } of subGame) {
        if (quantity > MAXIMA[color]) {
          isPossible = false;
        }
      }
    }
    if (isPossible) {
      sum += Number(id);
    }
  }

  return sum;
}

function power(game: Game): number {
  let maxima: Record<string, number> = {
    red: 0,
    green: 0,
    blue: 0,
  };

  for (const subGame of game.subGames) {
    for (const { color, quantity } of subGame) {
      if (quantity > maxima[color]) {
        maxima[color] = quantity;
      }
    }
  }

  return maxima.red * maxima.green * maxima.blue;
}

function part2(input: string[]): number {
  const games = parseGames(input);
  return games.reduce((sum, game) => sum + power(game), 0);
}


const part1Result = part1(INPUT);
console.log(`Part 1: ${part1Result}`);
const part2Result = part2(INPUT);
console.log(`Part 2: ${part2Result}`);
