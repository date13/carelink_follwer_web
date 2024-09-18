#!/usr/bin/env python3

import requests
from requests.adapters import HTTPAdapter

HTTP_TIMEOUT = 60
HTTP_MAX_RETRIES = 3


class HttpUtils:

    def __init__(self):
        self.session = requests.Session()
        adapter = HTTPAdapter(max_retries=HTTP_MAX_RETRIES)
        # session.keep_alive = False
        self.session.mount('http://', adapter)
        self.session.mount('https://', adapter)

    def getSession(self):
        return self.session
