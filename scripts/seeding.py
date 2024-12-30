import sqlite3
import ulid
import os
from dotenv import load_dotenv

load_dotenv()
DB_PATH = os.getenv('DB_PATH', 'example.db')

conn = sqlite3.connect(DB_PATH)
cursor = conn.cursor()

sector_groups = [
    (1, '食品'),
    (2, 'エネルギー資源'),
    (3, '建設・資材'),
    (4, '素材・化学'),
    (5, '医薬品'),
    (6, '自動車・輸送機'),
    (7, '鉄鋼・非鉄'),
    (8, '機械'),
    (9, '電機・精密'),
    (10, '情報通信・サービスその他'),
    (11, '電気・ガス'),
    (12, '運輸・物流'),
    (13, '商社・卸売'),
    (14, '小売'),
    (15, '銀行'),
    (16, '金融（除く銀行）'),
    (17, '不動産'),
]


def insert_sector_groups():
    inserting = list(map(lambda x: (str(ulid.new()), *x), sector_groups))
    cursor.executemany(
        'INSERT INTO sector_groups (id, code, name) VALUES (?, ?, ?)',
        inserting
    )
    conn.commit()
    return inserting


categories = [
    ('シクリカル'),
    ('セミシクリカル'),
    ('ディフェンシブ'),
]


def insert_categories():
    inserting = list(map(lambda x: (str(ulid.new()), x), categories))
    cursor.executemany(
        'INSERT INTO categories (id, name) VALUES (?, ?)', inserting)
    conn.commit()
    return inserting


sectors = [
    ('0050', '水産・農林業', 1, 'ディフェンシブ'),
    ('1050', '鉱業', 2, 'シクリカル'),
    ('2050', '建設業', 3, 'シクリカル'),
    ('3050', '食料品', 1, 'ディフェンシブ'),
    ('3100', '繊維製品', 4, 'シクリカル'),
    ('3150', 'パルプ・紙', 4, 'シクリカル'),
    ('3200', '化学', 4, 'シクリカル'),
    ('3250', '医薬品', 5, 'ディフェンシブ'),
    ('3300', '石油･石炭製品', 2, 'シクリカル'),
    ('3350', 'ゴム製品', 6, 'シクリカル'),
    ('3400', 'ガラス･土石製品', 3, 'シクリカル'),
    ('3450', '鉄鋼', 7, 'シクリカル'),
    ('3500', '非鉄金属', 7, 'シクリカル'),
    ('3550', '金属製品', 3, 'シクリカル'),
    ('3600', '機械', 8, 'シクリカル'),
    ('3650', '電気機器', 9, 'シクリカル'),
    ('3700', '輸送用機器', 6, 'シクリカル'),
    ('3750', '精密機器', 9, 'シクリカル'),
    ('3800', 'その他製品', 10, 'シクリカル'),
    ('4050', '電気･ガス業', 11, 'ディフェンシブ'),
    ('5050', '陸運業', 12, 'ディフェンシブ'),
    ('5100', '海運業', 12, 'シクリカル'),
    ('5150', '空運業', 12, 'シクリカル'),
    ('5200', '倉庫･運輸関連業', 12, 'セミシクリカル'),
    ('5250', '情報･通信業', 10, 'ディフェンシブ'),
    ('6050', '卸売業', 13, 'シクリカル'),
    ('6100', '小売業', 14, 'セミシクリカル'),
    ('7050', '銀行業', 15, 'セミシクリカル'),
    ('7100', '証券･商品先物取引業', 16, 'シクリカル'),
    ('7150', '保険業', 16, 'セミシクリカル'),
    ('7200', 'その他金融業', 16, 'セミシクリカル'),
    ('8050', '不動産業', 17, 'セミシクリカル'),
    ('9050', 'サービス業', 10, 'セミシクリカル'),
    ('9999', 'その他', 10, 'シクリカル'),
]


def get_sector_group_id_from_code(code, sector_groups):
    for group in sector_groups:
        if group[1] == code:
            return group[0]


def get_category_id_from_name(name, categories):
    for category in categories:
        if category[1] == name:
            return category[0]


def create_sector_inserting(sector, sector_groups, categories):
    group_id = get_sector_group_id_from_code(sector[2], sector_groups)
    category_id = get_category_id_from_name(sector[3], categories)
    return (str(ulid.new()), sector[0], sector[1], group_id, category_id)


def insert_sectors(sector_groups, categories):
    inserting = list(map(lambda x: create_sector_inserting(
        x, sector_groups, categories), sectors))
    cursor.executemany(
        """INSERT INTO sectors
            (id, code, name, sector_group_id, category_id) 
            VALUES
            (?, ?, ?, ?, ?)""",
        inserting
    )
    conn.commit()


inserted_sector_groups = insert_sector_groups()
inserted_categories = insert_categories()
inserted_sectors = insert_sectors(inserted_sector_groups, inserted_categories)

# Query the database
select_query = """
  SELECT 
    sectors.id, 
    sectors.name,
    sectors.code,
    sector_groups.name AS sector_group_name, 
    sector_groups.code AS sector_group_code,
    categories.name AS category_name
  FROM sectors
  LEFT JOIN sector_groups ON sectors.sector_group_id = sector_groups.id
  LEFT JOIN categories ON sectors.category_id = categories.id
"""
cursor.execute(select_query)
rows = cursor.fetchall()
for row in rows:
    print(row)

# Close the connection
conn.close()
