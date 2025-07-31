import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns

def visualize_rr_comparison_corrected():
    """
    可视化比较amp-sdp和network shuffling的RR扰动效果（修正版）
    """
    # 读取分析报告
    report_df = pd.read_csv('fire_alarm_rr_analysis_corrected.csv')
    
    # 分离两种技术的数据
    amp_data = report_df[report_df['Technique'] == 'amp-sdp']
    net_data = report_df[report_df['Technique'] == 'network_shuffling']
    
    # 设置中文字体
    plt.rcParams['font.sans-serif'] = ['SimHei', 'Arial Unicode MS', 'DejaVu Sans']
    plt.rcParams['axes.unicode_minus'] = False
    
    # 创建图表
    fig, axes = plt.subplots(2, 2, figsize=(15, 12))
    
    # 1. 精度对比（以epsilon为x轴）
    axes[0, 0].plot(amp_data['Epsilon'], amp_data['Precision'], 'bo-', 
                     linewidth=2, markersize=8, label='amp-sdp', alpha=0.8)
    axes[0, 0].plot(net_data['Epsilon'], net_data['Precision'], 'ro-', 
                     linewidth=2, markersize=8, label='network shuffling', alpha=0.8)
    axes[0, 0].set_xlabel('Epsilon (ε)')
    axes[0, 0].set_ylabel('Precision')
    axes[0, 0].set_title('RR扰动精度对比')
    axes[0, 0].legend()
    axes[0, 0].grid(True, alpha=0.3)
    axes[0, 0].set_ylim(0.99, 1.01)
    
    # 2. 准确率对比（以epsilon为x轴）
    axes[0, 1].plot(amp_data['Epsilon'], amp_data['Accuracy'], 'bo-', 
                     linewidth=2, markersize=8, label='amp-sdp', alpha=0.8)
    axes[0, 1].plot(net_data['Epsilon'], net_data['Accuracy'], 'ro-', 
                     linewidth=2, markersize=8, label='network shuffling', alpha=0.8)
    axes[0, 1].set_xlabel('Epsilon (ε)')
    axes[0, 1].set_ylabel('Accuracy')
    axes[0, 1].set_title('RR扰动准确率对比')
    axes[0, 1].legend()
    axes[0, 1].grid(True, alpha=0.3)
    axes[0, 1].set_ylim(0.99, 1.01)
    
    # 3. 扰动后比例对比（以epsilon为x轴）
    axes[1, 0].plot(amp_data['Epsilon'], amp_data['Perturbed_Proportion_1'], 'bo-', 
                     linewidth=2, markersize=8, label='amp-sdp', alpha=0.8)
    axes[1, 0].plot(net_data['Epsilon'], net_data['Perturbed_Proportion_1'], 'ro-', 
                     linewidth=2, markersize=8, label='network shuffling', alpha=0.8)
    # 添加原始比例的水平线
    original_prop = amp_data['Original_Proportion_1'].iloc[0]
    axes[1, 0].axhline(y=original_prop, color='green', linestyle='--', 
                        label=f'Original ({original_prop:.4f})', alpha=0.7)
    axes[1, 0].set_xlabel('Epsilon (ε)')
    axes[1, 0].set_ylabel('Proportion (Binary=1)')
    axes[1, 0].set_title('扰动后比例对比')
    axes[1, 0].legend()
    axes[1, 0].grid(True, alpha=0.3)
    
    # 4. 误差分析（以epsilon为x轴）
    amp_errors = np.abs(amp_data['Perturbed_Proportion_1'] - amp_data['Original_Proportion_1'])
    net_errors = np.abs(net_data['Perturbed_Proportion_1'] - net_data['Original_Proportion_1'])
    
    axes[1, 1].plot(amp_data['Epsilon'], amp_errors, 'bo-', 
                     linewidth=2, markersize=8, label='amp-sdp', alpha=0.8)
    axes[1, 1].plot(net_data['Epsilon'], net_errors, 'ro-', 
                     linewidth=2, markersize=8, label='network shuffling', alpha=0.8)
    axes[1, 1].set_xlabel('Epsilon (ε)')
    axes[1, 1].set_ylabel('Absolute Error')
    axes[1, 1].set_title('扰动误差分析')
    axes[1, 1].legend()
    axes[1, 1].grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.savefig('fire_alarm_rr_comparison_corrected.png', dpi=300, bbox_inches='tight')
    plt.show()
    
    return report_df

def generate_tikz_code_corrected(report_df):
    """
    生成TikZ代码用于LaTeX文档（修正版）
    """
    # 分离两种技术的数据
    amp_data = report_df[report_df['Technique'] == 'amp-sdp']
    net_data = report_df[report_df['Technique'] == 'network_shuffling']
    
    tikz_code = r"""
\begin{figure}[htbp]
\centering
\begin{tikzpicture}
\begin{axis}[
    width=12cm,
    height=8cm,
    xlabel={Epsilon ($\varepsilon$)},
    ylabel={Precision},
    title={Fire Alarm Binary RR Analysis (Corrected)},
    grid=major,
    legend pos=north west,
    xmin=0,
    ymin=0.99,
    ymax=1.01
]

% amp-sdp精度数据点
\addplot[blue, thick, mark=*, mark size=3pt] coordinates {
"""
    
    for _, row in amp_data.iterrows():
        eps = row['Epsilon']
        prec = row['Precision']
        tikz_code += f"    ({eps}, {prec:.6f})\n"
    
    tikz_code += r"""};
\addlegendentry{amp-sdp}

% network shuffling精度数据点
\addplot[red, thick, mark=square, mark size=3pt] coordinates {
"""
    
    for _, row in net_data.iterrows():
        eps = row['Epsilon']
        prec = row['Precision']
        tikz_code += f"    ({eps}, {prec:.6f})\n"
    
    tikz_code += r"""};
\addlegendentry{network shuffling}

\end{axis}
\end{tikzpicture}
\caption{Fire Alarm二进制数据RR扰动精度对比（修正版）}
\label{fig:fire_alarm_rr_analysis_corrected}
\end{figure}
"""
    
    return tikz_code

def create_summary_report_corrected(report_df):
    """
    创建总结报告（修正版）
    """
    print("\n=== Fire Alarm RR扰动总结报告（修正版）===")
    print(f"数据集大小: 112,436 条记录")
    print(f"原始Binary=1的比例: {report_df['Original_Proportion_1'].iloc[0]:.4f}")
    print(f"原始Binary=1的记录数: {report_df['Original_Count_1'].iloc[0]:,}")
    
    # 分离两种技术的数据
    amp_data = report_df[report_df['Technique'] == 'amp-sdp']
    net_data = report_df[report_df['Technique'] == 'network_shuffling']
    
    print(f"\n=== amp-sdp 结果 ===")
    for _, row in amp_data.iterrows():
        eps = row['Epsilon']
        eps_prime = row['Epsilon_Prime']
        acc = row['Accuracy']
        prec = row['Precision']
        perturbed_prop = row['Perturbed_Proportion_1']
        print(f"ε={eps}, ε'={eps_prime}: 准确率={acc:.4f}, 精度={prec:.4f}, 扰动后比例={perturbed_prop:.4f}")
    
    print(f"\n=== network shuffling 结果 ===")
    for _, row in net_data.iterrows():
        eps = row['Epsilon']
        eps_prime = row['Epsilon_Prime']
        acc = row['Accuracy']
        prec = row['Precision']
        perturbed_prop = row['Perturbed_Proportion_1']
        print(f"ε={eps}, ε'={eps_prime}: 准确率={acc:.4f}, 精度={prec:.4f}, 扰动后比例={perturbed_prop:.4f}")
    
    # 找到最佳设置
    best_amp_idx = amp_data['Precision'].idxmax()
    best_net_idx = net_data['Precision'].idxmax()
    
    print(f"\n=== 最佳设置 ===")
    print(f"amp-sdp最佳: ε={amp_data.loc[best_amp_idx, 'Epsilon']}, ε'={amp_data.loc[best_amp_idx, 'Epsilon_Prime']}, 精度={amp_data.loc[best_amp_idx, 'Precision']:.6f}")
    print(f"network shuffling最佳: ε={net_data.loc[best_net_idx, 'Epsilon']}, ε'={net_data.loc[best_net_idx, 'Epsilon_Prime']}, 精度={net_data.loc[best_net_idx, 'Precision']:.6f}")
    
    # 比较两种技术的效果
    print(f"\n=== 技术对比 ===")
    print(f"amp-sdp平均精度: {amp_data['Precision'].mean():.6f}")
    print(f"network shuffling平均精度: {net_data['Precision'].mean():.6f}")
    print(f"amp-sdp平均准确率: {amp_data['Accuracy'].mean():.6f}")
    print(f"network shuffling平均准确率: {net_data['Accuracy'].mean():.6f}")

if __name__ == "__main__":
    # 可视化结果
    print("生成可视化图表...")
    report_df = visualize_rr_comparison_corrected()
    
    # 生成TikZ代码
    print("生成TikZ代码...")
    tikz_code = generate_tikz_code_corrected(report_df)
    
    with open('fire_alarm_rr_tikz_corrected.tex', 'w') as f:
        f.write(tikz_code)
    
    # 创建总结报告
    create_summary_report_corrected(report_df)
    
    print("\n分析完成！")
    print("- 图表已保存为: fire_alarm_rr_comparison_corrected.png")
    print("- TikZ代码已保存为: fire_alarm_rr_tikz_corrected.tex") 