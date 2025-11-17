# calcator CLI

```rust

use std::io::{self, Write}; // 導入標準輸入/輸出模組

/// 執行實際的四則運算邏輯
/// 接收兩個 f64 浮點數和一個 char 運算符
/// 成功時返回 Result<f64, String> (f64 結果或錯誤訊息)
fn calculate(num1: f64, num2: f64, operator: char) -> Result<f64, String> {
    match operator {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => {
            if num2 == 0.0 {
                // 處理除以零的錯誤情況
                Err("錯誤：除數不能為零".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(format!("錯誤：不支援的運算符號 '{}'", operator)),
    }
}

/// 解析輸入字串，假設輸入格式為 "數字 運算符 數字" (例如: 5 + 3)
fn parse_input(input: &str) -> Result<(f64, char, f64), String> {
    // 將輸入字串按空白符切割成多個部分
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        return Err("輸入格式錯誤。請使用 '數字 運算符 數字' 格式 (例如: 5 + 3)".to_string());
    }

    // 嘗試將第一個部分解析為 f64 數字
    let num1 = parts[0].parse::<f64>()
        .map_err(|_| format!("錯誤：無法解析第一個數字 '{}'", parts[0]))?;

    // 嘗試取得運算符號 (預期為單一字元)
    let operator_str = parts[1];
    let operator = operator_str.chars().next()
        .filter(|_c| operator_str.len() == 1) // 確保只有一個字元
        .ok_or_else(|| format!("錯誤：運算符號格式錯誤 '{}'", operator_str))?;

    // 嘗試將第三個部分解析為 f64 數字
    let num2 = parts[2].parse::<f64>()
        .map_err(|_| format!("錯誤：無法解析第二個數字 '{}'", parts[2]))?;

    Ok((num1, operator, num2))
}

fn main() {
    println!("--- 🦀 簡易 CLI 計算機 ---");
    println!("請輸入運算式，格式為: 數字 運算符 數字 (例如: 5 + 3)");
    println!("支援運算符: +, -, *, /");
    println!("輸入 'quit' 或 'q' 退出程式");

    // 開始事件循環
    loop {
        print!("> ");
        // 確保提示符號立即顯示
        io::stdout().flush().expect("無法清空緩衝區"); 

        let mut input = String::new();

        // 讀取使用者輸入
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim(); // 移除首尾空白和換行符

                // 檢查是否為退出指令
                if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("q") {
                    println!("程式退出。");
                    break;
                }

                // 核心邏輯：解析輸入 -> 執行計算 -> 顯示結果
                match parse_input(input) {
                    Ok((num1, operator, num2)) => {
                        match calculate(num1, num2, operator) {
                            Ok(result) => {
                                println!("結果: {}", result);
                            }
                            // 處理 calculate 函式返回的錯誤 (例如除以零)
                            Err(e) => {
                                eprintln!("{}", e); // 使用 eprintln 輸出錯誤訊息到標準錯誤
                            }
                        }
                    }
                    // 處理 parse_input 函式返回的錯誤 (例如格式錯誤)
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            Err(error) => {
                eprintln!("讀取輸入時發生錯誤: {}", error);
                // 發生嚴重 I/O 錯誤時可以選擇退出
            }
        }
    }
}
// 您可以在終端機中執行 'cargo run' 來執行此程式

```
