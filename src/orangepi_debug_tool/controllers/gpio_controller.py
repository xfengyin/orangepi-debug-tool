"""
GPIO 控制器

处理 GPIO 相关的用户操作
"""

import logging
from typing import Dict

from ..models.gpio import GPIOModel, GPIOConfig, GPIOState, GPIOMode


class GPIOController:
    """GPIO 控制器"""
    
    def __init__(self, model: GPIOModel):
        self.logger = logging.getLogger(__name__)
        self.model = model
        
    def get_all_pins(self) -> Dict[int, GPIOConfig]:
        """获取所有引脚配置"""
        return self.model.get_all_pins()
        
    def get_pin(self, pin: int) -> GPIOConfig:
        """获取单个引脚配置"""
        return self.model.get_pin(pin)
        
    def toggle(self, pin: int) -> GPIOState:
        """切换引脚状态"""
        return self.model.toggle_pin(pin)
        
    def set_state(self, pin: int, state: GPIOState) -> bool:
        """设置引脚状态"""
        return self.model.set_pin_state(pin, state)
        
    def set_mode(self, pin: int, mode: GPIOMode) -> bool:
        """设置引脚模式"""
        return self.model.set_pin_mode(pin, mode)
        
    def set_pwm(self, pin: int, duty: float, freq: int = 1000) -> bool:
        """设置 PWM"""
        return self.model.set_pwm(pin, duty, freq)
