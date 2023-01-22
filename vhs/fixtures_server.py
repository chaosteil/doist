#!/usr/bin/python3
import http.server
import socketserver

PORT = 3000

PREFIX = "../tests/commands/fixtures/"

url_map = {
    # TODO: do specific task ids etc
    "/rest/v2/tasks": "tasks.json",
    "/rest/v2/labels": "labels.json",
    "/rest/v2/projects": "projects.json",
    "/rest/v2/sections": "sections.json",
}
url_map = {k: open(PREFIX+v).read() for k, v in url_map.items()}


class HTTPHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        print("GET", self.path)
        for k, v in url_map.items():
            if self.path.startswith(k):
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.end_headers()
                self.wfile.write(v.encode())
                return
        self.send_response(404)
        self.end_headers()
    def do_POST(self):
        print("POST", self.path)
        self.send_response(204)
        self.end_headers()


with socketserver.TCPServer(("", PORT), HTTPHandler) as httpd:
    print("running mock server on port", PORT)
    httpd.serve_forever()
