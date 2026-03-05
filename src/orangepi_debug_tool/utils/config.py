"""
配置管理工具

支持 YAML 配置文件的加载和保存
"""

import os
import yaml
from typing import Optional, List
from dataclasses import dataclass, field
from pathlib import Path


@dataclass
class Config:
    """应用配置"""
    
    # 外观设置
    theme_mode: str = "dark"  # dark / light / system
    color_theme: str = "blue"  # blue / green / dark-blue
    
    # 日志设置
    log_level: str = "INFO"
    log_file: Optional[str] = None
    
    # 串口设置
    default_baudrate: int = 115200
    default_port: str = ""
    
    # 界面设置
    window_width: int = 1200
    window_height: int = 800
    font_family: str = "Microsoft YaHei"
    font_size: int = 12
    
    # 数据设置
    max_buffer_size: int = 1000
    auto_scroll: bool = True
    show_timestamp: bool = True
    hex_mode: bool = False
    
    @classmethod
    def load(cls, config_path: Optional[str] = None) -> "Config":
        """
        加载配置文件
        
        Args:
            config_path: 配置文件路径
            
        Returns:
            配置对象
        """
        if config_path is None:
            config_path = cls._get_default_config_path()
            
        path = Path(config_path)
        if not path.exists():
            return cls()
            
        try:
            with open(path, 'r', encoding='utf-8') as f:
                data = yaml.safe_load(f) or {}
            return cls(**data)
        except Exception:
            return cls()
            
    def save(self, config_path: Optional[str] = None) -> None:
        """
        保存配置文件
        
        Args:
            config_path: 配置文件路径
        """
        if config_path is None:
            config_path = self._get_default_config_path()
            
        path = Path(config_path)
        path.parent.mkdir(parents=True, exist_ok=True)
        
        with open(path, 'w', encoding='utf-8') as f:
            yaml.dump(self.__dict__, f, default_flow_style=False, allow_unicode=True)
            
    @staticmethod
    def _get_default_config_path() -> str:
        """获取默认配置路径"""
        return str(Path.home() / ".orangepi-debug-tool" / "config.yaml")
