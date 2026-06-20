from bs4 import BeautifulSoup
import requests

# parse page
page_info = requests.get("https://ewwii-sh.github.io/docs/widgets/props")
page_html = page_info.text
soup = BeautifulSoup(page_html, 'html.parser')

# find docs
docs = soup.find('div', class_='theme-doc-markdown markdown')
header = docs.find('header')

with open("prepend.html") as f:
    html = BeautifulSoup(f.read(), 'html.parser')
    header.insert_after(html)

with open("src/config/supported_widgets.md", "w", encoding="utf-8") as f:
    f.write(docs.decode_contents())
