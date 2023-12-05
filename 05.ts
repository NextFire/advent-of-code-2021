import { EXAMPLE, INPUT } from "./05.input.ts";

type Category =
  | "seed"
  | "soil"
  | "fertilizer"
  | "water"
  | "light"
  | "temperature"
  | "humidity"
  | "location";

interface AlmanacMapRange {
  sourceStart: number;
  destStart: number;
  length: number;
}

class AlmanacMap {
  constructor(
    public source: Category,
    public dest: Category,
    public ranges: AlmanacMapRange[]
  ) {}

  convert(i: number): number {
    for (const range of this.ranges) {
      if (range.sourceStart <= i && i < range.sourceStart + range.length) {
        return range.destStart + i - range.sourceStart;
      }
    }
    return i;
  }

  inverse(i: number): number {
    for (const range of this.ranges) {
      if (range.destStart <= i && i < range.destStart + range.length) {
        return range.sourceStart + i - range.destStart;
      }
    }
    return i;
  }

  static parse(txt: string): AlmanacMap {
    const lines = txt.trim().split("\n");
    const m = lines[0].match(/(.*)-to-(.*) /)!;
    const source = m[1] as Category;
    const dest = m[2] as Category;

    const ranges: AlmanacMapRange[] = lines.slice(1).map((l) => {
      const m = l.match(/(\d+) +(\d+) +(\d+)/)!;
      return {
        sourceStart: parseInt(m[2]),
        destStart: parseInt(m[1]),
        length: parseInt(m[3]),
      };
    });

    return new this(source, dest, ranges);
  }
}

class ParsedInput {
  constructor(
    public seeds: number[],
    public maps: Map<Category, AlmanacMap>,
    public inverseMaps: Map<Category, AlmanacMap>
  ) {}

  convertRecursive(i: number, current: Category = "seed"): number {
    const map = this.maps.get(current);
    return map ? this.convertRecursive(map.convert(i), map.dest) : i;
  }

  inverseRecursive(i: number, current: Category = "location"): number {
    const map = this.inverseMaps.get(current);
    return map ? this.inverseRecursive(map.inverse(i), map.source) : i;
  }

  static parse(input: string): ParsedInput {
    const blocks = input.trim().split("\n\n");

    const seeds: number[] = [];
    for (const m of blocks[0].matchAll(/\d+/g)) {
      seeds.push(parseInt(m[0]));
    }

    const mapsArray = blocks.slice(1).map((b) => AlmanacMap.parse(b));
    const maps = new Map(mapsArray.map((m) => [m.source, m]));
    const inverseMaps = new Map(mapsArray.map((m) => [m.dest, m]));

    return new this(seeds, maps, inverseMaps);
  }
}

const parsed = ParsedInput.parse(INPUT);
const converted = parsed.seeds.map((s) => parsed.convertRecursive(s));
console.log(Math.min(...converted));

function part2(parsed: ParsedInput) {
  for (let i = 0; ; i++) {
    const seed = parsed.inverseRecursive(i);
    for (let j = 0; j < parsed.seeds.length; j += 2) {
      if (
        parsed.seeds[j] <= seed &&
        seed < parsed.seeds[j] + parsed.seeds[j + 1]
      ) {
        return i;
      }
    }
  }
}
console.log(part2(parsed));
