from pathlib import Path


def resolve_web() -> Path:
    for p in [Path.cwd(), *Path.cwd().parents]:
        if list(p.glob("web")):
            return p / "web"

    raise ValueError()
