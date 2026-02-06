"""
简化测试脚本 - 验证 OrangePi 上位机调试工具的核心逻辑
"""

import sys
import os
import importlib.util

# 导入主要模块进行测试
def test_imports():
    """测试能否成功导入所需模块"""
    try:
        import tkinter as tk
        print("✓ tkinter 导入成功")
    except ImportError:
        print("✗ tkinter 导入失败")
        return False
    
    try:
        import serial
        print("✓ serial 导入成功")
    except ImportError:
        print("✗ serial 导入失败")
        return False
    
    try:
        import serial.tools.list_ports
        print("✓ serial.tools.list_ports 导入成功")
    except ImportError:
        print("✗ serial.tools.list_ports 导入失败")
        return False
    
    # 尝试导入我们的主模块
    try:
        spec = importlib.util.spec_from_file_location("main", "./src/main.py")
        main_module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(main_module)
        print("✓ main.py 模块加载成功")
    except Exception as e:
        print(f"✗ main.py 模块加载失败: {e}")
        return False
    
    return True

def test_main_function():
    """测试主模块的基本功能"""
    try:
        spec = importlib.util.spec_from_file_location("main", "./src/main.py")
        main_module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(main_module)
        
        # 检查主要类是否存在
        assert hasattr(main_module, 'OrangePiDebugger'), "OrangePiDebugger 类不存在"
        print("✓ OrangePiDebugger 类存在")
        
        # 检查主要函数是否存在
        assert hasattr(main_module, 'main'), "main 函数不存在"
        print("✓ main 函数存在")
        
        return True
    except Exception as e:
        print(f"✗ 主模块功能测试失败: {e}")
        return False

def test_config_file():
    """测试配置文件"""
    try:
        with open('./config.ini', 'r') as f:
            content = f.read()
            assert '[DEFAULT]' in content, "配置文件缺少 [DEFAULT] 段"
            assert '[SERIAL]' in content, "配置文件缺少 [SERIAL] 段"
            print("✓ config.ini 文件格式正确")
            return True
    except Exception as e:
        print(f"✗ 配置文件测试失败: {e}")
        return False

def test_requirements():
    """测试依赖文件"""
    try:
        with open('./requirements.txt', 'r') as f:
            content = f.read()
            assert 'pyserial' in content, "requirements.txt 中缺少 pyserial"
            print("✓ requirements.txt 文件格式正确")
            return True
    except Exception as e:
        print(f"✗ requirements.txt 测试失败: {e}")
        return False

def test_documentation():
    """测试文档文件"""
    files_to_check = [
        './README.md',
        './docs/user_guide.md'
    ]
    
    for file_path in files_to_check:
        try:
            with open(file_path, 'r') as f:
                content = f.read()
                assert len(content) > 0, f"{file_path} 文件为空"
                print(f"✓ {file_path} 文件存在且非空")
        except Exception as e:
            print(f"✗ {file_path} 测试失败: {e}")
            return False
    
    return True

def run_tests():
    """运行所有测试"""
    print("开始测试 OrangePi 上位机调试工具...")
    print("="*50)
    
    tests = [
        ("导入测试", test_imports),
        ("主模块功能测试", test_main_function),
        ("配置文件测试", test_config_file),
        ("依赖文件测试", test_requirements),
        ("文档文件测试", test_documentation),
    ]
    
    passed = 0
    total = len(tests)
    
    for test_name, test_func in tests:
        print(f"\n{test_name}:")
        if test_func():
            passed += 1
            print(f"  结果: 通过 ✓")
        else:
            print(f"  结果: 失败 ✗")
    
    print("\n" + "="*50)
    print(f"总体结果: {passed}/{total} 测试通过")
    
    if passed == total:
        print("🎉 所有测试通过！项目已准备好部署。")
        return True
    else:
        print("⚠️  部分测试未通过，请检查问题。")
        return False

if __name__ == '__main__':
    success = run_tests()
    sys.exit(0 if success else 1)