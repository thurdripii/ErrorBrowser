from PyQt5.QtWebEngineWidgets import QWebEngineView

class ErrorEngine:
    def __init__(self):
        self.browser = QWebEngineView()

    def load(self, url):
        self.browser.setUrl(url)

    def get_view(self):
        return self.browser
