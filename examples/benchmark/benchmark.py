#!/usr/bin/env python3

import subprocess
from pathlib import Path
import json

SCRIPT_PATH = Path(__file__).resolve().parent

MACHINES = [("thumbv6m-none-eabi", "microbit"),
            ("thumbv7em-none-eabi", "mps2-an386")]


def main():

    binary_names = []
    for p in (SCRIPT_PATH / 'src' / 'bin').iterdir():
        if p.is_file():
            print(p.stem)
            binary_names.append(p.stem)

    for (build_target, qemu_machine) in MACHINES:
        subprocess.run(['cargo', 'build', '--release', '--target', build_target],
                       cwd=SCRIPT_PATH, check=True)

        for binary_name in binary_names:

            print(f"Running benchmark '{binary_name}' on '{build_target}'...")
            binary_path = SCRIPT_PATH / 'target' / build_target / 'release' / binary_name

            output = subprocess.run(['docker', 'run', '--rm',
                                     '--mount', f'type=bind,src={binary_path},dst=/algo.firmware',
                                     'ghcr.io/finomnis/qemu-embedded-bench:v0.2.0',
                                     qemu_machine],
                                    check=True)

            output = json.loads(output)
            print(output)
            assert (output['regions_started'] == 1)
            assert (output['regions_completed'] == 1)


if __name__ == "__main__":
    main()
