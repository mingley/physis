"""Compare paired release builds on identical inputs. Uses the Python standard library."""
import argparse
import hashlib
import os
from pathlib import Path
import statistics
import subprocess
import tempfile
import time


def output(*args):
    return subprocess.run(args, check=True, stdout=subprocess.PIPE).stdout


def bench(binary):
    samples = {}
    counts = {}
    for row in output(binary).decode().splitlines():
        name, decisions, _, ns, _, checksum = row.split(maxsplit=5)
        samples[name] = float(ns)
        counts[name] = (decisions, checksum)
    return samples, counts


def grids(binary, journals):
    weights = ",".join(map(str, range(1, 9)))
    digest = hashlib.sha256()
    start = time.perf_counter_ns()
    for journal in journals:
        digest.update(output(binary, "--journal", journal, "--json", "conditions",
                             "excursion-ring", "excursion.beyond-joint-bound",
                             f"clockwise={weights}", f"anticlockwise={weights}"))
    return (time.perf_counter_ns() - start) / 1e6, digest.hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ["base_lab", "new_lab", "base_bench", "new_bench"]:
        parser.add_argument(name, type=Path)
    parser.add_argument("--trials", type=int, default=7)
    args = parser.parse_args()
    if args.trials < 1:
        parser.error("--trials must be positive")
    if hasattr(os, "sched_setaffinity"):
        os.sched_setaffinity(0, {min(os.sched_getaffinity(0))})
    builds = [(args.base_lab.resolve(), args.base_bench.resolve()),
              (args.new_lab.resolve(), args.new_bench.resolve())]
    results = [{}, {}]
    reference = None
    with tempfile.TemporaryDirectory() as tmp:
        journals = [Path(tmp) / f"{i}.jsonl" for i in range(1, 9)]
        for i, journal in enumerate(journals, start=1):
            output(builds[0][0], "--journal", journal, "set", "excursion-ring", "opposite", str(i))
        for trial in range(args.trials + 1):
            for i in ([0, 1] if trial % 2 == 0 else [1, 0]):
                lab, micro = builds[i]
                samples, counts = bench(micro)
                samples["eight-grids-ms"], digest = grids(lab, journals)
                if reference is None:
                    reference = (counts, digest)
                if (counts, digest) != reference:
                    raise ValueError("the classifier totals or complete factory grid output differ")
                if trial:  # discard the initial warm-up
                    for key, value in samples.items():
                        results[i].setdefault(key, []).append(value)
    print("class totals and grid SHA-256", reference)
    for key in results[0]:
        old, new = (statistics.median(row[key]) for row in results)
        print(f"{key} base {old:.2f} new {new:.2f} speedup {old/new:.2f}x")
        print("  base trials", results[0][key])
        print("  new trials ", results[1][key])


if __name__ == "__main__":
    main()
