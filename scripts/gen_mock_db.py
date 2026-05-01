"""
60 日分のランダムなキーストロークデータを生成し mock/keystroke.db に保存する。

Usage:
    python scripts/gen_mock_db.py
"""

import os
import random
import sqlite3
from datetime import datetime, timedelta, timezone

JST = timezone(timedelta(hours=9))
DAYS = 60
OUTPUT_PATH = os.path.join(os.path.dirname(__file__), "..", "mock", "keystroke.db")


def generate_records(base_date: datetime) -> list[tuple[str, int]]:
    """1 日分のレコード一覧を返す。各レコードは (recorded_at, minute_count) のタプル。"""
    records = []
    for hour in range(24):
        for minute in range(60):
            count = random.randint(0, 300)
            if count == 0:
                continue
            dt = base_date.replace(hour=hour, minute=minute, second=0, microsecond=0)
            records.append((dt.isoformat(), count))
    return records


def main() -> None:
    os.makedirs(os.path.dirname(OUTPUT_PATH), exist_ok=True)

    if os.path.exists(OUTPUT_PATH):
        os.remove(OUTPUT_PATH)

    conn = sqlite3.connect(OUTPUT_PATH)
    conn.execute(
        """
        CREATE TABLE keystroke_logs (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            recorded_at  TEXT    NOT NULL,
            minute_count INTEGER NOT NULL
        )
        """
    )

    today = datetime.now(JST).replace(hour=0, minute=0, second=0, microsecond=0)
    start_date = today - timedelta(days=DAYS - 1)

    all_records: list[tuple[str, int]] = []
    for i in range(DAYS):
        day = start_date + timedelta(days=i)
        all_records.extend(generate_records(day))

    conn.executemany(
        "INSERT INTO keystroke_logs (recorded_at, minute_count) VALUES (?, ?)",
        all_records,
    )
    conn.commit()
    conn.close()

    print(f"生成完了: {len(all_records)} レコード → {os.path.abspath(OUTPUT_PATH)}")


if __name__ == "__main__":
    main()
