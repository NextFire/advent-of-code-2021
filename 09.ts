import { EXAMPLE, INPUT } from "./09.input.ts";

const report = INPUT.trim()
  .split("\n")
  .map((l) => l.split(" ").map((v) => parseInt(v)));

function extrapolate(history: number[], left = false): number {
  const sequences = [history];
  do {
    const lastSeq = sequences[sequences.length - 1];
    const newSeq = [];
    for (let i = 1; i < lastSeq.length; i++) {
      newSeq.push(lastSeq[i] - lastSeq[i - 1]);
    }
    sequences.push(newSeq);
  } while (!sequences[sequences.length - 1].every((v) => v === 0));
  const res = sequences.reduceRight(
    (p, c) => (left ? c[0] - p : p + c[c.length - 1]),
    0
  );
  return res;
}

console.log(report.reduce((p, c) => p + extrapolate(c), 0));
console.log(report.reduce((p, c) => p + extrapolate(c, true), 0));
