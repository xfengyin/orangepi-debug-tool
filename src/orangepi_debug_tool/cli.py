"""
Command Line Interface
"""

import argparse
from .app import main


def run():
    """CLI entry point"""
    parser = argparse.ArgumentParser(
        description="OrangePi Debug Tool - 现代化硬件调试工具"
    )
    parser.add_argument(
        "-c", "--config",
        help="配置文件路径"
    )
    parser.add_argument(
        "-v", "--version",
        action="version",
        version="%(prog)s 2.0.0"
    )
    
    args = parser.parse_args()
    main()


if __name__ == "__main__":
    run()
