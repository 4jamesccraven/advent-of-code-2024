from argparse import ArgumentParser, Namespace
from pathlib import Path
import os

def path(raw: str) -> Path:
    r_val = Path(raw)
    if not os.path.isfile(r_val):
        raise ValueError('file does not exist')
    return r_val

def cli() -> Namespace:
    parser = ArgumentParser('15')
    parser.add_argument('file', type=Path)
    parser.add_argument('-v', '--verbose', action='store_true')

    return parser.parse_args()
