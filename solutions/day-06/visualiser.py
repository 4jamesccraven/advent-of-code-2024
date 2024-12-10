import argparse
from os import get_inheritable
from pathlib import Path
from time import sleep

DIRS = ((0, -1), (1, 0), (0, 1), (-1, 0))

def cli() -> argparse.Namespace:
    parser = argparse.ArgumentParser('visualiser')
    parser.add_argument('file', type=Path)

    return parser.parse_args()

def main() -> None:
    args = cli()

    with open(args.file, 'r', encoding='utf8') as f:
        map_: list[list[str]] = [list(line.strip()) for line in f]

    guard: tuple[int, int] = tuple()

    bounds = len(map_[0]), len(map_)

    for i, line in enumerate(map_):
        for j, char in enumerate(line):
            match char:
                case '^' | '>' | 'v' | '<':
                    guard = (j, i)

    j, i = guard
    direction = -1
    match map_[i][j]:
        case '^':
            direction = 0
        case '>':
            direction = 1
        case 'v':
            direction = 2
        case '<':
            direction = 3

    dir_symbol = {
        0: '^',
        1: '>',
        2: 'v',
        3: '<',
    }

    last_dir = direction
    while True:
        dir = DIRS[direction]
        next_step = guard[0] + dir[0], guard[1] + dir[1]

        conditions = [not (0 <= next_step[i] < bounds[i])
                      for i in range(2)]

        if any(conditions):
            break

        if map_[next_step[1]][next_step[0]] == '#':
            direction = (direction + 1) % 4
            continue
        else:
            new_symbol = ''
            match direction:
                case 0 | 2:
                    new_symbol = '|'
                case 1 | 3:
                    new_symbol = '-'

            if last_dir != direction:
                new_symbol = '+'

            map_[guard[1]][guard[0]] = new_symbol
            map_[next_step[1]][next_step[0]] = dir_symbol[direction]

            last_dir = direction
            guard = next_step

        print('\033[H\033[2J', end='')
        print('\n'.join(map(lambda x: ''.join(x), map_)))

        sleep(0.1)


if __name__ == '__main__':
    main()
