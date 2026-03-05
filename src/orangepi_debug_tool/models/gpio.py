"""
GPIO 数据模型

负责 GPIO 引脚状态管理和控制
"""

from typing import Dict, Optional, Callable
from dataclasses import dataclass
from enum import Enum
import logging


class GPIOMode(Enum):
    """GPIO 模式"""
    INPUT = "input"
    OUTPUT = "output"
    PWM = "pwm"


class GPIOState(Enum):
    """GPIO 状态"""
    LOW = 0
    HIGH = 1


@dataclass
class GPIOConfig:
    """GPIO 引脚配置"""
    pin: int
    name: str
    mode: GPIOMode = GPIOMode.OUTPUT
    state: GPIOState = GPIOState.LOW
    pwm_duty: float = 0.0
    pwm_freq: int = 1000


class GPIOModel:
    """
    GPIO 数据模型
    
    职责：
    - 管理引脚配置
    - 维护引脚状态
    - 提供状态查询接口
    """
    
    # OrangePi 常用 GPIO 引脚定义
    DEFAULT_PINS = {
        2: "GPIO2 (SDA)",
        3: "GPIO3 (SCL)",
        4: "GPIO4",
        5: "GPIO5",
        6: "GPIO6",
        7: "GPIO7 (CE1)",
        8: "GPIO8 (CE0)",
        9: "GPIO9 (MISO)",
        10: "GPIO10 (MOSI)",
        11: "GPIO11 (SCLK)",
        12: "GPIO12",
        13: "GPIO13",
        14: "GPIO14 (TXD)",
        15: "GPIO15 (RXD)",
        16: "GPIO16",
        17: "GPIO17",
        18: "GPIO18",
        19: "GPIO19",
        20: "GPIO20",
        21: "GPIO21",
        22: "GPIO22",
        23: "GPIO23",
        24: "GPIO24",
        25: "GPIO25",
        26: "GPIO26",
        27: "GPIO27",
    }
    
    def __init__(self):
        self.logger = logging.getLogger(__name__)
        
        # 初始化引脚配置
        self._pins: Dict[int, GPIOConfig] = {}
        self._init_default_pins()
        
        # 回调函数
        self._on_pin_changed: Optional[Callable[[int, GPIOConfig], None]] = None
        
    def _init_default_pins(self) -> None:
        """初始化默认引脚"""
        for pin, name in self.DEFAULT_PINS.items():
            self._pins[pin] = GPIOConfig(pin=pin, name=name)
            
    def get_pin(self, pin: int) -> Optional[GPIOConfig]:
        """获取引脚配置"""
        return self._pins.get(pin)
        
    def get_all_pins(self) -> Dict[int, GPIOConfig]:
        """获取所有引脚"""
        return self._pins.copy()
        
    def set_pin_state(self, pin: int, state: GPIOState) -> bool:
        """
        设置引脚状态
        
        Args:
            pin: 引脚号
            state: 状态
            
        Returns:
            是否设置成功
        """
        if pin not in self._pins:
            self.logger.warning(f"引脚 {pin} 不存在")
            return False
            
        config = self._pins[pin]
        config.state = state
        
        self._notify_pin_changed(pin, config)
        self.logger.info(f"引脚 {pin} 状态设置为 {state.name}")
        return True
        
    def set_pin_mode(self, pin: int, mode: GPIOMode) -> bool:
        """
        设置引脚模式
        
        Args:
            pin: 引脚号
            mode: 模式
            
        Returns:
            是否设置成功
        """
        if pin not in self._pins:
            return False
            
        self._pins[pin].mode = mode
        self._notify_pin_changed(pin, self._pins[pin])
        return True
        
    def toggle_pin(self, pin: int) -> GPIOState:
        """
        切换引脚状态
        
        Args:
            pin: 引脚号
            
        Returns:
            新状态
        """
        config = self._pins.get(pin)
        if not config:
            return GPIOState.LOW
            
        new_state = GPIOState.HIGH if config.state == GPIOState.LOW else GPIOState.LOW
        self.set_pin_state(pin, new_state)
        return new_state
        
    def set_pwm(self, pin: int, duty: float, freq: int = 1000) -> bool:
        """
        设置 PWM
        
        Args:
            pin: 引脚号
            duty: 占空比 (0-100)
            freq: 频率
            
        Returns:
            是否设置成功
        """
        if pin not in self._pins:
            return False
            
        config = self._pins[pin]
        config.mode = GPIOMode.PWM
        config.pwm_duty = max(0, min(100, duty))
        config.pwm_freq = freq
        
        self._notify_pin_changed(pin, config)
        return True
        
    def _notify_pin_changed(self, pin: int, config: GPIOConfig) -> None:
        """通知引脚状态变化"""
        if self._on_pin_changed:
            self._on_pin_changed(pin, config)
            
    def set_callback(self, callback: Optional[Callable]) -> None:
        """设置状态变化回调"""
        self._on_pin_changed = callback
