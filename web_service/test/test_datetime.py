from datetime import datetime, date, timedelta

print("今天的日期是:", date.today())
print("日期是:", datetime(2021, 6, 8))
print(datetime.now() - timedelta(days=1))
