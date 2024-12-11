from argparse import ArgumentParser, Namespace
from pathlib import Path
from time import sleep
import re
from math import floor, log10

import numpy as np


DIRECTIONS = [(0, 1), (0, -1), (1, 0), (-1, 0)]


class MapWrapper:
    def __init__(self, map_: np.ndarray) -> None:
        self.map_ = map_
        self.rows, self.cols = map_.shape
        self.mask = np.zeros(map_.shape, dtype=np.uint8)
        self.highlights = np.zeros(map_.shape, dtype=np.uint8)
        self.delay = (10 ** -(floor(log10(self.rows * self.cols)) + 1))

    def __str__(self) -> str:
        PUT_0_0 = '\033[H\033[2J' # ]]
        RED = '\033[91m'          # ]
        GREEN = '\033[92m'          # ]
        RESET = '\033[0m'         # ]
        ANSI = re.compile(r'\x1b\[([0-9;]*[a-zA-Z])')


        @np.vectorize
        def colourise(val: str, col, masked: bool) -> str:
            return col + re.sub(ANSI, '', val) + RESET if masked else val

        map_str = self.map_.astype(str)
        map_str = colourise(map_str, RED, self.mask)
        map_str = colourise(map_str, GREEN, self.highlights)

        return PUT_0_0 + '\n'.join([''.join(line) for line in map_str])

    def score(self, x: int, y: int, curr_depth: int = 0) -> set[tuple[int, int]]:
        sleep(self.delay)
        vals: set[tuple[int, int]] = set()

        self.mask[x][y] = 1
        print(self)

        if self.map_[x][y] == 9 and curr_depth == 9:
            self.highlights[x][y] = 1
            vals.add((x, y))
            return vals
        elif self.map_[x][y] != curr_depth:
            return vals
        else:
            for dir in DIRECTIONS:
                next_ = (dir[0] + x, dir[1] + y)
                if (0 <= next_[0] < self.rows) and (0 <= next_[1] < self.cols):
                    vals |= (self.score(next_[0], next_[1], curr_depth + 1))

            if curr_depth == 0:
                self.mask = np.zeros(self.map_.shape, dtype=np.uint8)

            return vals


def cli() -> Namespace:
    parser = ArgumentParser()
    parser.add_argument('file', type=Path)

    return parser.parse_args()


def main() -> None:
    args = cli()

    with open(args.file, 'r', encoding='utf8') as f:
        map_ = MapWrapper(np.array([list(map(int, line.strip())) for line in f]))

    try:
        total = 0
        for i, line in enumerate(map_.map_):
            for j, _ in enumerate(line):
                total += len(map_.score(i, j))
        map_.mask = np.zeros(map_.map_.shape, dtype=np.uint8)
        print(map_)

        print(f'\nTotal Score: {total}')
    except KeyboardInterrupt:
        print('Terminating...')



if __name__ == '__main__':
    main()
