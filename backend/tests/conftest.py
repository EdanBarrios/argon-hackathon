import sys
from pathlib import Path

# Make `api` importable when running `uv run pytest` from backend/.
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
