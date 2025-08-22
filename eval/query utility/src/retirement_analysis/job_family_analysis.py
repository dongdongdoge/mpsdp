import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns

def analyze_job_statistics():
    """
    按Job类型统计retirement数据
    """
    print("正在加载retirement数据...")
    
    # 读取过滤后的retirement数据
    df = pd.read_csv('Retirement/retirement_filtered.csv')
    
    print(f"数据形状: {df.shape}")
    print(f"总记录数: {len(df):,}")
    
    # 按Job类型统计
    print(f"\n=== 按Job类型统计 ===")
    job_stats = df.groupby('Job').agg({
        'Retirement': ['count', 'sum', 'mean', 'std', 'min', 'max']
    }).round(2)
    
    # 重命名列
    job_stats.columns = ['Count', 'Total_Retirement', 'Mean_Retirement', 'Std_Retirement', 'Min_Retirement', 'Max_Retirement']
    
    # 按记录数排序，显示前20个
    job_stats_sorted = job_stats.sort_values('Count', ascending=False)
    
    print(f"前20个Job类型统计:")
    print(job_stats_sorted.head(20))
    
    # 显示前5个Job类型
    top_5_jobs = job_stats_sorted.head(5)
    print(f"\n=== 前5个Job类型 ===")
    for i, (job_name, stats) in enumerate(top_5_jobs.iterrows(), 1):
        print(f"{i}. {job_name}")
        print(f"   记录数: {stats['Count']:,}")
        print(f"   总retirement: ${stats['Total_Retirement']:,.2f}")
        print(f"   平均retirement: ${stats['Mean_Retirement']:,.2f}")
        print(f"   标准差: ${stats['Std_Retirement']:,.2f}")
        print(f"   最小值: ${stats['Min_Retirement']:,.2f}")
        print(f"   最大值: ${stats['Max_Retirement']:,.2f}")
        print()
    
    return job_stats_sorted, df

def analyze_top_jobs_detailed(top_jobs, df):
    """
    对前5个Job类型进行详细分析
    """
    print(f"=== 前5个Job类型详细分析 ===")
    
    top_job_names = top_jobs.head(5).index.tolist()
    
    for job_name in top_job_names:
        print(f"\n--- {job_name} ---")
        
        # 获取该Job的数据
        job_data = df[df['Job'] == job_name]
        
        print(f"记录数: {len(job_data):,}")
        print(f"平均retirement: ${job_data['Retirement'].mean():,.2f}")
        print(f"中位数retirement: ${job_data['Retirement'].median():,.2f}")
        
        # 按年份分析
        print(f"\n按年份分析:")
        year_stats = job_data.groupby('Year')['Retirement'].agg(['count', 'mean', 'std']).round(2)
        print(year_stats)
        
        # 按组织组分析
        print(f"\n按组织组分析:")
        org_stats = job_data.groupby('Organization Group')['Retirement'].agg(['count', 'mean', 'std']).round(2)
        print(org_stats)
        
        # 按工会分析
        print(f"\n按工会分析 (前5个):")
        union_stats = job_data.groupby('Union')['Retirement'].agg(['count', 'mean', 'std']).round(2)
        union_stats = union_stats.sort_values('count', ascending=False).head(5)
        print(union_stats)

def visualize_job_statistics(job_stats):
    """
    可视化Job统计结果
    """
    print("生成Job统计可视化...")
    
    # 设置中文字体
    plt.rcParams['font.sans-serif'] = ['SimHei', 'Arial Unicode MS', 'DejaVu Sans']
    plt.rcParams['axes.unicode_minus'] = False
    
    # 创建图表
    fig, axes = plt.subplots(2, 2, figsize=(15, 12))
    
    # 获取前10个Job类型
    top_10_jobs = job_stats.head(10)
    
    # 1. 记录数对比
    job_names = [job[:30] + '...' if len(job) > 30 else job for job in top_10_jobs.index]
    counts = top_10_jobs['Count'].values
    
    axes[0, 0].barh(range(len(job_names)), counts, color='skyblue', alpha=0.8)
    axes[0, 0].set_yticks(range(len(job_names)))
    axes[0, 0].set_yticklabels(job_names)
    axes[0, 0].set_xlabel('记录数')
    axes[0, 0].set_title('前10个Job类型记录数')
    axes[0, 0].grid(True, alpha=0.3)
    
    # 2. 平均retirement对比
    means = top_10_jobs['Mean_Retirement'].values
    
    axes[0, 1].barh(range(len(job_names)), means, color='lightcoral', alpha=0.8)
    axes[0, 1].set_yticks(range(len(job_names)))
    axes[0, 1].set_yticklabels(job_names)
    axes[0, 1].set_xlabel('平均Retirement ($)')
    axes[0, 1].set_title('前10个Job类型平均Retirement')
    axes[0, 1].grid(True, alpha=0.3)
    
    # 3. 总retirement对比
    totals = top_10_jobs['Total_Retirement'].values / 1e6  # 转换为百万美元
    
    axes[1, 0].barh(range(len(job_names)), totals, color='lightgreen', alpha=0.8)
    axes[1, 0].set_yticks(range(len(job_names)))
    axes[1, 0].set_yticklabels(job_names)
    axes[1, 0].set_xlabel('总Retirement (百万美元)')
    axes[1, 0].set_title('前10个Job类型总Retirement')
    axes[1, 0].grid(True, alpha=0.3)
    
    # 4. 标准差对比
    stds = top_10_jobs['Std_Retirement'].values
    
    axes[1, 1].barh(range(len(job_names)), stds, color='gold', alpha=0.8)
    axes[1, 1].set_yticks(range(len(job_names)))
    axes[1, 1].set_yticklabels(job_names)
    axes[1, 1].set_xlabel('标准差 ($)')
    axes[1, 1].set_title('前10个Job类型Retirement标准差')
    axes[1, 1].grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.savefig('retirement_job_statistics.png', dpi=300, bbox_inches='tight')
    plt.show()
    
    return fig

def generate_job_tikz_code(job_stats):
    """
    生成Job统计的TikZ代码
    """
    # 获取前5个Job类型
    top_5_jobs = job_stats.head(5)
    
    tikz_code = r"""
\begin{figure}[htbp]
\centering
\begin{tikzpicture}
\begin{axis}[
    width=12cm,
    height=8cm,
    xlabel={Job Type},
    ylabel={Record Count},
    title={Top 5 Job Types by Record Count},
    grid=major,
    legend pos=north west,
    x tick label style={rotate=45, anchor=east},
    ybar,
    bar width=0.6
]

% 记录数数据
\addplot[fill=blue!60] coordinates {
"""
    
    for i, (job_name, stats) in enumerate(top_5_jobs.iterrows()):
        # 截断过长的Job名称
        short_name = job_name[:20] + '...' if len(job_name) > 20 else job_name
        count = stats['Count']
        tikz_code += f"    ({i+1}, {count})\n"
    
    tikz_code += r"""};
\addlegendentry{Record Count}

\end{axis}
\end{tikzpicture}
\caption{前5个Job类型记录数统计}
\label{fig:top_5_job_types}
\end{figure}
"""
    
    return tikz_code

def create_job_summary_table(job_stats):
    """
    创建Job统计总结表
    """
    print(f"\n=== Job统计总结表 ===")
    
    # 获取前10个Job类型
    top_10_jobs = job_stats.head(10)
    
    print(f"{'排名':<4} {'Job类型':<40} {'记录数':<10} {'平均Retirement':<15} {'总Retirement':<15}")
    print("-" * 90)
    
    for i, (job_name, stats) in enumerate(top_10_jobs.iterrows(), 1):
        short_name = job_name[:38] + '..' if len(job_name) > 40 else job_name
        count = f"{stats['Count']:,}"
        mean_ret = f"${stats['Mean_Retirement']:,.0f}"
        total_ret = f"${stats['Total_Retirement']/1e6:.1f}M"
        
        print(f"{i:<4} {short_name:<40} {count:<10} {mean_ret:<15} {total_ret:<15}")

def main():
    """
    主函数
    """
    print("开始Job类型统计分析...")
    
    # 分析Job统计
    job_stats, df = analyze_job_statistics()
    
    # 详细分析前5个Job类型
    analyze_top_jobs_detailed(job_stats, df)
    
    # 可视化结果
    visualize_job_statistics(job_stats)
    
    # 生成TikZ代码
    tikz_code = generate_job_tikz_code(job_stats)
    with open('retirement_job_tikz.tex', 'w') as f:
        f.write(tikz_code)
    
    # 创建总结表
    create_job_summary_table(job_stats)
    
    # 保存Job统计数据
    job_stats.to_csv('retirement_job_statistics.csv')
    
    print(f"\n分析完成！")
    print(f"- 图表已保存为: retirement_job_statistics.png")
    print(f"- TikZ代码已保存为: retirement_job_tikz.tex")
    print(f"- Job统计数据已保存为: retirement_job_statistics.csv")
    
    return job_stats

if __name__ == "__main__":
    job_stats = main() 