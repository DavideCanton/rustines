import argparse
import os
import sys
import webbrowser
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from threading import Event, Thread
from urllib.parse import urlparse

from common import resolve_web


def parse_args():
    parser = argparse.ArgumentParser(
        description="Start a local development server for the web assets.",
        add_help=False,
    )
    parser.add_argument(
        "-h",
        "--host",
        default="localhost",
        help="Host to bind the server to (default: localhost)",
    )
    parser.add_argument(
        "-p",
        "--port",
        type=int,
        default=8000,
        help="Port to bind the server to (default: 8000)",
    )
    parser.add_argument(
        "-o",
        "--open",
        action="store_true",
        default=False,
        help="Open the server URL in a web browser after starting",
    )
    parser.add_argument(
        "-?",
        "--help",
        action="store_true",
        default=False,
        help="Show this help message and exit",
    )
    return parser.parse_args(), parser


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


def main():
    args, parser = parse_args()

    if args.help:
        parser.print_help()
        sys.exit(0)

    os.chdir(resolve_web() / "dist")

    host = args.host
    port = args.port
    server = ThreadingHTTPServer((host, port), GzipRequestHandler)
    url = f"http://{host}:{port}/"
    print(f"Running server at {url}")

    stop_event = Event()

    def serve():
        server.serve_forever(poll_interval=0.1)

    Thread(target=serve, daemon=True).start()

    if args.open:
        webbrowser.open(url)

    try:
        stop_event.wait()
    except KeyboardInterrupt:
        print("\nStopping server...")
    finally:
        stop_event.set()
        server.shutdown()
        server.server_close()


if __name__ == "__main__":
    main()
