import { EXAMPLE, INPUT } from "./07.input.ts";
import * as _ from "lodash";

type Card = "A" | "K" | "Q" | "J" | "T" | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2;

type HandCards = [Card, Card, Card, Card, Card];

abstract class AbstractHandType {
  abstract strength: number;
  constructor(public handCards: HandCards) {}
}

class FiveOfAKind extends AbstractHandType {
  strength = 6;
}

class FourOfAKind extends AbstractHandType {
  strength = 5;
}

class FullHouse extends AbstractHandType {
  strength = 4;
}

class ThreeOfAKind extends AbstractHandType {
  strength = 3;
}

class TwoPair extends AbstractHandType {
  strength = 2;
}

class OnePair extends AbstractHandType {
  strength = 1;
}

class HighCard extends AbstractHandType {
  strength = 0;
}

type HandType =
  | FiveOfAKind
  | FourOfAKind
  | FullHouse
  | ThreeOfAKind
  | TwoPair
  | OnePair
  | HighCard;

function handTypeSort(
  a: AbstractHandType,
  b: AbstractHandType,
  cardValues: Record<Card, number>
) {
  return (
    a.strength - b.strength ||
    cardValues[a.handCards[0]] - cardValues[b.handCards[0]] ||
    cardValues[a.handCards[1]] - cardValues[b.handCards[1]] ||
    cardValues[a.handCards[2]] - cardValues[b.handCards[2]] ||
    cardValues[a.handCards[3]] - cardValues[b.handCards[3]] ||
    cardValues[a.handCards[4]] - cardValues[b.handCards[4]]
  );
}

interface Hand {
  type: HandType;
  bid: number;
}

function sortedHandCardsMapEntries(
  map: Map<Card, number>,
  cardValues: Record<Card, number>
) {
  const entries = [...map.entries()];
  entries.sort((a, b) => b[1] - a[1] || cardValues[b[0]] - cardValues[a[0]]);
  return entries;
}

function parseInput1(input: string, cardValues: Record<Card, number>): Hand[] {
  return input
    .trim()
    .split("\n")
    .map((l) => {
      const [rawHand, rawBid] = l.split(" ");

      const cards = rawHand.split("") as HandCards;

      const cardsMap = new Map<Card, number>();
      for (const card of cards) {
        cardsMap.set(card, (cardsMap.get(card) ?? 0) + 1);
      }

      const entries = sortedHandCardsMapEntries(cardsMap, cardValues);
      const shape = entries.map((v) => v[1]);

      let type: HandType;
      if (_.isEqual(shape, [5])) {
        type = new FiveOfAKind(cards);
      } else if (_.isEqual(shape, [4, 1])) {
        type = new FourOfAKind(cards);
      } else if (_.isEqual(shape, [3, 2])) {
        type = new FullHouse(cards);
      } else if (_.isEqual(shape, [3, 1, 1])) {
        type = new ThreeOfAKind(cards);
      } else if (_.isEqual(shape, [2, 2, 1])) {
        type = new TwoPair(cards);
      } else if (_.isEqual(shape, [2, 1, 1, 1])) {
        type = new OnePair(cards);
      } else if (_.isEqual(shape, [1, 1, 1, 1, 1])) {
        type = new HighCard(cards);
      } else {
        throw new Error();
      }

      return { type, bid: parseInt(rawBid) };
    });
}

const CardValues1 = {
  A: 12,
  K: 11,
  Q: 10,
  J: 9,
  T: 8,
  9: 7,
  8: 6,
  7: 5,
  6: 4,
  5: 3,
  4: 2,
  3: 1,
  2: 0,
} satisfies Record<Card, number>;

const hands1 = parseInput1(INPUT, CardValues1);
hands1.sort((a, b) => handTypeSort(a.type, b.type, CardValues1));
console.log(hands1.reduce((p, c, i) => p + (i + 1) * c.bid, 0));

function parseInput2(input: string, cardValues: Record<Card, number>): Hand[] {
  return input
    .trim()
    .split("\n")
    .map((l) => {
      const [rawHand, rawBid] = l.split(" ");

      const cards = rawHand.split("") as HandCards;

      const cardsMap = new Map<Card, number>();
      for (const card of cards) {
        if (card === "J") {
          continue;
        }
        cardsMap.set(card, (cardsMap.get(card) ?? 0) + 1);
      }

      const entries = sortedHandCardsMapEntries(cardsMap, cardValues);
      const shape = entries.map((v) => v[1]);

      let type: HandType;
      if (shape.length <= 1) {
        type = new FiveOfAKind(cards);
      } else if (shape.length === 2 && Math.min(...shape) === 1) {
        type = new FourOfAKind(cards);
      } else if (shape.length === 2 && Math.min(...shape) === 2) {
        type = new FullHouse(cards);
      } else if (shape.length === 3 && shape[1] === 1 && shape[2] === 1) {
        type = new ThreeOfAKind(cards);
      } else if (shape.length === 3) {
        type = new TwoPair(cards);
      } else if (shape.length === 4) {
        type = new OnePair(cards);
      } else if (shape.length === 5) {
        type = new HighCard(cards);
      } else {
        throw new Error();
      }

      return { type, bid: parseInt(rawBid) };
    });
}

const CardValues2 = {
  A: 12,
  K: 11,
  Q: 10,
  T: 8,
  9: 7,
  8: 6,
  7: 5,
  6: 4,
  5: 3,
  4: 2,
  3: 1,
  2: 0,
  J: -1,
} satisfies Record<Card, number>;

const hands2 = parseInput2(INPUT, CardValues2);
hands2.sort((a, b) => handTypeSort(a.type, b.type, CardValues2));
console.log(hands2.reduce((p, c, i) => p + (i + 1) * c.bid, 0));
