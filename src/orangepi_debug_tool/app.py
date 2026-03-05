"""
OrangePi Debug Tool - 主应用类

采用 MVC 架构设计，分离视图和业务逻辑
"""

import customtkinter as ctk
from typing import Optional
import logging

from .models.serial_port import SerialPortModel
from .models.gpio import GPIOModel
from .controllers.serial_controller import SerialController
from .controllers.gpio_controller import GPIOController
from .views.main_window import MainWindow
from .utils.config import Config
from .utils.logger import setup_logger


class OrangePiDebugTool:
    """
    OrangePi 调试工具主应用类
    
    职责：
    - 初始化应用组件
    - 协调 MVC 各层
    - 管理应用生命周期
    """
    
    def __init__(self, config_path: Optional[str] = None):
        """
        初始化应用
        
        Args:
            config_path: 配置文件路径
        """
        # 加载配置
        self.config = Config.load(config_path)
        
        # 设置日志
        self.logger = setup_logger(
            level=self.config.log_level,
            log_file=self.config.log_file
        )
        self.logger.info("OrangePi Debug Tool 启动中...")
        
        # 初始化主题
        self._setup_theme()
        
        # 创建主窗口
        self.root = ctk.CTk()
        self.root.title("OrangePi 调试工具 v2.0")
        self.root.geometry("1200x800")
        
        # 初始化模型
        self.serial_model = SerialPortModel()
        self.gpio_model = GPIOModel()
        
        # 初始化控制器
        self.serial_controller = SerialController(self.serial_model)
        self.gpio_controller = GPIOController(self.gpio_model)
        
        # 初始化视图
        self.main_window = MainWindow(
            self.root,
            self.serial_controller,
            self.gpio_controller
        )
        
        # 绑定关闭事件
        self.root.protocol("WM_DELETE_WINDOW", self._on_close)
        
    def _setup_theme(self) -> None:
        """设置主题"""
        ctk.set_appearance_mode(self.config.theme_mode)
        ctk.set_default_color_theme(self.config.color_theme)
        
    def run(self) -> None:
        """运行应用"""
        self.logger.info("应用启动成功")
        self.root.mainloop()
        
    def _on_close(self) -> None:
        """关闭应用"""
        self.logger.info("正在关闭应用...")
        
        # 断开串口连接
        if self.serial_model.is_connected:
            self.serial_controller.disconnect()
            
        # 保存配置
        self.config.save()
        
        self.root.destroy()


def main():
    """应用入口"""
    app = OrangePiDebugTool()
    app.run()


if __name__ == "__main__":
    main()
