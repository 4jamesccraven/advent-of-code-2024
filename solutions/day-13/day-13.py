import re
import argparse
from pathlib import Path
from dataclasses import dataclass
from itertools import batched


@dataclass
class Machine:
    a: tuple[float, ...]
    b: tuple[float, ...]
    price: tuple[float, ...]


    def __str__(self) -> str:
        return (f'Button A: X+{self.a[0]}, Y+{self.a[1]}\n'
                f'Button B: X+{self.b[0]}, Y+{self.b[1]}\n'
                f'Prize: X={self.price[0]}, Y={self.price[1]}')

    def solve(self, offset: int) -> int:
        # https://github.com/jixunmoe/aoc-2024/blob/main/aoc-2024/day-13/solve.py
        (x1, y1), (x2, y2), (xp, yp) = self.a, self.b, self.price

        xp += offset
        yp += offset

        b = (y1 * xp - x1 * yp) / (x2 * y1 - x1 * y2)
        if b.is_integer():
            a = (xp - b * x2) / x1
            if a.is_integer():
                return int(3*a + b)
        return 0

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('file', type=Path)
    args = parser.parse_args()

    with open(args.file, 'r', encoding='utf8') as f:
        input_ = f.read()

    pattern_string = (r'Button A: X\+(\d+), Y\+(\d+)\s*'
                      r'Button B: X\+(\d+), Y\+(\d+)\s*'
                      r'Prize: X=(\d+), Y=(\d+)')
    pat = re.compile(pattern_string, re.M)
    machines: list[Machine] = []
    for match_ in re.finditer(pat, input_):
        vals = map(float, (match_.group(i) for i in range(1, 7)))
        a, b, p = (tuple(val) for val in batched(vals, 2))

        machines.append(Machine(a, b, p))

    ans1 = sum(map(lambda x: x.solve(0), machines))
    ans2 = sum(map(lambda x: x.solve(10_000_000_000_000), machines))
    print(f'Part 1: {ans1}')
    print(f'Part 2: {ans2}')

if __name__ == '__main__':
    main()
