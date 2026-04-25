import subprocess
import pathlib

ROOT = pathlib.Path(__file__).resolve().parent
RUST_DIR = ROOT.parent / "rust_core"

subprocess.run(
    ["py", "-m", "maturin", "develop"],
    cwd=RUST_DIR,
    check=True
)

print("Rust module rebuilt successfully")