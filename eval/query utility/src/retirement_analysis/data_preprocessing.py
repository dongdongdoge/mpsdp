import pandas as pd
import numpy as np

def filter_retirement_data():
    """
    从employee compensation数据集中过滤出retirement > 0的所有行
    """
    print("正在加载employee compensation数据集...")
    
    # 读取数据
    df = pd.read_csv('Retirement/employee-compensation.csv')
    
    print(f"原始数据形状: {df.shape}")
    print(f"列名: {list(df.columns)}")
    
    # 检查Retirement列的数据类型和基本统计
    print(f"\nRetirement列数据类型: {df['Retirement'].dtype}")
    print(f"Retirement列基本统计:")
    print(df['Retirement'].describe())
    
    # 过滤retirement > 0的数据
    retirement_filtered = df[df['Retirement'] > 0].copy()
    
    print(f"\n过滤后数据形状: {retirement_filtered.shape}")
    print(f"过滤条件: Retirement > 0")
    print(f"保留行数: {len(retirement_filtered)} / {len(df)} ({len(retirement_filtered)/len(df)*100:.2f}%)")
    
    # 显示过滤后Retirement列的基本统计
    print(f"\n过滤后Retirement列基本统计:")
    print(retirement_filtered['Retirement'].describe())
    
    # 保存过滤后的数据
    output_file = 'Retirement/retirement_filtered.csv'
    retirement_filtered.to_csv(output_file, index=False)
    print(f"\n过滤后的数据已保存到: {output_file}")
    
    # 显示前几行数据
    print(f"\n前5行数据:")
    print(retirement_filtered.head())
    
    # 分析不同组织组的retirement情况
    print(f"\n各组织组的retirement统计:")
    org_retirement = retirement_filtered.groupby('Organization Group')['Retirement'].agg(['count', 'mean', 'std', 'min', 'max'])
    print(org_retirement)
    
    # 分析不同年份的retirement情况
    print(f"\n各年份的retirement统计:")
    year_retirement = retirement_filtered.groupby('Year')['Retirement'].agg(['count', 'mean', 'std', 'min', 'max'])
    print(year_retirement)
    
    return retirement_filtered

def analyze_retirement_distribution(retirement_data):
    """
    分析retirement数据的分布
    """
    print(f"\n=== Retirement数据分布分析 ===")
    
    # 基本统计
    retirement_values = retirement_data['Retirement']
    print(f"总记录数: {len(retirement_values)}")
    print(f"平均值: ${retirement_values.mean():,.2f}")
    print(f"中位数: ${retirement_values.median():,.2f}")
    print(f"标准差: ${retirement_values.std():,.2f}")
    print(f"最小值: ${retirement_values.min():,.2f}")
    print(f"最大值: ${retirement_values.max():,.2f}")
    
    # 分位数分析
    percentiles = [10, 25, 50, 75, 90, 95, 99]
    print(f"\n分位数分析:")
    for p in percentiles:
        value = retirement_values.quantile(p/100)
        print(f"{p}th percentile: ${value:,.2f}")
    
    # 分布可视化数据
    print(f"\n分布区间统计:")
    bins = [0, 5000, 10000, 15000, 20000, 25000, 30000, 35000, 40000, float('inf')]
    labels = ['0-5K', '5K-10K', '10K-15K', '15K-20K', '20K-25K', '25K-30K', '30K-35K', '35K-40K', '40K+']
    
    retirement_data['Retirement_Bin'] = pd.cut(retirement_data['Retirement'], bins=bins, labels=labels, include_lowest=True)
    bin_counts = retirement_data['Retirement_Bin'].value_counts().sort_index()
    
    for bin_label, count in bin_counts.items():
        percentage = count / len(retirement_data) * 100
        print(f"{bin_label}: {count} records ({percentage:.1f}%)")

def create_summary_report(retirement_data):
    """
    创建总结报告
    """
    print(f"\n=== Retirement数据总结报告 ===")
    
    # 基本统计
    total_records = len(retirement_data)
    total_retirement = retirement_data['Retirement'].sum()
    avg_retirement = retirement_data['Retirement'].mean()
    
    print(f"总记录数: {total_records:,}")
    print(f"总retirement金额: ${total_retirement:,.2f}")
    print(f"平均retirement: ${avg_retirement:,.2f}")
    
    # 按组织组统计
    print(f"\n按组织组统计:")
    org_stats = retirement_data.groupby('Organization Group').agg({
        'Retirement': ['count', 'sum', 'mean', 'std']
    }).round(2)
    org_stats.columns = ['Count', 'Total', 'Mean', 'Std']
    print(org_stats)
    
    # 按年份统计
    print(f"\n按年份统计:")
    year_stats = retirement_data.groupby('Year').agg({
        'Retirement': ['count', 'sum', 'mean', 'std']
    }).round(2)
    year_stats.columns = ['Count', 'Total', 'Mean', 'Std']
    print(year_stats)
    
    # 按工会统计
    print(f"\n按工会统计 (前10个):")
    union_stats = retirement_data.groupby('Union').agg({
        'Retirement': ['count', 'sum', 'mean', 'std']
    }).round(2)
    union_stats.columns = ['Count', 'Total', 'Mean', 'Std']
    # 修复排序问题
    union_stats = union_stats.sort_values('Count', ascending=False).head(10)
    print(union_stats)

if __name__ == "__main__":
    # 过滤数据
    retirement_data = filter_retirement_data()
    
    # 分析分布
    analyze_retirement_distribution(retirement_data)
    
    # 创建总结报告
    create_summary_report(retirement_data)
    
    print(f"\n处理完成！")
    print(f"- 过滤后的数据已保存到: Retirement/retirement_filtered.csv")
    print(f"- 总记录数: {len(retirement_data):,}")
    print(f"- 平均retirement: ${retirement_data['Retirement'].mean():,.2f}") 