import os
import time
import random
from datetime import datetime

# 設定目標大小 (1GB = 1024 * 1024 * 1024 bytes)
TARGET_SIZE = 1 * 1024 * 1024 * 1024
FILENAME = "large_test_log.log"

# 定義一些模擬的 Log 訊息
LOG_LEVELS = ["INFO", "DEBUG", "WARN", "ERROR"]
MESSAGES = [
    "User login successful",
    "Database connection timeout",
    "File uploaded successfully",
    "Payment processing error: insufficient funds",
    "Cache refreshed",
    "API request received from 192.168.1.1"
]

def generate_large_log():
    start_time = time.time()
    current_size = 0
    
    # 為了效能，我們一次寫入一大塊 buffer，而不是一行一行寫
    buffer_size = 10000  # 每次累積 1萬行再一次寫入
    buffer = []
    
    with open(FILENAME, "w", encoding="utf-8") as f:
        print(f"開始生成 {FILENAME}，目標大小: 1GB...")
        
        while current_size < TARGET_SIZE:
            # 產生一行 Log
            timestamp = datetime.now().isoformat()
            level = random.choice(LOG_LEVELS)
            msg = random.choice(MESSAGES)
            log_line = f"[{timestamp}] [{level}] {msg} - TransactionID:{random.randint(1000,9999)}\n"
            
            buffer.append(log_line)
            
            if len(buffer) >= buffer_size:
                chunk = "".join(buffer)
                f.write(chunk)
                current_size += len(chunk.encode('utf-8'))
                buffer = [] # 清空 buffer
                
                # 顯示進度
                progress = (current_size / TARGET_SIZE) * 100
                print(f"\r進度: {progress:.2f}% ({current_size / (1024*1024):.2f} MB)", end="")

        # 寫入剩餘的 buffer
        if buffer:
            f.write("".join(buffer))
            
    print(f"\n完成！檔案已建立: {FILENAME}")
    print(f"耗時: {time.time() - start_time:.2f} 秒")

if __name__ == "__main__":
    generate_large_log()