import sys
from PyQt5.QtWidgets import QApplication
from ui.browser_ui import BrowserUI

app = QApplication(sys.argv)

window = BrowserUI()
window.show()

sys.exit(app.exec_())
