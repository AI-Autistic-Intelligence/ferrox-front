import http.server
import socketserver
import os
import sys

PORT = int(sys.argv[2]) if len(sys.argv) > 2 else int(os.environ.get("PORT", 8089))
DIRECTORY = sys.argv[1] if len(sys.argv) > 1 else "."

class SPAHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    def end_headers(self):
        # Prevent browser caching during active development
        self.send_header('Cache-Control', 'no-cache, no-store, must-revalidate')
        self.send_header('Pragma', 'no-cache')
        self.send_header('Expires', '0')
        super().end_headers()

    def do_GET(self):
        # Resolve path
        path = self.translate_path(self.path)
        # If file doesn't exist and doesn't look like a static asset, serve index.html
        if not os.path.exists(path) and not '.' in os.path.basename(self.path):
            self.path = '/index.html'
        return super().do_GET()

with socketserver.TCPServer(("", PORT), SPAHandler) as httpd:
    print(f"Serving SPA at http://localhost:{PORT} from {DIRECTORY}")
    httpd.serve_forever()
