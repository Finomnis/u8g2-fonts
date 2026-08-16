#!/usr/bin/env python3

import subprocess
from pathlib import Path
from elftools.elf.elffile import ELFFile, Segment, SymbolTableSection
from argparse import ArgumentParser
from tqdm import tqdm
import capstone as cs
from collections import defaultdict

SCRIPT_PATH = Path(__file__).resolve().parent


def demangle_rust(name: str) -> str:
    result = subprocess.run(
        ["rustfilt", name],
        capture_output=True,
        text=True,
        check=True,
    )
    return result.stdout.strip()


def resolve_fnname(elf: ELFFile, addr: int):
    for section in elf.iter_sections():
        if not isinstance(section, SymbolTableSection):
            continue

        for sym in section.iter_symbols():
            if sym["st_info"]["type"] != "STT_FUNC":
                continue

            start = sym["st_value"] & ~1
            size = sym["st_size"]

            if size:
                if start <= addr < start + size:
                    return demangle_rust(sym.name)
            elif addr == start:
                return demangle_rust(sym.name)

    return f"0x{addr:x}"


class CodeSegment:
    def __init__(self, seg: Segment):
        self.data = seg.data()
        self.start = seg["p_vaddr"]
        self.end = self.start + seg["p_filesz"]
        print(f"{self.start:x}, {self.end:x}")

        self.cache = {}

    def contains(self, addr: int):
        return self.start <= addr < self.end

    def get_instr(self, addr: int):
        if not self.contains(addr):
            return None

        if addr in self.cache:
            return self.cache[addr]

        offset = addr-self.start

        md = cs.Cs(cs.CS_ARCH_ARM, cs.CS_MODE_THUMB | cs.CS_MODE_LITTLE_ENDIAN)
        md.detail = True
        instr = next(md.disasm(self.data[offset:offset+4], addr, count=1))

        self.cache[addr] = instr

        return instr


class CodeSegments:
    def __init__(self, elf: ELFFile):
        self.segments = []

        for seg in elf.iter_segments():
            if seg["p_type"] != "PT_LOAD":
                continue

            # PF_X = executable
            if not (seg["p_flags"] & 0x1):
                continue

            self.segments.append(CodeSegment(seg))

    def get_instr(self, addr: int):
        for seg in self.segments:
            if seg.contains(addr):
                return seg.get_instr(addr)
        return None


class ParsedLine:
    def __init__(self, line: str, segments: CodeSegments):
        self.raw = line
        self.is_read = False
        self.is_write = False
        self.instr = None

        if line == b'r':
            self.is_read = True
        elif line == b'w':
            self.is_write = True
        else:
            address = int(self.raw, 16)

            self.instr = segments.get_instr(address)
            if self.instr is None:
                raise ValueError(
                    f"PC 0x{self.addr:016x} is not inside "
                    "an executable ELF segment"
                )

    def __str__(self):
        if self.is_read:
            return 'read'
        if self.is_write:
            return 'write'
        return f"{self.instr}"


def create_section_str(current_section):
    return ';'.join([s[0] for s in current_section])


def render_flamegraph(filepath, data):
    folded_file = "\n".join([f"{key} {value}" for (
        key, value) in data.items()]).encode()

    svg_content = subprocess.run(
        [SCRIPT_PATH / "flamegraph.pl"],
        input=folded_file,
        check=True,
        stdout=subprocess.PIPE,
    ).stdout

    with open(filepath, 'wb') as fil:
        fil.write(svg_content)


def main():
    parser = ArgumentParser()
    parser.add_argument('target')
    parser.add_argument('machine')
    parser.add_argument('binary')
    args = parser.parse_args()

    subprocess.run(['cargo', 'build', '--release', '--target', args.target],
                   cwd=SCRIPT_PATH, check=True)

    binary_name = args.binary

    print(f"Running benchmark '{binary_name}' ...")
    binary_path = SCRIPT_PATH / 'target' / args.target / 'release' / binary_name

    output = subprocess.run(['docker', 'run', '--rm',
                             '--mount', f'type=bind,src={binary_path},dst=/algo.firmware',
                             'ghcr.io/finomnis/qemu-embedded-bench:v0.4.0',
                             args.machine,
                             '--trace'],
                            check=True,
                            stdout=subprocess.PIPE)

    with open(binary_path, 'rb') as elf_filehandle:
        elf = ELFFile(elf_filehandle)
        code_segments = CodeSegments(elf)

        lines = [ParsedLine(line, code_segments)
                 for line in output.stdout.splitlines()]

        # Generate flamegraphs
        current_section_stack = []
        entering_fn = None
        current_section_str = create_section_str(current_section_stack)

        flamegraph_instrs = defaultdict(int)
        flamegraph_reads = defaultdict(int)
        flamegraph_writes = defaultdict(int)

        for line in tqdm(lines):
            if line.instr is not None:
                instr = line.instr
                if len(current_section_stack) != 0 and current_section_stack[-1][1] == instr.address:
                    current_section_stack.pop()
                if entering_fn is not None:
                    current_section_stack.append(
                        (resolve_fnname(elf, instr.address), entering_fn))
                    entering_fn = None
                    current_section_str = create_section_str(
                        current_section_stack)
                if instr.group(cs.CS_GRP_CALL):
                    entering_fn = instr.address + instr.size

                flamegraph_instrs[current_section_str] += 1

            if line.is_write:
                flamegraph_writes[current_section_str] += 1

            if line.is_read:
                flamegraph_reads[current_section_str] += 1

    render_flamegraph(SCRIPT_PATH / "flamegraph_instrs.svg", flamegraph_instrs)
    render_flamegraph(SCRIPT_PATH / "flamegraph_reads.svg", flamegraph_reads)
    render_flamegraph(SCRIPT_PATH / "flamegraph_writes.svg", flamegraph_writes)


if __name__ == "__main__":
    main()
