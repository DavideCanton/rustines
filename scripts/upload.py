import argparse
import os
import subprocess
from ftplib import FTP  # pyright: ignore
from pathlib import PurePosixPath

from common import resolve_web

try:
    import dotenv

    dotenv.load_dotenv()
except ImportError:
    pass


def parse_args():
    parser = argparse.ArgumentParser(
        description="Upload web assets to an FTP server.",
    )
    parser.add_argument(
        "-n",
        "--dry-run",
        action="store_true",
        default=False,
        help="Perform a dry run without actually uploading anything",
    )
    parser.add_argument(
        "--allow-dirty",
        action="store_true",
        default=False,
        help="Skip the dirty workspace check",
    )
    return parser.parse_args()


class DryRunFTP:
    """Mock FTP client for dry run mode."""

    def __init__(self, *a, **kw):
        pass

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc, tb):
        pass

    def getwelcome(self):
        pass

    def storlines(self, *a, **kw):
        pass

    def storbinary(self, *a, **kw):
        pass


def get_password(ftp_host: str, ftp_user: str) -> str:
    ftp_password = os.environ.get("FTP_PASSWORD")
    if not ftp_password:
        import keyring

        ftp_password = keyring.get_password(ftp_host, ftp_user)
    assert ftp_password
    return ftp_password


def check_clean_workspace(allow_dirty=False):
    if allow_dirty:
        return

    result = subprocess.run(
        ["git", "status", "--porcelain"],
        capture_output=True,
        text=True,
        check=True,
    )
    if result.stdout.strip():
        print("ERROR: Workspace is dirty. Pending changes detected:")
        print(result.stdout)
        raise SystemExit(1)


def main():
    args = parse_args()

    if args.dry_run:
        ftp = DryRunFTP()
    else:
        ftp_user = os.environ["FTP_USER"].strip()
        ftp_host = os.environ["FTP_HOST"].strip()
        ftp_password = get_password(ftp_host, ftp_user)
        ftp = FTP(host=ftp_host, user=ftp_user, passwd=ftp_password)

    check_clean_workspace(allow_dirty=args.allow_dirty)

    EXT = {
        ".html": "t",
        ".js": "t",
        ".css": "t",
        ".wasm": "b",
        ".gz": "b",
    }
    ROOT = PurePosixPath("/")

    root = (resolve_web() / "dist").resolve()

    with ftp:
        print(ftp.getwelcome())

        for cur, _, files in root.walk():
            cwd = ROOT / "rustines" / cur.resolve().relative_to(root)

            for f in files:
                rf = cwd / f
                mode = EXT.get(rf.suffix)
                if mode is None:
                    continue

                print(f"{rf}...", end="")

                match mode:
                    case "t":
                        with cur.joinpath(f).open("rb") as f:
                            ftp.storlines(f"STOR {rf}", f)
                    case "b":
                        with cur.joinpath(f).open("rb") as f:
                            ftp.storbinary(f"STOR {rf}", f)

                print("end!")


if __name__ == "__main__":
    main()
