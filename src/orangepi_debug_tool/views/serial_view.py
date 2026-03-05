"""
串口通信视图

提供现代化的串口通信界面
"""

import customtkinter as ctk
from typing import Optional
from datetime import datetime
import logging

from ..controllers.serial_controller import SerialController
from ..models.serial_port import ConnectionStatus


class SerialView(ctk.CTkFrame):
    """串口通信视图"""
    
    def __init__(self, master, controller: SerialController):
        super().__init__(master)
        self.logger = logging.getLogger(__name__)
        self.controller = controller
        
        self._create_widgets()
        self._bind_events()
        
    def _create_widgets(self) -> None:
        """创建界面组件"""
        # 左侧控制面板
        self.control_panel = ctk.CTkFrame(self, width=250)
        self.control_panel.pack(side="left", fill="y", padx=10, pady=10)
        self.control_panel.pack_propagate(False)
        
        # 连接设置
        self._create_connection_settings()
        
        # 发送设置
        self._create_send_settings()
        
        # 右侧数据区域
        self.data_panel = ctk.CTkFrame(self)
        self.data_panel.pack(side="right", fill="both", expand=True, padx=10, pady=10)
        
        # 接收数据显示
        self._create_receive_area()
        
    def _create_connection_settings(self) -> None:
        """创建连接设置"""
        # 标题
        ctk.CTkLabel(
            self.control_panel,
            text="连接设置",
            font=ctk.CTkFont(size=14, weight="bold")
        ).pack(pady=10)
        
        # 串口选择
        ctk.CTkLabel(self.control_panel, text="串口:").pack(anchor="w", padx=10)
        
        self.port_var = ctk.StringVar()
        self.port_combo = ctk.CTkComboBox(
            self.control_panel,
            variable=self.port_var,
            values=[""],
            width=200
        )
        self.port_combo.pack(padx=10, pady=5)
        
        # 刷新按钮
        ctk.CTkButton(
            self.control_panel,
            text="🔄 刷新",
            width=200,
            command=self._refresh_ports
        ).pack(pady=5)
        
        # 波特率选择
        ctk.CTkLabel(self.control_panel, text="波特率:").pack(anchor="w", padx=10, pady=(10, 0))
        
        self.baudrate_var = ctk.StringVar(value="115200")
        self.baudrate_combo = ctk.CTkComboBox(
            self.control_panel,
            variable=self.baudrate_var,
            values=["9600", "19200", "38400", "57600", "115200", "230400", "460800", "921600"],
            width=200
        )
        self.baudrate_combo.pack(padx=10, pady=5)
        
        # 连接按钮
        self.connect_btn = ctk.CTkButton(
            self.control_panel,
            text="🔗 连接",
            width=200,
            command=self._toggle_connection
        )
        self.connect_btn.pack(pady=20)
        
    def _create_send_settings(self) -> None:
        """创建发送设置"""
        # 分隔线
        ctk.CTkFrame(self.control_panel, height=2).pack(fill="x", padx=10, pady=10)
        
        ctk.CTkLabel(
            self.control_panel,
            text="发送设置",
            font=ctk.CTkFont(size=14, weight="bold")
        ).pack(pady=10)
        
        # 发送模式
        self.hex_mode_var = ctk.BooleanVar(value=False)
        ctk.CTkCheckBox(
            self.control_panel,
            text="十六进制模式",
            variable=self.hex_mode_var
        ).pack(anchor="w", padx=10)
        
        # 自动换行
        self.auto_newline_var = ctk.BooleanVar(value=True)
        ctk.CTkCheckBox(
            self.control_panel,
            text="自动添加换行",
            variable=self.auto_newline_var
        ).pack(anchor="w", padx=10, pady=5)
        
    def _create_receive_area(self) -> None:
        """创建接收区域"""
        # 接收数据标题
        header = ctk.CTkFrame(self.data_panel)
        header.pack(fill="x", padx=10, pady=10)
        
        ctk.CTkLabel(
            header,
            text="📥 接收数据",
            font=ctk.CTkFont(size=14, weight="bold")
        ).pack(side="left")
        
        # 清空按钮
        ctk.CTkButton(
            header,
            text="清空",
            width=60,
            command=self._clear_receive
        ).pack(side="right")
        
        # 接收文本框
        self.receive_text = ctk.CTkTextbox(
            self.data_panel,
            font=ctk.CTkFont(family="Consolas", size=12)
        )
        self.receive_text.pack(fill="both", expand=True, padx=10, pady=10)
        
        # 发送区域
        send_frame = ctk.CTkFrame(self.data_panel)
        send_frame.pack(fill="x", padx=10, pady=10)
        
        ctk.CTkLabel(
            send_frame,
            text="📤 发送:",
            font=ctk.CTkFont(size=12)
        ).pack(side="left", padx=5)
        
        self.send_entry = ctk.CTkEntry(send_frame)
        self.send_entry.pack(side="left", fill="x", expand=True, padx=5)
        
        ctk.CTkButton(
            send_frame,
            text="发送",
            width=60,
            command=self._send_data
        ).pack(side="right", padx=5)
        
    def _bind_events(self) -> None:
        """绑定事件"""
        self.send_entry.bind("<Return>", lambda e: self._send_data())
        
    def _refresh_ports(self) -> None:
        """刷新串口列表"""
        ports = self.controller.get_available_ports()
        self.port_combo.configure(values=ports if ports else [""])
        if ports:
            self.port_var.set(ports[0])
            
    def _toggle_connection(self) -> None:
        """切换连接状态"""
        if self.controller.is_connected():
            self.controller.disconnect()
            self.connect_btn.configure(text="🔗 连接", fg_color="blue")
        else:
            port = self.port_var.get()
            baudrate = int(self.baudrate_var.get())
            if self.controller.connect(port, baudrate):
                self.connect_btn.configure(text="🔌 断开", fg_color="red")
                
    def _send_data(self) -> None:
        """发送数据"""
        data = self.send_entry.get()
        if not data:
            return
            
        if self.auto_newline_var.get():
            data += "\n"
            
        hex_mode = self.hex_mode_var.get()
        if self.controller.send_data(data, hex_mode):
            self.send_entry.delete(0, "end")
            
    def _clear_receive(self) -> None:
        """清空接收区"""
        self.receive_text.delete("1.0", "end")
        self.controller.clear_buffer()
