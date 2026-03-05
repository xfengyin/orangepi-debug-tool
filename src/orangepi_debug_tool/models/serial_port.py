"""
串口数据模型

负责串口状态管理和数据存储
"""

import serial
import serial.tools.list_ports
from typing import Optional, List, Callable
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
import threading
import logging


class ConnectionStatus(Enum):
    """连接状态枚举"""
    DISCONNECTED = "disconnected"
    CONNECTING = "connecting"
    CONNECTED = "connected"
    ERROR = "error"


@dataclass
class SerialConfig:
    """串口配置"""
    port: str = ""
    baudrate: int = 115200
    bytesize: int = 8
    parity: str = "N"
    stopbits: int = 1
    timeout: float = 1.0


@dataclass
class ReceivedData:
    """接收数据结构"""
    timestamp: datetime
    data: bytes
    data_type: str = "text"  # text / hex


class SerialPortModel:
    """
    串口数据模型
    
    职责：
    - 管理串口连接状态
    - 处理数据收发
    - 维护数据缓冲区
    """
    
    def __init__(self):
        self.logger = logging.getLogger(__name__)
        
        # 串口连接
        self._serial: Optional[serial.Serial] = None
        self._config = SerialConfig()
        self._status = ConnectionStatus.DISCONNECTED
        
        # 数据缓冲
        self._receive_buffer: List[ReceivedData] = []
        self._max_buffer_size = 1000
        
        # 回调函数
        self._on_data_received: Optional[Callable[[ReceivedData], None]] = None
        self._on_status_changed: Optional[Callable[[ConnectionStatus], None]] = None
        
        # 接收线程
        self._receive_thread: Optional[threading.Thread] = None
        self._is_receiving = False
        
    @property
    def is_connected(self) -> bool:
        """是否已连接"""
        return self._status == ConnectionStatus.CONNECTED
        
    @property
    def status(self) -> ConnectionStatus:
        """获取连接状态"""
        return self._status
        
    @property
    def config(self) -> SerialConfig:
        """获取配置"""
        return self._config
        
    @staticmethod
    def list_available_ports() -> List[str]:
        """列出可用串口"""
        ports = serial.tools.list_ports.comports()
        return [port.device for port in ports]
        
    def connect(self, port: Optional[str] = None, baudrate: Optional[int] = None) -> bool:
        """
        连接串口
        
        Args:
            port: 串口名称
            baudrate: 波特率
            
        Returns:
            是否连接成功
        """
        if port:
            self._config.port = port
        if baudrate:
            self._config.baudrate = baudrate
            
        if not self._config.port:
            self.logger.error("未指定串口")
            return False
            
        try:
            self._set_status(ConnectionStatus.CONNECTING)
            
            self._serial = serial.Serial(
                port=self._config.port,
                baudrate=self._config.baudrate,
                bytesize=self._config.bytesize,
                parity=self._config.parity,
                stopbits=self._config.stopbits,
                timeout=self._config.timeout
            )
            
            self._set_status(ConnectionStatus.CONNECTED)
            self._start_receiving()
            
            self.logger.info(f"已连接到 {self._config.port} @ {self._config.baudrate}")
            return True
            
        except Exception as e:
            self.logger.error(f"连接失败: {e}")
            self._set_status(ConnectionStatus.ERROR)
            return False
            
    def disconnect(self) -> None:
        """断开连接"""
        self._stop_receiving()
        
        if self._serial and self._serial.is_open:
            self._serial.close()
            
        self._serial = None
        self._set_status(ConnectionStatus.DISCONNECTED)
        self.logger.info("已断开连接")
        
    def send(self, data: bytes) -> bool:
        """
        发送数据
        
        Args:
            data: 要发送的数据
            
        Returns:
            是否发送成功
        """
        if not self.is_connected or not self._serial:
            self.logger.warning("未连接，无法发送数据")
            return False
            
        try:
            self._serial.write(data)
            self.logger.debug(f"发送数据: {data}")
            return True
        except Exception as e:
            self.logger.error(f"发送失败: {e}")
            return False
            
    def _start_receiving(self) -> None:
        """启动接收线程"""
        self._is_receiving = True
        self._receive_thread = threading.Thread(target=self._receive_loop, daemon=True)
        self._receive_thread.start()
        
    def _stop_receiving(self) -> None:
        """停止接收线程"""
        self._is_receiving = False
        if self._receive_thread:
            self._receive_thread.join(timeout=2)
            
    def _receive_loop(self) -> None:
        """接收数据循环"""
        while self._is_receiving and self._serial:
            try:
                if self._serial.in_waiting > 0:
                    data = self._serial.read(self._serial.in_waiting)
                    received = ReceivedData(
                        timestamp=datetime.now(),
                        data=data
                    )
                    self._add_to_buffer(received)
                    
                    if self._on_data_received:
                        self._on_data_received(received)
                        
            except Exception as e:
                self.logger.error(f"接收错误: {e}")
                break
                
    def _add_to_buffer(self, data: ReceivedData) -> None:
        """添加到缓冲区"""
        self._receive_buffer.append(data)
        if len(self._receive_buffer) > self._max_buffer_size:
            self._receive_buffer.pop(0)
            
    def _set_status(self, status: ConnectionStatus) -> None:
        """设置状态"""
        self._status = status
        if self._on_status_changed:
            self._on_status_changed(status)
            
    def set_callbacks(self, 
                      on_data_received: Optional[Callable] = None,
                      on_status_changed: Optional[Callable] = None) -> None:
        """设置回调函数"""
        self._on_data_received = on_data_received
        self._on_status_changed = on_status_changed
        
    def clear_buffer(self) -> None:
        """清空缓冲区"""
        self._receive_buffer.clear()
        
    def get_buffer_data(self) -> List[ReceivedData]:
        """获取缓冲区数据"""
        return self._receive_buffer.copy()
