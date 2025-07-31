import pandas as pd
from datetime import datetime

# 读取数据
file = 'Fire/Fire_alarm.csv'
df = pd.read_csv(file)

# 转换为datetime
received = pd.to_datetime(df['Received DtTm'], format='%m/%d/%Y %I:%M:%S %p')
dispatch = pd.to_datetime(df['Dispatch DtTm'], format='%m/%d/%Y %I:%M:%S %p')

# 计算时间差（分钟）
delta_min = (dispatch - received).dt.total_seconds() / 60

# 统计差值在1分钟以内的数量
within_1min = (delta_min <= 1).sum()
total = len(df)

print(f"Number of records with dispatch-received <= 1 min: {within_1min}")
print(f"Total records: {total}")
print(f"Proportion: {within_1min/total:.4f}") 