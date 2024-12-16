#!/usr/bin/env python3
from cli import cli

from dataclasses import dataclass
from itertools import takewhile, dropwhile, chain
from typing import Literal
from time import sleep

import numpy as np
from numpy import index_exp, ndarray


type direction = Literal['^', '>', 'v', '<']


@dataclass
class Warehouse:
    map: list[list[direction]]
    robot: ndarray

    def __getitem__(self, vec: ndarray) -> str:
        x, y = vec
        return self.map[y][x]

    def __setitem__(self, key: ndarray, val: str) -> None:
        x, y = key
        self.map[y][x] = val

    def __str__(self) -> str:
        return '\n'.join([''.join(line) for line in self.map])

    @property
    def moves(self) -> dict[direction, ndarray]:
        MOVES: dict[direction, ndarray] = {
            '^': np.array((0, -1)),
            '<': np.array((-1, 0)),
            '>': np.array((1, 0)),
            'v': np.array((0, 1)),
        }
        return MOVES

    def move(
        self,
        direction: direction,
    ) -> None:
        curr_pos = self.robot
        curr_val = self[curr_pos]

        d_vec = self.moves[direction]

        p_i = curr_pos + d_vec
        match self[p_i]:
            case '#':
                return
            case '.':
                self[p_i] = curr_val
                self[curr_pos] = '.'
                self.robot = p_i
            case 'O':
                i = 1
                empty_pos = np.array([])
                while True:
                    next_ = curr_pos + (i * d_vec)
                    i += 1
                    match self[next_]:
                        case '#':
                            return
                        case 'O':
                            continue
                        case '.':
                            empty_pos = next_
                            break

                self[empty_pos], self[p_i], self[curr_pos] = 'O', '@', '.'
                self.robot = p_i

    def score(self) -> int:
        indices = []
        for y, line in enumerate(self.map):
            for x, char in enumerate(line):
                if char == 'O':
                    indices.append((x, y))

        indices = np.array(indices)
        x = indices[:, 0]
        y = indices[:, 1]

        return sum(x + (100 * y))


def main() -> None:
    args = cli()

    with open(args.file, 'r', encoding='utf8') as f:
        lines = [line.strip() for line in f]

    map: list[list[direction]] = []
    robot = np.array((-1, -1))
    for i, line in enumerate(takewhile(bool, lines)):
        map.append(list())
        for j, char in enumerate(line):
            if char == '@':
                robot = np.array((j, i))
            map[-1].append(char) # type: ignore


    warehouse = Warehouse(map, robot)
    moves: list[direction] = list(chain.from_iterable([list(line) for line in dropwhile(bool, lines)][1:])) # type: ignore

    if args.verbose:
        print('\033[2J', end='')
        print('\033[H', end='')
        print(warehouse)
    for move in moves:
        warehouse.move(move)
        if args.verbose:
            print(f'\033[H{warehouse}')
            print(move)
            sleep(0.001)
    print(warehouse.score())

if __name__ == '__main__':
    main()
