use iced::{
    executor, Application, Command, Element, Settings, Theme,
    widget::{button, column, container, row, text,},
    alignment,
};

// --- (1) 應用程式狀態 Model ---
#[derive(Debug, Default)]
struct Calculator {
    display: String, // 顯示在螢幕上的字串
    first_operand: Option<f64>, // 儲存第一個運算數 (如果存在)
    operator: Option<char>, // 儲存待執行的運算符 (如果存在)
    awaiting_second: bool, // 標記是否在等待第二個運算數
}

// --- (2) 訊息 Message (使用者輸入/事件) ---
// 定義應用程式可能接收到的所有事件
#[derive(Debug, Clone)]
enum Message {
    DigitPressed(char),  // 數字按鈕被按下 (e.g., '1', '2')
    OperatorPressed(char), // 運算符號被按下 (e.g., '+', '*')
    EqualsPressed,       // 等號按鈕被按下
    ClearPressed,        // 清除按鈕被按下
}

// --- 核心計算邏輯 (從 CLI 範例重用) ---
// 執行實際的四則運算邏輯
fn calculate(num1: f64, num2: f64, operator: char) -> Result<f64, String> {
    match operator {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => {
            if num2 == 0.0 {
                Err("錯誤: 除數不能為零".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(format!("錯誤: 不支援的運算符號 '{}'", operator)),
    }
}


// --- (3) 應用程式實現 Application Trait ---
impl Application for Calculator {
    type Executor = executor::Default; // 執行器類型
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    // 初始化應用程式狀態
    fn new(_flags: ()) -> (Calculator, Command<Self::Message>) {
        (
            Calculator {
                display: "0".to_string(),
                ..Default::default()
            },
            Command::none(), // 初始化時無需執行命令
        )
    }

    // 應用程式標題
    fn title(&self) -> String {
        String::from("Rust Iced 計算機")
    }

    // --- Update 核心邏輯：處理訊息並更新 Model ---
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::DigitPressed(digit) => {
                if self.awaiting_second || self.display == "0" {
                    // 如果正在等待第二個數，或目前顯示為 0，則替換它
                    self.display = digit.to_string();
                    self.awaiting_second = false;
                } else {
                    // 否則，將數字附加到目前顯示
                    self.display.push(digit);
                }
            }
            Message::OperatorPressed(op) => {
                // 當按下運算符時，嘗試儲存第一個運算數
                if let Ok(num) = self.display.parse::<f64>() {
                    self.first_operand = Some(num);
                    self.operator = Some(op);
                    self.awaiting_second = true; // 準備接收第二個運算數
                }
            }
            Message::EqualsPressed => {
                // 按下等號，執行計算
                if let (Some(num1), Some(op), Ok(num2)) = (
                    self.first_operand,
                    self.operator,
                    self.display.parse::<f64>(),
                ) {
                    match calculate(num1, num2, op) {
                        Ok(result) => {
                            self.display = format!("{}", result);
                            // 重置狀態，等待新的計算開始
                            self.first_operand = None;
                            self.operator = None;
                            self.awaiting_second = true; // 計算結果顯示後，下一個數字應該替換它
                        }
                        Err(e) => {
                            self.display = e; // 在螢幕上顯示錯誤訊息
                            self.first_operand = None;
                            self.operator = None;
                            self.awaiting_second = true;
                        }
                    }
                }
            }
            Message::ClearPressed => {
                // 清除所有狀態
                *self = Calculator::default();
                self.display = "0".to_string();
            }
        }
        Command::none()
    }

    // --- View 核心邏輯：描述如何繪製 UI ---
    fn view(&self) -> Element<'_, Self::Message> {
        let display_text = text(&self.display)
            .size(40)
            .horizontal_alignment(alignment::Horizontal::Right);

        // 輔助函式，用於快速建立按鈕
        let btn = |label: &str, msg: Message| {
            button(text(label).size(25))
                .padding(15)
                .on_press(msg)
                .width(iced::Length::Fill)
        };

        let num_btn = |digit: char| btn(&digit.to_string(), Message::DigitPressed(digit));
        let op_btn = |op: char| btn(&op.to_string(), Message::OperatorPressed(op));

            // UI 佈局: 顯示螢幕 + 4 行按鈕
            column![
                // 第一行：顯示螢幕
                container(display_text)
                    .padding(10)
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fixed(80.0)),

            // 第二行：清除
            row![
                btn("C", Message::ClearPressed).width(iced::Length::FillPortion(3)),
                op_btn('/'),
            ]
            .spacing(10),

            // 第三行：7 8 9 *
            row![
                num_btn('7'),
                num_btn('8'),
                num_btn('9'),
                op_btn('*'),
            ]
            .spacing(10),

            // 第四行：4 5 6 -
            row![
                num_btn('4'),
                num_btn('5'),
                num_btn('6'),
                op_btn('-'),
            ]
            .spacing(10),

            // 第五行：1 2 3 +
            row![
                num_btn('1'),
                num_btn('2'),
                num_btn('3'),
                op_btn('+'),
            ]
            .spacing(10),

            // 第六行：0 . =
            row![
                num_btn('0').width(iced::Length::FillPortion(2)), // 讓 0 寬一點
                btn("=", Message::EqualsPressed).width(iced::Length::FillPortion(2)),
            ]
            .spacing(10),
        ]
        .padding(20)
        .spacing(10)
        .into() // 轉換為 Element
    }
}

// 主入口點
fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

// 您可以在終端機中執行 'cargo run' 來執行此程式