import { INPUT } from "./02.input.ts";

const EXAMPLE = `
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
`;

const BAG = {
  red: 12,
  green: 13,
  blue: 14,
};

console.log(
  INPUT.trim()
    .split("\n")
    .reduce((p, c) => {
      const id = parseInt(c.match(/Game (\d+):/)![1]);
      const sets = c.match(/Game \d+:(.*)/)![1].split(";");
      const ok = sets.every((s) => {
        const counter = { red: 0, green: 0, blue: 0 };
        const cubes = s.matchAll(/ (\d+) (red|green|blue),?/g);
        for (const match of cubes) {
          counter[match[2] as keyof typeof counter] += parseInt(match[1]);
        }
        return (
          counter.red <= BAG.red &&
          counter.green <= BAG.green &&
          counter.blue <= BAG.blue
        );
      });
      return p + (ok ? id : 0);
    }, 0)
);

console.log(
  INPUT.trim()
    .split("\n")
    .reduce((p, c) => {
      const sets = c.match(/Game \d+:(.*)/)![1].split(";");
      const counter = { red: 0, green: 0, blue: 0 };
      sets.forEach((s) => {
        const cubes = s.matchAll(/ (\d+) (red|green|blue),?/g);
        for (const match of cubes) {
          const key = match[2] as keyof typeof counter;
          counter[key] = Math.max(counter[key], parseInt(match[1]));
        }
      });
      const power = counter.red * counter.green * counter.blue;
      return p + power;
    }, 0)
);
