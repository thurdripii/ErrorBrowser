from PyQt5.QtWidgets import QMainWindow, QLineEdit, QVBoxLayout, QWidget
from engine.yeetcore.engine import ErrorEngine

class BrowserUI(QMainWindow):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Error Browser")

        self.engine = ErrorEngine()

        self.url_bar = QLineEdit()
        self.url_bar.returnPressed.connect(self.load_url)

        layout = QVBoxLayout()
        layout.addWidget(self.url_bar)
        layout.addWidget(self.engine.get_view())

        container = QWidget()
        container.setLayout(layout)

        self.setCentralWidget(container)

    def load_url(self):
        url = self.url_bar.text()
        self.engine.load(url)
