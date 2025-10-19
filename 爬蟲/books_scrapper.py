import requests
from bs4 import BeautifulSoup
import time
import re # 用於正規表達式清洗字串
import urllib.parse
from wcwidth import wcswidth

def pad_string(text, target_width):
    """
    使用 wcwidth 計算字符寬度，並返回左對齊的填充字符串。
    :param text: 要對齊的字符串 (包含中文)
    :param target_width: 期望的顯示寬度 (以半形字符為單位)
    :return: 填充後的字符串
    """
    # 計算字符串的實際顯示寬度
    current_width = wcswidth(text)
    
    # 計算需要填充的空格數量
    padding_needed = target_width - current_width
    
    if padding_needed > 0:
        return text + ' ' * padding_needed
    else:
        # 如果字符串比目標寬度長，則直接返回（或根據需求截斷）
        return text
def parse_book_item(item_html):

    """
    從單本書籍的 HTML 區塊中解析書名和價格。
    :param item_html: 單本書籍的 HTML 字符串 (或 BeautifulSoup tag 對象)
    :return: 包含書名、折扣和價格的字典
    """
    soup = BeautifulSoup(item_html, 'html.parser')
    
    # 1. 提取書名 (Title)
    # 尋找 <a> 標籤
    title_tag = soup.find('a', title=True)
    title = title_tag['title'] if title_tag else "0"
    if title == "下次再買" or title == "可訂購時通知我" or title == "0":
        return {
            "書名": "0",
            "折扣": 0,
            "價格": f"{0} 元"
        }
    # 2. 提取價格資訊 (Price)
    # 尋找 ul class="price" 內的 li 標籤
    price_li = soup.find('ul', class_='price').find('li') if soup.find('ul', class_='price') else None

    if price_li:
        # 提取整個文本，例如 "優惠價: 9 折, 126 元"
        full_price_text = price_li.text.strip()
        
        # 使用正規表達式或字串分割來提取 折扣 和 價格
        
        # 新規則：第一個 <b> 為折扣，第二個 <b> 為價格
        b_tags = price_li.find_all('b')

        # 預設值
        discount = "N/A"
        price_value = "N/A (價格)"

        if b_tags:
            # 第一個 <b> 當作折扣（例如 '9' 代表 9 折）
            discount = b_tags[0].text.strip()

            # 第二個 <b> 當作價格 (如果存在)
            if len(b_tags) > 1:
                second_b_text = b_tags[1].text.strip()
                # 清理價格中的非數字字元，保留數字
                price_digits = re.search(r"(\d+[\d,]*)", second_b_text)
                price_value = price_digits.group(1).replace(',', '') if price_digits else second_b_text
            else:
                price_value = discount  # 若無第二個 <b>，將價格設為折扣的值（或其他預設值）
                discount = "N/A"

        
        else:
            # 若沒有 <b>，嘗試從整段文字中提取折扣與價格作為備援
            discount_match = re.search(r'(\d+)[\s]*折', full_price_text)
            discount = f"{discount_match.group(1)} 折" if discount_match else discount
            price_match = re.search(r'(\d+[\d,]*)\s*元', full_price_text)
            price_value = price_match.group(1).replace(',', '') if price_match else price_value

    else:
        discount = "N/A (折扣)"
        price_value = "N/A (價格)"
    
    return {
        "書名": title,
        "折扣": f"{discount}折",
        "價格": f"{price_value}元"
    }
def scrape_book_series(series_url):
    """
    爬取特定書籍系列頁面上的所有書籍資訊。
    """
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
    }

    try:
        response = requests.get(series_url, headers=headers, timeout=10)
        response.raise_for_status()
        response.encoding = 'utf-8'

        soup = BeautifulSoup(response.text, 'html.parser')

        # ！！這是關鍵步驟：您需要在目標網頁上找到包含單本書籍資訊的最小重複區塊 ！！
        # 該 <div> 裡面包含了您提供的 <a> 和 <ul> 結構
        book_items = soup.find_all('div', class_='table-td') 

        if not book_items:
            # 嘗試其他常見的選擇器，或請您提供目標網址讓我確認
            book_items = soup.find_all('div', class_='box') 

        if not book_items:
            print("警告：未找到書籍項目，請檢查 HTML 選擇器是否正確。")
            return []

        all_books_data = []
        for item in book_items:
            # 將每個完整的書籍區塊傳入解析函數
            book_data = parse_book_item(str(item)) 
            all_books_data.append(book_data)
            # print(f"已爬取: {book_data['書名']} | {book_data['價格']}")
            
        return all_books_data

    except requests.exceptions.RequestException as e:
        print(f"請求錯誤: {e}")
        return []
    except Exception as e:
        print(f"發生解析錯誤: {e}")
        return []
def printfunction():
    # 1. 定義每個欄位的目標顯示寬度 (以半形字符為單位)
    TITLE_WIDTH = 60 # 書名欄位寬度
    DISCOUNT_WIDTH = 16 # 折扣欄位寬度
    PRICE_WIDTH = 8   # 價格欄位寬度

    # 2. 輸出標題
    header_title = pad_string('書名', TITLE_WIDTH)
    header_discount = pad_string('折扣', DISCOUNT_WIDTH)
    header_price = pad_string('價格', PRICE_WIDTH)

    header = f"{header_title} | {header_discount} | {header_price}"
    print(header)

    total_length = TITLE_WIDTH + DISCOUNT_WIDTH + PRICE_WIDTH + 6 # 6 是分隔符號和空格的長度
    print("-" * total_length) 

    # 3. 輸出結果
    for book in results:
        # 根據您的原始邏輯過濾
        if book['書名'] != "0" and book['書名'] != "可訂購時通知我":
            
            # 處理各個欄位
            title_padded = pad_string(book['書名'], TITLE_WIDTH)
            discount_padded = pad_string(book['折扣'], DISCOUNT_WIDTH)
            price_padded = pad_string(book['價格'], PRICE_WIDTH)
            
            print(f"{title_padded} | {discount_padded} | {price_padded}")

# ----------------------------------------------------
# 最終執行：請替換成您的系列網址
# ----------------------------------------------------
original_string = "我和班上第二可愛的女生成為朋友"

target_series_url = f"https://search.books.com.tw/search/query/key/{urllib.parse.quote(original_string)}/cat/all"
results = scrape_book_series(target_series_url)
print("\n--- 爬取完成結果 ---")

"""
for book in results:
    if book['書名'] != "0" and book['書名'] != "可訂購時通知我":
        print(book)
"""
printfunction()
