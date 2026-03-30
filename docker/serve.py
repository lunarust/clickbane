# Python 3

import sys
import socketserver
from http.server import SimpleHTTPRequestHandler

class WasmHandler(SimpleHTTPRequestHandler):
    def end_headers(self):
        # Include additional response headers here. CORS for example:
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', '*')
        self.send_header('Access-Control-Allow-Headers', '*')
        self.send_header('Cache-Control', 'no-store, no-cache, must-revalidate')
        SimpleHTTPRequestHandler.end_headers(self)


# Python 3.7.5 adds in the WebAssembly Media Type. If this is an older
# version, add in the Media Type.
if sys.version_info < (3, 7, 5):
    WasmHandler.extensions_map['.wasm'] = 'application/wasm'


if __name__ == '__main__':
    ADDR = '0.0.0.0'
    PORT = 8000
    with socketserver.TCPServer((ADDR, PORT), WasmHandler) as httpd:
        print("Listening on port {}. Press Ctrl+C to stop.".format(PORT))
        httpd.serve_forever()
