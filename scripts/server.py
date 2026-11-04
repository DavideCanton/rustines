import os
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer

from common import resolve_web

os.chdir(resolve_web())
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
