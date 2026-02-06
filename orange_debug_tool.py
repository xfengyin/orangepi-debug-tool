#!/usr/bin/env python3
"""
OrangePi 上位机调试工具 - 启动脚本
"""

import sys
import os

# 添加源码目录到路径
sys.path.append(os.path.join(os.path.dirname(__file__), 'src'))

from main import OrangePiDebugger
import tkinter as tk


def main():
    """主函数"""
    try:
        root = tk.Tk()
        app = OrangePiDebugger(root)
        root.mainloop()
    except KeyboardInterrupt:
        print("\n程序被用户中断")
        sys.exit(0)
    except Exception as e:
        print(f"程序运行错误: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()