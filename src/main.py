"""
OrangePi 上位机调试工具
一个用于调试 OrangePi 设备的上位机软件
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import threading
import time
import json
import serial
import serial.tools.list_ports
from datetime import datetime


class OrangePiDebugger:
    def __init__(self, root):
        self.root = root
        self.root.title("OrangePi 上位机调试工具")
        self.root.geometry("900x700")
        
        # 串口相关变量
        self.serial_conn = None
        self.is_connected = False
        self.receive_thread = None
        self.is_receiving = False
        
        self.setup_ui()
        
    def setup_ui(self):
        """设置用户界面"""
        # 创建主框架
        main_frame = ttk.Frame(self.root, padding="10")
        main_frame.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))
        
        # 顶部连接设置区域
        connection_frame = ttk.LabelFrame(main_frame, text="连接设置", padding="10")
        connection_frame.grid(row=0, column=0, columnspan=2, sticky=(tk.W, tk.E), pady=(0, 10))
        
        # 串口选择
        ttk.Label(connection_frame, text="串口号:").grid(row=0, column=0, padx=(0, 5))
        self.port_var = tk.StringVar()
        self.port_combo = ttk.Combobox(connection_frame, textvariable=self.port_var, width=15)
        self.port_combo.grid(row=0, column=1, padx=(0, 10))
        
        ttk.Button(connection_frame, text="刷新端口", command=self.refresh_ports).grid(row=0, column=2, padx=(0, 10))
        
        # 波特率选择
        ttk.Label(connection_frame, text="波特率:").grid(row=0, column=3, padx=(0, 5))
        self.baud_var = tk.StringVar(value="115200")
        baud_options = ["9600", "19200", "38400", "57600", "115200", "230400", "460800", "921600"]
        self.baud_combo = ttk.Combobox(connection_frame, textvariable=self.baud_var, values=baud_options, width=10)
        self.baud_combo.grid(row=0, column=4, padx=(0, 10))
        
        # 连接按钮
        self.connect_btn = ttk.Button(connection_frame, text="连接", command=self.toggle_connection)
        self.connect_btn.grid(row=0, column=5)
        
        # 发送区域
        send_frame = ttk.LabelFrame(main_frame, text="发送数据", padding="10")
        send_frame.grid(row=1, column=0, sticky=(tk.W, tk.E, tk.N, tk.S), pady=(0, 10))
        
        # 发送文本框
        self.send_text = scrolledtext.ScrolledText(send_frame, height=6, width=50)
        self.send_text.grid(row=0, column=0, columnspan=3, sticky=(tk.W, tk.E))
        
        # 发送选项
        self.send_hex_var = tk.BooleanVar()
        ttk.Checkbutton(send_frame, text="十六进制发送", variable=self.send_hex_var).grid(row=1, column=0, sticky=tk.W, pady=(5, 0))
        
        self.add_line_var = tk.BooleanVar(value=True)
        ttk.Checkbutton(send_frame, text="自动换行", variable=self.add_line_var).grid(row=1, column=1, sticky=tk.W, pady=(5, 0), padx=(10, 0))
        
        # 发送按钮
        ttk.Button(send_frame, text="发送", command=self.send_data).grid(row=1, column=2, pady=(5, 0))
        
        # 接收区域
        receive_frame = ttk.LabelFrame(main_frame, text="接收数据", padding="10")
        receive_frame.grid(row=1, column=1, sticky=(tk.W, tk.E, tk.N, tk.S), padx=(10, 0), pady=(0, 10))
        
        # 接收文本框
        self.receive_text = scrolledtext.ScrolledText(receive_frame, height=20, width=50)
        self.receive_text.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))
        
        # 接收选项
        self.receive_hex_var = tk.BooleanVar()
        ttk.Checkbutton(receive_frame, text="十六进制显示", variable=self.receive_hex_var).grid(row=1, column=0, sticky=tk.W, pady=(5, 0))
        
        # 清空接收区按钮
        ttk.Button(receive_frame, text="清空", command=self.clear_receive).grid(row=1, column=0, sticky=tk.E, pady=(5, 0))
        
        # 底部控制区域
        control_frame = ttk.Frame(main_frame)
        control_frame.grid(row=2, column=0, columnspan=2, sticky=(tk.W, tk.E), pady=(0, 10))
        
        # GPIO控制区域
        gpio_frame = ttk.LabelFrame(control_frame, text="GPIO控制", padding="10")
        gpio_frame.grid(row=0, column=0, sticky=(tk.W, tk.E), padx=(0, 10))
        
        # GPIO引脚输入
        ttk.Label(gpio_frame, text="GPIO编号:").grid(row=0, column=0, padx=(0, 5))
        self.gpio_pin_var = tk.StringVar()
        self.gpio_pin_entry = ttk.Entry(gpio_frame, textvariable=self.gpio_pin_var, width=10)
        self.gpio_pin_entry.grid(row=0, column=1, padx=(0, 10))
        
        # GPIO值选择
        ttk.Label(gpio_frame, text="值:").grid(row=0, column=2, padx=(0, 5))
        self.gpio_value_var = tk.StringVar(value="HIGH")
        gpio_values = ["HIGH", "LOW"]
        self.gpio_value_combo = ttk.Combobox(gpio_frame, textvariable=self.gpio_value_var, values=gpio_values, width=8)
        self.gpio_value_combo.grid(row=0, column=3, padx=(0, 10))
        
        # GPIO控制按钮
        ttk.Button(gpio_frame, text="设置GPIO", command=self.set_gpio).grid(row=0, column=4)
        
        # 状态栏
        self.status_var = tk.StringVar(value="就绪")
        status_bar = ttk.Label(main_frame, textvariable=self.status_var, relief=tk.SUNKEN, anchor=tk.W)
        status_bar.grid(row=3, column=0, columnspan=2, sticky=(tk.W, tk.E))
        
        # 配置网格权重
        self.root.columnconfigure(0, weight=1)
        self.root.rowconfigure(0, weight=1)
        main_frame.columnconfigure(0, weight=1)
        main_frame.columnconfigure(1, weight=1)
        main_frame.rowconfigure(1, weight=1)
        send_frame.columnconfigure(0, weight=1)
        send_frame.rowconfigure(0, weight=1)
        receive_frame.columnconfigure(0, weight=1)
        receive_frame.rowconfigure(0, weight=1)
        
        # 初始刷新端口
        self.refresh_ports()
    
    def refresh_ports(self):
        """刷新串口列表"""
        ports = [port.device for port in serial.tools.list_ports.comports()]
        self.port_combo['values'] = ports
        if ports:
            self.port_var.set(ports[0])
    
    def toggle_connection(self):
        """切换连接状态"""
        if not self.is_connected:
            self.connect_serial()
        else:
            self.disconnect_serial()
    
    def connect_serial(self):
        """连接串口"""
        try:
            port = self.port_var.get()
            baudrate = int(self.baud_var.get())
            
            if not port:
                messagebox.showerror("错误", "请选择串口号")
                return
                
            self.serial_conn = serial.Serial(port, baudrate, timeout=1)
            self.is_connected = True
            self.connect_btn.config(text="断开")
            self.status_var.set(f"已连接到 {port} @ {baudrate}bps")
            
            # 启动接收线程
            self.is_receiving = True
            self.receive_thread = threading.Thread(target=self.receive_data, daemon=True)
            self.receive_thread.start()
            
        except Exception as e:
            messagebox.showerror("连接错误", f"无法连接到串口: {str(e)}")
            self.is_connected = False
    
    def disconnect_serial(self):
        """断开串口连接"""
        self.is_receiving = False
        if self.serial_conn and self.serial_conn.is_open:
            self.serial_conn.close()
        self.is_connected = False
        self.connect_btn.config(text="连接")
        self.status_var.set("已断开连接")
    
    def receive_data(self):
        """接收串口数据"""
        while self.is_receiving and self.is_connected:
            try:
                if self.serial_conn and self.serial_conn.in_waiting > 0:
                    data = self.serial_conn.read(self.serial_conn.in_waiting)
                    self.display_received_data(data)
                time.sleep(0.01)  # 10ms延迟
            except Exception as e:
                print(f"接收数据错误: {e}")
                break
    
    def display_received_data(self, data):
        """显示接收到的数据"""
        try:
            if self.receive_hex_var.get():
                # 十六进制显示
                hex_str = ' '.join([f'{byte:02X}' for byte in data])
                display_text = hex_str
            else:
                # 字符串显示
                display_text = data.decode('utf-8', errors='ignore')
            
            timestamp = datetime.now().strftime("%H:%M:%S.%f")[:-3]
            
            # 在主线程中更新UI
            self.root.after(0, self.update_receive_display, f"[{timestamp}] {display_text}\n")
        except Exception as e:
            print(f"显示数据错误: {e}")
    
    def update_receive_display(self, text):
        """更新接收显示"""
        self.receive_text.insert(tk.END, text)
        self.receive_text.see(tk.END)
    
    def send_data(self):
        """发送数据"""
        if not self.is_connected or not self.serial_conn:
            messagebox.showwarning("警告", "请先连接串口")
            return
            
        try:
            text = self.send_text.get("1.0", tk.END).strip()
            if not text:
                return
                
            if self.send_hex_var.get():
                # 十六进制发送
                hex_values = text.split()
                byte_data = bytes([int(h, 16) for h in hex_values])
            else:
                # 文本发送
                byte_data = text.encode('utf-8')
                
            if self.add_line_var.get():
                byte_data += b'\r\n'
                
            self.serial_conn.write(byte_data)
            
            # 更新状态
            self.status_var.set(f"已发送 {len(byte_data)} 字节")
            
        except Exception as e:
            messagebox.showerror("发送错误", f"发送数据失败: {str(e)}")
    
    def clear_receive(self):
        """清空接收区域"""
        self.receive_text.delete("1.0", tk.END)
    
    def set_gpio(self):
        """设置GPIO"""
        pin = self.gpio_pin_var.get()
        value = self.gpio_value_var.get()
        
        if not pin:
            messagebox.showwarning("警告", "请输入GPIO编号")
            return
            
        # 构造GPIO控制命令
        command = f"GPIO,{pin},{value}\n"
        
        if self.is_connected:
            try:
                self.serial_conn.write(command.encode())
                self.status_var.set(f"GPIO {pin} 设置为 {value}")
            except Exception as e:
                messagebox.showerror("GPIO错误", f"GPIO控制失败: {str(e)}")
        else:
            # 如果未连接，显示命令但不发送
            self.send_text.delete("1.0", tk.END)
            self.send_text.insert("1.0", command.strip())
            messagebox.showinfo("提示", f"串口未连接，已将命令填入发送区:\n{command.strip()}")


def main():
    root = tk.Tk()
    app = OrangePiDebugger(root)
    root.mainloop()


if __name__ == "__main__":
    main()