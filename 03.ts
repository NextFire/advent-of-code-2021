import { INPUT } from "./03.input.ts";

const EXAMPLE = `
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
`;

console.log(
  INPUT.trim()
    .split("\n")
    .reduce((p, c, i, a) => {
      for (const m of c.matchAll(/[^\d\.]/g)) {
        const j = m.index!;
        if (i > 0) {
          for (const sm of a[i - 1].matchAll(/\d+/g)) {
            if (sm.index! <= j + 1 && sm.index! + sm[0].length - 1 >= j - 1) {
              p += parseInt(sm[0]);
            }
          }
        }
        for (const sm of c.matchAll(/\d+/g)) {
          if (sm.index! <= j + 1 && sm.index! + sm[0].length - 1 >= j - 1) {
            p += parseInt(sm[0]);
          }
        }
        if (i < a.length - 1) {
          for (const sm of a[i + 1].matchAll(/\d+/g)) {
            if (sm.index! <= j + 1 && sm.index! + sm[0].length - 1 >= j - 1) {
              p += parseInt(sm[0]);
            }
          }
        }
      }
      return p;
    }, 0)
);

console.log(
  INPUT.trim()
    .split("\n")
    .reduce((p, c, i, a) => {
      for (const m of c.matchAll(/\*/g)) {
        const j = m.index!;
        let ratio = 1;
        let nbd = 0;
        if (i > 0) {
          for (const sm of a[i - 1].matchAll(/\d+/g)) {
            if (sm.index! <= j + 1 && sm.index! + sm[0].length - 1 >= j - 1) {
              ratio *= parseInt(sm[0]);
              nbd += 1;
            }
          }
        }
        for (const sm of c.matchAll(/\d+/g)) {
          if (sm.index! <= j + 1 && sm.index! + sm[0].length - 1 >= j - 1) {
            ratio *= parseInt(sm[0]);
            nbd += 1;
          }
        }
        if (i < a.length - 1) {
          for (const sm of a[i + 1].matchAll(/\d+/g)) {
            if (sm.index! <= j + 1 && sm.index! + sm[0].length - 1 >= j - 1) {
              ratio *= parseInt(sm[0]);
              nbd += 1;
            }
          }
        }
        if (nbd == 2) {
          p += ratio;
        }
      }
      return p;
    }, 0)
);
