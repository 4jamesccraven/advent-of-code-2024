import re
from functools import reduce
from operator import mul

def main() -> None:
    with open('input.in', 'r', encoding='utf8') as f:
        expression = f.read()

    ans1 = 0
    ans2 = 0
    do = True
    for match in re.finditer(r'mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don\'t\(\)', expression):
        match match.group():
            case 'do()':
                do = True
            case 'don\'t()':
                do = False
            case _:
                res = reduce(mul, map(int, (match.group(1), match.group(2))))
                ans1 += res
                if do: ans2 += res

    print(f'Part 1: {ans1}')
    print(f'Part 2: {ans2}')


if __name__ == '__main__':
    main()
