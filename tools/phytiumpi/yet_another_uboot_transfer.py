#!/usr/bin/env python3

#↑兄啊你不知道uboot可以保存启动配置的嘛？设置为默认等待从串口传入内核就好了啊?
import sys
import time
import serial
from xmodem import XMODEM

def send_file(port, baudrate, file_path):
    # 打开串口
    ser = serial.Serial(port, baudrate, timeout=1)
    
    # 等待 U-Boot 提示符
    while True:
        line = ser.readline().decode('utf-8', errors='ignore').strip()
        print(line)
        if line.endswith('Phytium-Pi#'):
            break
    
    # 发送 loady 命令
    ser.write(b'loadx 0x90100000\n')
    time.sleep(0.5)
    
    # 等待 U-Boot 准备好接收文件
    while True:
        line = ser.readline().decode('utf-8', errors='ignore').strip()
        print(line)
        if 'Ready for binary' in line:
            break
    
    # 发送 'C' 字符开始传输
    ser.write(b'C')
    
    # 使用 xmodem 协议传输文件
    with open(file_path, 'rb') as f:
        def getc(size, timeout=1):
            return ser.read(size) or None

        def putc(data, timeout=1):
            return ser.write(data)

        modem = XMODEM(getc, putc)
        modem.send(f)
    
    # 关闭串口
    ser.close()

if __name__ == '__main__':
    if len(sys.argv) != 4:
        print("Usage: python script.py <port> <baudrate> <file_path>")
        sys.exit(1)
    
    port = sys.argv[1]
    baudrate = int(sys.argv[2])
    file_path = sys.argv[3]
    
    send_file(port, baudrate, file_path)
