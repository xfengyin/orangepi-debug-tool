"""
GPIO 控制视图

提供可视化的 GPIO 引脚控制界面
"""

import customtkinter as ctk
from typing import Dict
import logging

from ..controllers.gpio_controller import GPIOController
from ..models.gpio import GPIOState, GPIOMode


class GPIOView(ctk.CTkFrame):
    """GPIO 控制视图"""
    
    def __init__(self, master, controller: GPIOController):
        super().__init__(master)
        self.logger = logging.getLogger(__name__)
        self.controller = controller
        
        self.pin_buttons: Dict[int, ctk.CTkButton] = {}
        
        self._create_widgets()
        
    def _create_widgets(self) -> None:
        """创建界面组件"""
        # 标题
        header = ctk.CTkFrame(self)
        header.pack(fill="x", padx=10, pady=10)
        
        ctk.CTkLabel(
            header,
            text="🎛️ GPIO 引脚控制",
            font=ctk.CTkFont(size=16, weight="bold")
        ).pack(side="left", padx=10)
        
        # 说明
        ctk.CTkLabel(
            header,
            text="点击按钮切换引脚状态",
            text_color="gray"
        ).pack(side="right", padx=10)
        
        # 引脚网格容器
        self.pins_frame = ctk.CTkFrame(self)
        self.pins_frame.pack(fill="both", expand=True, padx=10, pady=10)
        
        # 创建引脚按钮网格
        self._create_pin_grid()
        
        # 控制面板
        self._create_control_panel()
        
    def _create_pin_grid(self) -> None:
        """创建引脚网格"""
        pins = self.controller.get_all_pins()
        
        # 创建滚动区域
        scrollable = ctk.CTkScrollableFrame(self.pins_frame)
        scrollable.pack(fill="both", expand=True)
        
        # 每行显示 4 个引脚
        row = 0
        col = 0
        
        for pin_num, pin_config in sorted(pins.items()):
            # 创建引脚卡片
            card = ctk.CTkFrame(scrollable)
            card.grid(row=row, column=col, padx=5, pady=5, sticky="nsew")
            
            # 引脚号
            ctk.CTkLabel(
                card,
                text=f"Pin {pin_num}",
                font=ctk.CTkFont(size=12, weight="bold")
            ).pack(pady=(10, 5))
            
            # 引脚名称
            ctk.CTkLabel(
                card,
                text=pin_config.name[:15],
                font=ctk.CTkFont(size=10),
                text_color="gray"
            ).pack(pady=5)
            
            # 状态按钮
            btn = ctk.CTkButton(
                card,
                text="LOW",
                width=80,
                command=lambda p=pin_num: self._toggle_pin(p),
                fg_color="gray"
            )
            btn.pack(pady=10)
            
            self.pin_buttons[pin_num] = btn
            
            # 模式选择
            mode_var = ctk.CTkOptionMenu(
                card,
                values=["OUTPUT", "INPUT", "PWM"],
                width=80,
                command=lambda m, p=pin_num: self._set_mode(p, m)
            )
            mode_var.set("OUTPUT")
            mode_var.pack(pady=(0, 10))
            
            # 更新列和行
            col += 1
            if col >= 4:
                col = 0
                row += 1
                
    def _create_control_panel(self) -> None:
        """创建控制面板"""
        panel = ctk.CTkFrame(self)
        panel.pack(fill="x", padx=10, pady=10)
        
        # 批量操作
        ctk.CTkLabel(
            panel,
            text="批量操作:",
            font=ctk.CTkFont(size=12, weight="bold")
        ).pack(side="left", padx=10)
        
        ctk.CTkButton(
            panel,
            text="全部 HIGH",
            width=100,
            command=lambda: self._set_all(GPIOState.HIGH)
        ).pack(side="left", padx=5)
        
        ctk.CTkButton(
            panel,
            text="全部 LOW",
            width=100,
            command=lambda: self._set_all(GPIOState.LOW)
        ).pack(side="left", padx=5)
        
    def _toggle_pin(self, pin: int) -> None:
        """切换引脚状态"""
        new_state = self.controller.toggle_pin(pin)
        self._update_pin_button(pin, new_state)
        
    def _update_pin_button(self, pin: int, state: GPIOState) -> None:
        """更新按钮显示"""
        if pin in self.pin_buttons:
            btn = self.pin_buttons[pin]
            if state == GPIOState.HIGH:
                btn.configure(text="HIGH", fg_color="green")
            else:
                btn.configure(text="LOW", fg_color="gray")
                
    def _set_mode(self, pin: int, mode: str) -> None:
        """设置引脚模式"""
        mode_map = {
            "OUTPUT": GPIOMode.OUTPUT,
            "INPUT": GPIOMode.INPUT,
            "PWM": GPIOMode.PWM
        }
        self.controller.set_mode(pin, mode_map.get(mode, GPIOMode.OUTPUT))
        
    def _set_all(self, state: GPIOState) -> None:
        """设置所有引脚状态"""
        for pin in self.pin_buttons:
            self.controller.set_state(pin, state)
            self._update_pin_button(pin, state)
