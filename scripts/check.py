"""Run the documented Rust checks from any directory (Python 3.10+)."""
import argparse
from pathlib import Path
import shlex
import subprocess
import sys


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--scope", choices=("workspace", "core"), default="workspace")
    parser.add_argument("--plan", action="store_true", help="Print commands without running them")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    if not (root / "Cargo.toml").is_file():
        parser.error("Cargo.toml not found beside scripts/")

    selection = ["--workspace"] if args.scope == "workspace" else [
        "-p", "stick-optimizer-core", "-p", "stick-optimizer-io"
    ]
    commands = [
        ["cargo", "fmt", "--all", "--", "--check"],
        ["cargo", "check", *selection, "--all-targets", "--all-features", "--locked"],
        ["cargo", "clippy", *selection, "--all-targets", "--all-features", "--locked", "--", "-D", "warnings"],
        ["cargo", "test", *selection, "--all-features", "--locked"],
        ["cargo", "build", *selection, "--release", "--locked"],
    ]
    for command in commands:
        print("+ " + shlex.join(command), flush=True)
        if args.plan:
            continue
        try:
            result = subprocess.run(command, cwd=root, check=False)
        except OSError as error:
            print(f"Cannot run cargo: {error}", file=sys.stderr)
            return 1
        if result.returncode:
            return result.returncode if result.returncode > 0 else 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
