"""
测试脚本 - 验证 OrangePi 上位机调试工具的基本功能
"""

import unittest
import sys
import os
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'src'))

import tkinter as tk
# 导入主要模块进行测试
try:
    from main import OrangePiDebugger
except ImportError as e:
    print(f"导入错误: {e}")
    print("请确保已安装所需依赖")


class TestOrangePiDebugger(unittest.TestCase):
    """测试 OrangePi 上位机调试工具的主要功能"""
    
    def setUp(self):
        """设置测试环境"""
        self.root = tk.Tk()
        self.root.withdraw()  # 隐藏主窗口
        self.app = OrangePiDebugger(self.root)
    
    def tearDown(self):
        """清理测试环境"""
        self.root.destroy()
    
    def test_initial_state(self):
        """测试初始状态"""
        self.assertFalse(self.app.is_connected)
        self.assertEqual(self.app.status_var.get(), "就绪")
    
    def test_port_refresh(self):
        """测试端口刷新功能"""
        initial_ports = self.app.port_combo['values']
        self.app.refresh_ports()
        updated_ports = self.app.port_combo['values']
        # 检查是否成功获取了端口列表
        self.assertIsInstance(updated_ports, tuple)
    
    def test_variables_initialized(self):
        """测试变量初始化"""
        self.assertIsNotNone(self.app.port_var)
        self.assertIsNotNone(self.app.baud_var)
        self.assertIsNotNone(self.app.send_hex_var)
        self.assertIsNotNone(self.app.receive_hex_var)


def run_tests():
    """运行所有测试"""
    print("开始测试 OrangePi 上位机调试工具...")
    
    # 创建测试套件
    suite = unittest.TestLoader().loadTestsFromTestCase(TestOrangePiDebugger)
    
    # 运行测试
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)
    
    # 输出测试结果
    print(f"\n测试完成:")
    print(f"运行测试数: {result.testsRun}")
    print(f"失败数: {len(result.failures)}")
    print(f"错误数: {len(result.errors)}")
    print(f"成功率: {((result.testsRun - len(result.failures) - len(result.errors)) / result.testsRun * 100):.1f}%" if result.testsRun > 0 else "0%")
    
    return result.wasSuccessful()


if __name__ == '__main__':
    success = run_tests()
    sys.exit(0 if success else 1)