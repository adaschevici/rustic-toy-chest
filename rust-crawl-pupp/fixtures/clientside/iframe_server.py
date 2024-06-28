import http.server
import time
import os

class DelayedHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    delay_seconds = 3
    def do_GET(self):
        # Introduce a delay before serving the file
        time.sleep(self.delay_seconds)

        # Check if the file exists
        if os.path.isfile(self.path[1:]):
            super().do_GET()
        else:
            self.send_error(404, "File not found")

def run(server_class=http.server.HTTPServer, handler_class=DelayedHTTPRequestHandler, port=8081):
    server_address = ('', port)
    httpd = server_class(server_address, handler_class)
    print(f"Serving on port {port} with a delay of {handler_class.delay_seconds} seconds")
    httpd.serve_forever()

if __name__ == "__main__":
    run()
