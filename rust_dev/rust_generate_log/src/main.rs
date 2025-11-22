use chrono::Utc;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::{self, Write, BufWriter};
use std::time::Instant;

const TARGET_SIZE: u64 = 1 * 1024 * 1024 * 1024; // 1 GB
const FILENAME: &str = "large_test_log.log";
const BUFFER_LINES: usize = 10_000; // accumulate this many lines before writing

static LOG_LEVELS: &[&str] = &["INFO", "DEBUG", "WARN", "ERROR"];
static MESSAGES: &[&str] = &[
    "User login successful",
    "Database connection timeout",
    "File uploaded successfully",
    "Payment processing error: insufficient funds",
    "Cache refreshed",
    "API request received from 192.168.1.1",
];

fn main() -> io::Result<()> {
    let start = Instant::now();
    let mut current_size: u64 = 0;

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(FILENAME)?;

    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, file);

    println!("開始生成 {}，目標大小: 1GB...", FILENAME);

    let mut rng = rand::thread_rng();
    let mut buffer = String::with_capacity(BUFFER_LINES * 120);
    let mut lines_in_buffer = 0usize;

    while current_size < TARGET_SIZE {
        let timestamp = Utc::now().to_rfc3339();
        let level = LOG_LEVELS.choose(&mut rng).unwrap();
        let msg = MESSAGES.choose(&mut rng).unwrap();
        let tx = rng.gen_range(1000..=9999);
        let line = format!("[{}] [{}] {} - TransactionID:{}\n", timestamp, level, msg, tx);

        buffer.push_str(&line);
        lines_in_buffer += 1;

        if lines_in_buffer >= BUFFER_LINES {
            let bytes = buffer.as_bytes();
            writer.write_all(bytes)?;
            writer.flush()?;

            current_size += bytes.len() as u64;
            buffer.clear();
            lines_in_buffer = 0;

            let progress = (current_size as f64 / TARGET_SIZE as f64) * 100.0;
            print!("\r進度: {progress:.2}% ({:.2} MB)", current_size as f64 / (1024.0*1024.0));
            io::stdout().flush()?;
        }
    }

    // 如果有剩下的 buffer，補寫一次
    if !buffer.is_empty() {
        let bytes = buffer.as_bytes();
        writer.write_all(bytes)?;
        writer.flush()?;
        current_size += bytes.len() as u64;
    }

    // 最後確認檔案大小至少達到目標（在某些情況下最後一次寫入已超過目標）
    println!("\n完成！檔案已建立: {}", FILENAME);
    println!("最終大小: {:.2} MB", current_size as f64 / (1024.0*1024.0));
    println!("耗時: {:.2} 秒", start.elapsed().as_secs_f64());

    Ok(())
}
