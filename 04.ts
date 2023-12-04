import { INPUT } from "./04.input.ts";

const EXAMPLE = `
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
`;

function parseCard(l: string) {
  const m = l.match(/Card +(\d+):(.*)\|(.*)/)!;
  const id = parseInt(m[1]);
  const winnings = [...m[2].matchAll(/\d+/g)].map((m) => parseInt(m[0]));
  const owned = [...m[3].matchAll(/\d+/g)].map((m) => parseInt(m[0]));
  return { id, winnings, owned };
}

console.log(
  INPUT.trim()
    .split("\n")
    .reduce((p, c) => {
      const parsed = parseCard(c);
      let points = 0;
      for (const o of parsed.owned) {
        if (parsed.winnings.find((v) => v == o)) {
          points = points ? points * 2 : 1;
        }
      }
      return p + points;
    }, 0)
);

const cache = new Map<number, number>();

function process(a: string[], j: number) {
  let won = cache.get(j);
  if (won === undefined) {
    const parsed = parseCard(a[j]);

    let nb = 0;
    for (const o of parsed.owned) {
      if (parsed.winnings.find((v) => v == o)) {
        nb += 1;
      }
    }

    won = nb;
    for (let k = 1; k <= nb; k++) {
      won += process(a, j + k);
    }

    cache.set(j, won);
  }
  return won;
}

console.log(
  INPUT.trim()
    .split("\n")
    .reduce((p, c, i, a) => p + 1 + process(a, i), 0)
);
