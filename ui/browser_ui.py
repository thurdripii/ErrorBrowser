from PyQt5.QtWidgets import QMainWindow, QLineEdit, QVBoxLayout, QWidget, QHBoxLayout, QPushButton
from engine.yeetcore.engine import ErrorEngine

class BrowserUI(QMainWindow):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Error Browser")

        self.engine = ErrorEngine()

        # Barra de URL
        self.url_bar = QLineEdit()
        self.url_bar.returnPressed.connect(self.load_url)

        # Botões
        self.back_btn = QPushButton("←")
        self.forward_btn = QPushButton("→")
        self.reload_btn = QPushButton("⟳")

        self.back_btn.clicked.connect(self.engine.get_view().back)
        self.forward_btn.clicked.connect(self.engine.get_view().forward)
        self.reload_btn.clicked.connect(self.engine.get_view().reload)

        # Layout da barra superior
        top_bar = QHBoxLayout()
        top_bar.addWidget(self.back_btn)
        top_bar.addWidget(self.forward_btn)
        top_bar.addWidget(self.reload_btn)
        top_bar.addWidget(self.url_bar)

        # Layout principal
        layout = QVBoxLayout()
        layout.addLayout(top_bar)
        layout.addWidget(self.engine.get_view())

        container = QWidget()
        container.setLayout(layout)

        self.setCentralWidget(container)

    def load_url(self):
        url = self.url_bar.text()
        if not url.startswith("http"):
            url = "https://" + url
        self.engine.load(url)
