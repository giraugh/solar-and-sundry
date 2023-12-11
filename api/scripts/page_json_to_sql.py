#! /bin/sh python3
# Converts page json to sql format
# Usage: ./page_json_to_sql.py my_file.json > out.sql

import json
import sys

# Determine file path
json_file_path = sys.argv[1]
sql_start = "INSERT INTO page (page_number, chapter_number, image_id, name, published_at) VALUES "
sql_values = []

with open(json_file_path, 'r') as f:
    contents = f.read()
    data = json.loads(contents)
    for page in data:
        sql_values.append(f"({page['page_number']}, {page['chapter_number']}, \'{page['image_id']}\', \'{page['name']}\', \'{page['published_status']['PublishedAt']}\')")

print(f'{sql_start}{",".join(sql_values)};')
