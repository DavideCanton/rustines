import os
from ftplib import FTP
from pathlib import PurePosixPath

import dotenv
import keyring
from common import resolve_web

EXT = {
    ".html": "t",
    ".js": "t",
    ".css": "t",
    ".wasm": "b",
    ".gz": "b",
}
ROOT = PurePosixPath("/")

dotenv.load_dotenv()

ftp_user = os.environ["FTP_USER"].strip()
ftp_host = os.environ["FTP_HOST"].strip()

ftp_password = keyring.get_password(ftp_host, ftp_user)
assert ftp_password

root = resolve_web().resolve()

with FTP(host=ftp_host, user=ftp_user, passwd=ftp_password) as ftp:
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
