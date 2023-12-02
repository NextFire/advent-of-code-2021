import { INPUT } from "./01.input.ts";

console.log(
  INPUT.replace(/[^\d]*(\d?).*(\d)[^\d]*/g, "$1$2\n")
    .trim()
    .split("\n")
    .reduce((p, c) => p + parseInt(c.length == 2 ? c : c + c), 0)
);

const map: Record<string, string> = {
  one: "1",
  two: "2",
  three: "3",
  four: "4",
  five: "5",
  six: "6",
  seven: "7",
  eight: "8",
  nine: "9",
};
console.log(
  INPUT.trim()
    .split("\n")
    .reduce((p, c) => {
      const first = c.match(
        /(\d|one|two|three|four|five|six|seven|eight|nine)/
      )![1];
      const last = c.match(
        /.*(\d|one|two|three|four|five|six|seven|eight|nine)/
      )![1];
      const parsed = parseInt((map[first] ?? first) + (map[last] ?? last));
      return p + parsed;
    }, 0)
);
