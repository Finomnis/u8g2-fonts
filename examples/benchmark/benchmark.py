#!/usr/bin/env python3

import subprocess
from pathlib import Path
import json
from argparse import ArgumentParser

SCRIPT_PATH = Path(__file__).resolve().parent


def main():
    parser = ArgumentParser()
    parser.add_argument('target')
    parser.add_argument('machine')
    args = parser.parse_args()

    binary_names = []
    for p in (SCRIPT_PATH / 'src' / 'bin').iterdir():
        if p.is_file():
            print(p.stem)
            binary_names.append(p.stem)

    subprocess.run(['cargo', 'build', '--release', '--target', args.target],
                   cwd=SCRIPT_PATH, check=True)

    for binary_name in binary_names:

        print(f"Running benchmark '{binary_name}' ...")
        binary_path = SCRIPT_PATH / 'target' / args.target / 'release' / binary_name

        output = subprocess.run(['docker', 'run', '--rm',
                                 '--mount', f'type=bind,src={binary_path},dst=/algo.firmware',
                                 'ghcr.io/finomnis/qemu-embedded-bench:v0.3.0',
                                 args.machine],
                                check=True,
                                stdout=subprocess.PIPE)

        output = json.loads(output.stdout)
        print(output)
        assert (output['regions_started'] == 1)
        assert (output['regions_completed'] == 1)


if __name__ == "__main__":
    main()
