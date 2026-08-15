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

    results = {}

    for binary_name in sorted(binary_names):

        print(f"Running benchmark '{binary_name}' ...")
        binary_path = SCRIPT_PATH / 'target' / args.target / 'release' / binary_name

        output = subprocess.run(['docker', 'run', '--rm',
                                 '--mount', f'type=bind,src={binary_path},dst=/algo.firmware',
                                 'ghcr.io/finomnis/qemu-embedded-bench:v0.3.0',
                                 args.machine],
                                check=True,
                                stdout=subprocess.PIPE)

        output = json.loads(output.stdout)
        assert (output['regions_started'] == 1)
        assert (output['regions_completed'] == 1)

        print(f"    Instructions: {output['instructions']: >6}")
        print(f"           Reads: {output['reads']: >6}")
        print(f"          Writes: {output['writes']: >6}")

        results[binary_name] = {
            "instructions": {"value": output['instructions']},
            "reads": {"value": output['reads']},
            "writes": {"value": output['writes']},
        }

    with open(SCRIPT_PATH / "benchmark_results.json", "w") as fil:
        json.dump(results, fil, indent=4, sort_keys=True)


if __name__ == "__main__":
    main()
