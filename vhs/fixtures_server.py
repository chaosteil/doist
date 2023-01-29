#!/usr/bin/python3
import json
import http.server
import socketserver

PORT = 3000

PREFIX = "../tests/commands/fixtures/"

def json_read(filename):
    with open(PREFIX+filename, 'r') as f:
        return json.load(f)

tasks = json_read("tasks.json")
tasks_partial = json_read("tasks_partial.json")
labels = json_read("labels.json")
projects = json_read("projects.json")
sections = json_read("sections.json")

url_map = {
    "/rest/v2/tasks": {
        "/": tasks,
        "?filter=%28today+%7C+overdue%29": tasks_partial,
    },
    "/rest/v2/labels": {
        "/": labels,
    },
    "/rest/v2/projects": {
        "/": projects,
    },
    "/rest/v2/sections": {
        "/": sections,
    },
}

for k, v in url_map.items():
    items = v["/"]
    for item in items:
        v["/"+item["id"]] = item

class HTTPHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        print("GET", self.path)
        for k, v in url_map.items():
            if self.path.startswith(k):
                suffix = self.path[len(k):]
                suffix = "/" if suffix == "" else suffix
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.end_headers()
                self.wfile.write(json.dumps(v[suffix]).encode())
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
