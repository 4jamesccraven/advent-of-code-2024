from itertools import starmap
from operator import sub
from collections import Counter, defaultdict


def main() -> None:
    file = './input.in'

    with open(file, 'r', encoding='utf8') as f:
        text_raw = [tuple(map(int, line.strip().split())) for line in f]

    list1, list2, = map(list, zip(*text_raw))

    list1.sort()
    list2.sort()

    ans = sum(map(abs, starmap(sub, zip(list1, list2))))
    print(f'Part 1: {ans}')

    counts = defaultdict(int) | dict(Counter(list2))

    ans = sum(map(lambda x: x * counts[x], list1))
    print(f'Part 2: {ans}')


if __name__ == '__main__':
    main()
