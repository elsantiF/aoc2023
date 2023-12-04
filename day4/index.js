import { readFileSync } from 'fs';

const part1 = () => {
    const FILE = readFileSync('./part1.txt', 'utf-8');

    const cardGames = FILE.split(/\r?\n/).map((line) => line.split(':'));
    let games = cardGames.map((game) => game[1].split(' | '))

    let sum = 0;

    for(let game of games) {
        let win = game[0].trim().split(/\s+/);
        let ticket = game[1].trim().split(/\s+/);
        console.log(`${win} | ${ticket}`)
        let points = 0;

        for(let number of ticket) {
            if(win.includes(number)) {
                (points == 0) ? points = 1 : points *= 2
                
            }
        }
        sum += points
    }
    console.log(sum)
}

part1();