#!/usr/bin/env python3

from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import logging

def bin2text(s):
    return "".join([unichr(int(s[i:i+8], 2)) for i in range(0, len(s), 8)])

def bin2utf(text):
    utf = ""
    for c in text:
        #print("{} {} 0x{:x}".format(chr(c), c, c))
        utf += chr(c)
    return utf


class MyHandler(BaseHTTPRequestHandler):
    def _set_response(self):

        self.send_response(200)

        self.send_header('Content-Type', 'text/html')
        self.end_headers()

        self._write("<head><body>")
        (client, port) = self.client_address
        self._write("Client: {}:{}<br>".format(str(client), str(port)))
        self._write("Command: {}<br>".format(str(self.command)))
        self._write("Path: {}<br>".format(str(self.path)))
        self._write("Version: {}<br>".format(str(self.request_version)))
        self._write("Headers: {}<br>".format(self.headers.as_string()))
        #for k, v in self.headers:
        #    self._write("* {}: {}".format(k, v))
        self._write("</body></head>")

    def _write(self, text):
        self.wfile.write(text.encode('utf-8'))
        logging.info("response: {}".format(text))

    def do_GET(self):
        logging.info("GET request")
        logging.info("Path: %s", str(self.path))
        logging.info("Headers: %s\n", str(self.headers))
        self._set_response()

    def do_POST(self):
        content_length = int(self.headers['Content-Length'])
        body = self.rfile.read(content_length)
        logging.info("POST request")
        logging.info("Path: %s", str(self.path))
        logging.info("Headers: %s\n", str(self.headers))
        bin2utf(body)
        logging.info("Body: %s", bin2utf(body))
        self._set_response()

def main(port=8000):
    logging.basicConfig(level=logging.INFO)
    logging.info("Hi!")

    server_address = ('', port)
    logging.info("Listening: %s\n", server_address)
    httpd = ThreadingHTTPServer(server_address, MyHandler)
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        httpd.server_close()
    logging.info("Ok, byeeee!")

if __name__ == '__main__':
    main()
