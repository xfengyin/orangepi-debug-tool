"""
串口控制器

负责处理串口相关的用户操作
"""

import logging
from typing import Optional, List

from ..models.serial_port import SerialPortModel, ConnectionStatus


class SerialController:
    """
    串口控制器
    
    职责：
    - 处理用户操作
    - 协调模型和视图
    - 执行业务逻辑
    """
    
    def __init__(self, model: SerialPortModel):
        self.logger = logging.getLogger(__name__)
        self.model = model
        
    def get_available_ports(self) -> List[str]:
        """获取可用串口列表"""
        return self.model.list_available_ports()
        
    def connect(self, port: Optional[str] = None, baudrate: Optional[int] = None) -> bool:
        """
        连接串口
        
        Args:
            port: 串口名称
            baudrate: 波特率
            
        Returns:
            是否连接成功
        """
        return self.model.connect(port, baudrate)
        
    def disconnect(self) -> None:
        """断开连接"""
        self.model.disconnect()
        
    def send_data(self, data: str, hex_mode: bool = False) -> bool:
        """
        发送数据
        
        Args:
            data: 数据字符串
            hex_mode: 是否为十六进制模式
            
        Returns:
            是否发送成功
        """
        if hex_mode:
            try:
                bytes_data = bytes.fromhex(data.replace(" ", ""))
            except ValueError:
                self.logger.error("无效的十六进制数据")
                return False
        else:
            bytes_data = data.encode('utf-8')
            
        return self.model.send(bytes_data)
        
    def clear_buffer(self) -> None:
        """清空接收缓冲区"""
        self.model.clear_buffer()
        
    def get_connection_status(self) -> ConnectionStatus:
        """获取连接状态"""
        return self.model.status
        
    def is_connected(self) -> bool:
        """是否已连接"""
        return self.model.is_connected
