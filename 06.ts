import { EXAMPLE, INPUT, Race } from "./06.input.ts";

/**
 * -t^2 + time * t - (distance + 1) = 0
 * @param race
 * @returns [t1, t2] with 0 <= ceil(t1) <= floor(t2) <= time
 */
function resolve(race: Race): [number, number] {
  const delta = race.time ** 2 - 4 * (race.distance + 1);
  const t1 = (race.time - Math.sqrt(delta)) / 2;
  const t2 = (race.time + Math.sqrt(delta)) / 2;
  return [
    Math.max(0, Math.ceil(Math.min(t1, t2))),
    Math.min(Math.floor(Math.max(t1, t2)), race.time),
  ];
}

console.log(
  INPUT.reduce((p, c) => {
    const [t1, t2] = resolve(c);
    return p * (t2 - t1 + 1);
  }, 1)
);

const race2 = {
  time: parseInt(INPUT.reduce((p, c) => p + c.time.toString(), "")),
  distance: parseInt(INPUT.reduce((p, c) => p + c.distance.toString(), "")),
};
const [t1, t2] = resolve(race2);
console.log(t2 - t1 + 1);
