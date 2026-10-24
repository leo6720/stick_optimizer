"""Capture the actual Glow viewport; local QA only, no user data or network.

Requires the existing ui-preview build, Xvfb DISPLAY, and Pillow.
Run from repository root. Each viewport capture closes its own process.
"""
import hashlib
import json
import os
from pathlib import Path
import subprocess

from PIL import Image

ROOT = Path(__file__).resolve().parents[3]
QA = ROOT / "docs/qa/leo23"
BIN = Path(os.environ.get("CARGO_TARGET_DIR", "/workspace/stick-rust-validation/target")) / "debug/stick-optimizer-desktop"
SOURCE = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
records = []
cases = [(mode, size, scale, "default")
         for mode in ("results", "pockets", "heads")
         for size, scale in (("1450x900", "1"), ("760x620", "1"),
                             ("720x560", "1"), ("1450x900", "1.5"))]
cases += [("results", "1450x900", "1", fixture) for fixture in ("dense", "long")]
for mode, size, scale, fixture in cases:
    for theme in ("light", "dark"):
        name = f"{mode}-{size}-z{scale}-{fixture}-{theme}"
        path = QA / "screenshots" / f"{name}.png"
        env = dict(os.environ, LIBGL_ALWAYS_SOFTWARE="1",
                   STICK_OPTIMIZER_PREVIEW=mode, STICK_OPTIMIZER_PREVIEW_THEME=theme,
                   STICK_OPTIMIZER_PREVIEW_SIZE=size, STICK_OPTIMIZER_PREVIEW_SCALE=scale,
                   STICK_OPTIMIZER_PREVIEW_RESULTS_FIXTURE=fixture,
                   STICK_OPTIMIZER_SCREENSHOT_TO=str(path))
        run = subprocess.run([str(BIN)], cwd=ROOT, env=env, capture_output=True,
                             text=True, timeout=50)
        (QA / "logs" / f"{name}.log").write_text(run.stdout + run.stderr)
        if run.returncode or not path.exists():
            raise SystemExit(f"{name}: exit={run.returncode}, {run.stderr}")
        with Image.open(path) as img:
            dimensions = list(img.size)
            rgba_hash = hashlib.sha256(img.convert("RGBA").tobytes()).hexdigest()
        records.append(dict(path=str(path.relative_to(ROOT)), source_sha=SOURCE,
                            mode=mode, requested_size=size, zoom=scale, theme=theme,
                            fixture=fixture, exit=run.returncode, dimensions=dimensions,
                            png_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),
                            rgba_sha256=rgba_hash, renderer="eframe Glow / X11 / software GL"))
        (QA / "manifest.json").write_text(json.dumps(records, indent=2) + "\n")
        print(name, dimensions, "PASS", flush=True)
