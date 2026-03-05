"""
主窗口视图

采用 CustomTkinter 实现现代化 UI
"""

import customtkinter as ctk
from typing import Optional
import logging

from ..controllers.serial_controller import SerialController
from ..controllers.gpio_controller import GPIOController
from .serial_view import SerialView
from .gpio_view import GPIOView


class MainWindow:
    """
    主窗口
    
    职责：
    - 管理整体布局
    - 协调子视图
    - 处理窗口事件
    """
    
    def __init__(self, 
                 root: ctk.CTk,
                 serial_controller: SerialController,
                 gpio_controller: GPIOController):
        self.logger = logging.getLogger(__name__)
        self.root = root
        self.serial_controller = serial_controller
        self.gpio_controller = gpio_controller
        
        self._setup_window()
        self._create_widgets()
        
    def _setup_window(self) -> None:
        """设置窗口属性"""
        # 设置窗口最小尺寸
        self.root.minsize(800, 600)
        
        # 居中显示
        self.root.update_idletasks()
        width = self.root.winfo_width()
        height = self.root.winfo_height()
        x = (self.root.winfo_screenwidth() // 2) - (width // 2)
        y = (self.root.winfo_screenheight() // 2) - (height // 2)
        self.root.geometry(f"{width}x{height}+{x}+{y}")
        
    def _create_widgets(self) -> None:
        """创建界面组件"""
        # 创建顶部工具栏
        self._create_toolbar()
        
        # 创建标签页容器
        self.tabview = ctk.CTkTabview(self.root)
        self.tabview.pack(fill="both", expand=True, padx=10, pady=10)
        
        # 添加标签页
        self.tab_serial = self.tabview.add("串口通信")
        self.tab_gpio = self.tabview.add("GPIO 控制")
        self.tab_settings = self.tabview.add("设置")
        
        # 创建子视图
        self.serial_view = SerialView(
            self.tab_serial, 
            self.serial_controller
        )
        self.serial_view.pack(fill="both", expand=True)
        
        self.gpio_view = GPIOView(
            self.tab_gpio,
            self.gpio_controller
        )
        self.gpio_view.pack(fill="both", expand=True)
        
        # 创建设置页面
        self._create_settings_page()
        
        # 创建底部状态栏
        self._create_statusbar()
        
    def _create_toolbar(self) -> None:
        """创建工具栏"""
        self.toolbar = ctk.CTkFrame(self.root, height=40)
        self.toolbar.pack(fill="x", padx=10, pady=(10, 0))
        
        # 标题
        title_label = ctk.CTkLabel(
            self.toolbar,
            text="🔌 OrangePi 调试工具",
            font=ctk.CTkFont(size=16, weight="bold")
        )
        title_label.pack(side="left", padx=10)
        
        # 主题切换按钮
        self.theme_btn = ctk.CTkButton(
            self.toolbar,
            text="🌙",
            width=40,
            command=self._toggle_theme
        )
        self.theme_btn.pack(side="right", padx=5)
        
    def _create_statusbar(self) -> None:
        """创建状态栏"""
        self.statusbar = ctk.CTkFrame(self.root, height=30)
        self.statusbar.pack(fill="x", padx=10, pady=(0, 10))
        
        # 连接状态
        self.status_label = ctk.CTkLabel(
            self.statusbar,
            text="● 未连接",
            text_color="gray"
        )
        self.status_label.pack(side="left", padx=10)
        
        # 版本信息
        version_label = ctk.CTkLabel(
            self.statusbar,
            text="v2.0.0",
            text_color="gray"
        )
        version_label.pack(side="right", padx=10)
        
    def _create_settings_page(self) -> None:
        """创建设置页面"""
        # 主题设置
        theme_frame = ctk.CTkFrame(self.tab_settings)
        theme_frame.pack(fill="x", padx=20, pady=20)
        
        ctk.CTkLabel(
            theme_frame,
            text="主题设置",
            font=ctk.CTkFont(size=14, weight="bold")
        ).pack(anchor="w", padx=10, pady=10)
        
        # 主题模式选择
        self.theme_var = ctk.StringVar(value="dark")
        ctk.CTkRadioButton(
            theme_frame,
            text="深色模式",
            variable=self.theme_var,
            value="dark",
            command=lambda: ctk.set_appearance_mode("dark")
        ).pack(anchor="w", padx=20, pady=5)
        
        ctk.CTkRadioButton(
            theme_frame,
            text="浅色模式",
            variable=self.theme_var,
            value="light",
            command=lambda: ctk.set_appearance_mode("light")
        ).pack(anchor="w", padx=20, pady=5)
        
    def _toggle_theme(self) -> None:
        """切换主题"""
        current = ctk.get_appearance_mode()
        if current == "Dark":
            ctk.set_appearance_mode("light")
            self.theme_btn.configure(text="☀️")
        else:
            ctk.set_appearance_mode("dark")
            self.theme_btn.configure(text="🌙")
            
    def update_status(self, text: str, color: str = "gray") -> None:
        """更新状态栏"""
        self.status_label.configure(text=f"● {text}", text_color=color)
