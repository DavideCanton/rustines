import os
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

for p in [Path.cwd(), *Path.cwd().parents]:
    if list(p.glob("web")):
        break
else:
    raise ValueError()

os.chdir(p / "web")
host = "localhost"
port = 8000
server = ThreadingHTTPServer((host, port), SimpleHTTPRequestHandler)
print(f"Running server at http://{host}:{port}/")
try:
    server.serve_forever()
except KeyboardInterrupt:
    print("\nStopping server...")
finally:
    server.server_close()
