import http.server
import socketserver
import os
import sys

PORT = int(sys.argv[2]) if len(sys.argv) > 2 else int(os.environ.get("PORT", 4000))
DIRECTORY = sys.argv[1] if len(sys.argv) > 1 else "dist"

class SPAHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    extensions_map = http.server.SimpleHTTPRequestHandler.extensions_map.copy()
    extensions_map['.wasm'] = 'application/wasm'
    extensions_map['.js'] = 'application/javascript'
    extensions_map['.css'] = 'text/css'

    def end_headers(self):
        self.send_header('Cache-Control', 'no-cache, no-store, must-revalidate')
        self.send_header('Pragma', 'no-cache')
        self.send_header('Expires', '0')
        self.send_header('Access-Control-Allow-Origin', '*')
        super().end_headers()

    def do_GET(self):
        path = self.translate_path(self.path)
        if not os.path.exists(path) and not '.' in os.path.basename(self.path):
            self.path = '/index.html'
        return super().do_GET()

with socketserver.TCPServer(("", PORT), SPAHandler) as httpd:
    print(f"Serving Ferrox Admin Dashboard at http://localhost:{PORT} from {DIRECTORY}")
    httpd.serve_forever()
