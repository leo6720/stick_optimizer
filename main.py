from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).parent))

from gui import OptimizerApp


if __name__ == "__main__":
    app = OptimizerApp()
    app.mainloop()
