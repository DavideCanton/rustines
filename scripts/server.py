import os
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import urlparse

from common import resolve_web

os.chdir(resolve_web())


class GzipRequestHandler(SimpleHTTPRequestHandler):
    def guess_type(self, path):
        parsed_path = urlparse(str(path)).path
        if parsed_path.endswith(".wasm.gz"):
            return "application/wasm"
        return super().guess_type(path)

    def end_headers(self):
        path = urlparse(self.path).path
        if path.endswith(".wasm.gz"):
            self.send_header("Content-Encoding", "gzip")
        super().end_headers()


host = "localhost"
port = 8000
server = ThreadingHTTPServer((host, port), GzipRequestHandler)
print(f"Running server at http://{host}:{port}/")
try:
    server.serve_forever()
except KeyboardInterrupt:
    print("\nStopping server...")
finally:
    server.server_close()
