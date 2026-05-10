from PyQt5.QtWebEngineWidgets import QWebEngineView

class ErrorEngine:
    def __init__(self):
        self.view = QWebEngineView()

    def load(self, url):
        self.view.setUrl(url)

    def get_view(self):
        return self.view
