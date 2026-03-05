#!/usr/bin/env python3
"""
OrangePi Debug Tool - 主入口
"""

import sys
import os

# 确保能找到模块
if getattr(sys, 'frozen', False):
    # PyInstaller 打包后的路径
    base_path = sys._MEIPASS
else:
    # 开发环境
    base_path = os.path.dirname(os.path.abspath(__file__))

sys.path.insert(0, os.path.join(base_path, 'src'))

# 导入并运行
from orangepi_debug_tool.app import OrangePiDebugTool

if __name__ == "__main__":
    app = OrangePiDebugTool()
    app.run()
