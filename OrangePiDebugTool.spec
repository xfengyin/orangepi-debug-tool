# -*- mode: python ; coding: utf-8 -*-
"""
PyInstaller 配置文件 - OrangePi Debug Tool
"""

import sys
from pathlib import Path

block_cipher = None

# 获取项目根目录
project_root = Path(SPECPATH)

a = Analysis(
    ['orange_debug_tool.py'],
    pathex=[str(project_root), str(project_root / 'src')],
    binaries=[],
    datas=[
        ('src/orangepi_debug_tool', 'orangepi_debug_tool'),
    ],
    hiddenimports=[
        'customtkinter',
        'serial',
        'serial.tools',
        'serial.tools.list_ports',
        'yaml',
        'rich',
    ],
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    win_no_prefer_redirects=False,
    win_private_assemblies=False,
    cipher=block_cipher,
    noarchive=False,
)

pyz = PYZ(a.pure, a.zipped_data, cipher=block_cipher)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.zipfiles,
    a.datas,
    [],
    name='OrangePiDebugTool',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=False,  # GUI模式，不显示控制台
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
)
