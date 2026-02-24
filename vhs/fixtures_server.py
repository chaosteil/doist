#!/usr/bin/python3
import json
import http.server
import socketserver

PORT = 3000

PREFIX = "data/"

def json_read(filename):
    with open(PREFIX+filename, 'r') as f:
        return json.load(f)

tasks = json_read("tasks.json")
labels = json_read("labels.json")
projects = json_read("projects.json")
sections = json_read("sections.json")

url_map = {
    "/api/v1/tasks": {
        "/": tasks,
        "?filter=%28today+%7C+overdue%29": tasks,
        "/7000003": tasks[3]
    },
    "/api/v1/labels": {
        "/": labels,
    },
    "/api/v1/projects": {
        "/": projects,
    },
    "/api/v1/sections": {
        "/": sections,
    },
}


response = {
    "do the laundry": tasks[0],
    "work out": tasks[1],
    "buy flowers": tasks[2],
    "eat a snack": tasks[3],
}

for k, v in url_map.items():
    items = v["/"]
    for item in items:
        v["/"+item["id"]] = item


def filter_completed():
    for k, v in url_map["/api/v1/tasks"].items():
        if not isinstance(v, list):
            continue
        url_map["/api/v1/tasks"][k] = \
            list(filter(lambda t: t["checked"] == False, tasks))


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
                data = v[suffix]
                if isinstance(data, list):
                    data = {"results": data, "next_cursor": None}
                self.wfile.write(json.dumps(data).encode())
                return
        self.send_response(404)
        self.end_headers()
    def do_POST(self):
        print("POST", self.path)
        length = int(self.headers.get('content-length'))
        field_data = json.loads(str(self.rfile.read(length), "UTF-8"))

        if field_data and field_data.get("content"):
            content = field_data.get("content")
            if response.get(content):
                self.send_response(200)
                self.end_headers()
                self.wfile.write(json.dumps(response[field_data["content"]]).encode())
                return
            elif content == "be lazy":
                tasks[1]["content"] = content

        if self.path.endswith("/7000003/close"):
            tasks[3]["checked"] = True
            filter_completed()

        self.send_response(204)
        self.end_headers()


with socketserver.TCPServer(("", PORT), HTTPHandler) as httpd:
    print("running mock server on port", PORT)
    httpd.serve_forever()
