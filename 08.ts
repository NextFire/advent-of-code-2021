import { CamelMap, EXAMPLE1, EXAMPLE2, INPUT } from "./08.input.ts";

function readDirections(directions: [string, string], chosen: "L" | "R") {
  return directions[chosen === "L" ? 0 : 1];
}

function readMap1(map: CamelMap) {
  let steps = 0;
  let curr = "AAA";
  while (true) {
    if (curr === "ZZZ") {
      return steps;
    }
    curr = readDirections(
      map.network[curr],
      map.directions[steps++ % map.directions.length]
    );
  }
}
console.log(readMap1(INPUT));

function readMap2(map: CamelMap) {
  let steps = 0;
  let currs = Object.keys(map.network).filter((v) => v.endsWith("A"));
  const cycles = [];
  while (true) {
    const filtered = currs.filter((v) => !v.endsWith("Z"));
    if (filtered.length != currs.length) {
      cycles.push(steps);
      currs = filtered;
    }
    if (currs.length === 0) {
      return cycles;
    }
    currs = currs.map((v) =>
      readDirections(
        map.network[v],
        map.directions[steps % map.directions.length]
      )
    );
    steps++;
  }
}
console.log(readMap2(INPUT)); // LCM it
