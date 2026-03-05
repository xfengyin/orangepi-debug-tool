"""
Unit tests for OrangePi Debug Tool
"""

import pytest
from orangepi_debug_tool.models.serial_port import SerialPortModel, ConnectionStatus
from orangepi_debug_tool.models.gpio import GPIOModel, GPIOState, GPIOMode
from orangepi_debug_tool.utils.config import Config


class TestSerialPortModel:
    """Test SerialPortModel"""
    
    def test_initialization(self):
        """Test model initialization"""
        model = SerialPortModel()
        assert model is not None
        assert model.status == ConnectionStatus.DISCONNECTED
        assert not model.is_connected
        
    def test_list_ports(self):
        """Test listing available ports"""
        ports = SerialPortModel.list_available_ports()
        assert isinstance(ports, list)
        
    def test_config_defaults(self):
        """Test default config values"""
        model = SerialPortModel()
        assert model.config.baudrate == 115200
        assert model.config.timeout == 1.0


class TestGPIOModel:
    """Test GPIOModel"""
    
    def test_initialization(self):
        """Test model initialization"""
        model = GPIOModel()
        assert model is not None
        assert len(model.get_all_pins()) > 0
        
    def test_get_pin(self):
        """Test getting a pin"""
        model = GPIOModel()
        pin = model.get_pin(2)
        assert pin is not None
        assert pin.pin == 2
        
    def test_toggle_pin(self):
        """Test toggling a pin"""
        model = GPIOModel()
        new_state = model.toggle_pin(2)
        assert new_state in [GPIOState.HIGH, GPIOState.LOW]
        
    def test_set_pin_state(self):
        """Test setting pin state"""
        model = GPIOModel()
        result = model.set_pin_state(2, GPIOState.HIGH)
        assert result is True
        
        pin = model.get_pin(2)
        assert pin.state == GPIOState.HIGH
        
    def test_set_pin_mode(self):
        """Test setting pin mode"""
        model = GPIOModel()
        result = model.set_pin_mode(2, GPIOMode.INPUT)
        assert result is True
        
        pin = model.get_pin(2)
        assert pin.mode == GPIOMode.INPUT


class TestConfig:
    """Test Config"""
    
    def test_defaults(self):
        """Test default values"""
        config = Config()
        assert config.theme_mode == "dark"
        assert config.log_level == "INFO"
        assert config.default_baudrate == 115200
        
    def test_load_save(self, tmp_path):
        """Test load and save"""
        config = Config()
        config.theme_mode = "light"
        
        config_path = str(tmp_path / "config.yaml")
        config.save(config_path)
        
        loaded = Config.load(config_path)
        assert loaded.theme_mode == "light"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
