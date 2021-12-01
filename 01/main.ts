import { INPUT } from './input.ts';

const numbers = INPUT.split('\n').map(Number);

const respOne = numbers.reduce((acc, curr, i) => acc + Number(curr > numbers[i - 1]), 0);
console.log(respOne);

const respTwo = numbers.reduce((acc, _curr, i) => {
    let inc = 0;
    if (1 <= i && i <= numbers.length - 3) {
        const prevSlice = numbers.slice(i - 2, i + 1).reduce((a, b) => a + b, 0);
        const currSlice = numbers.slice(i - 1, i + 2).reduce((a, b) => a + b, 0);
        if (currSlice > prevSlice) inc = 1;
    }
    return acc + inc
}, 0);
console.log(respTwo);
