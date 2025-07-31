import pandas as pd
import numpy as np
from datetime import datetime
import matplotlib.pyplot as plt

def apply_laplace_noise_to_timeseries(df, epsilon, sensitivity=1.0):
    """
    对时间序列数据应用Laplace噪声
    
    Parameters:
    - df: 包含时间列的DataFrame
    - epsilon: 隐私预算
    - sensitivity: 敏感度（时间差的最大变化）
    """
    
    # 转换时间格式
    received = pd.to_datetime(df['Received DtTm'], format='%m/%d/%Y %I:%M:%S %p')
    dispatch = pd.to_datetime(df['Dispatch DtTm'], format='%m/%d/%Y %I:%M:%S %p')
    
    # 计算原始时间差（分钟）
    original_delta = (dispatch - received).dt.total_seconds() / 60
    
    # 计算敏感度（时间差的最大可能变化）
    max_delta = original_delta.max()
    min_delta = original_delta.min()
    sensitivity = max_delta - min_delta
    
    # 添加Laplace噪声
    scale = sensitivity / epsilon
    noise = np.random.laplace(0, scale, len(original_delta))
    perturbed_delta = original_delta + noise
    
    # 确保扰动后的值非负
    perturbed_delta = np.maximum(perturbed_delta, 0)
    
    return original_delta, perturbed_delta, noise

def analyze_time_series_dp(df, epsilon_values=[0.1, 0.5, 1.0, 2.0]):
    """
    分析不同epsilon值下的时间序列DP效果
    """
    
    results = {}
    
    for epsilon in epsilon_values:
        print(f"\n=== 分析 epsilon = {epsilon} ===")
        
        # 应用DP扰动
        original, perturbed, noise = apply_laplace_noise_to_timeseries(df, epsilon)
        
        # 计算统计指标
        original_mean = original.mean()
        perturbed_mean = perturbed.mean()
        original_std = original.std()
        perturbed_std = perturbed.std()
        
        # 计算精度
        precision = max(0, 1 - abs(perturbed_mean - original_mean) / original_mean)
        
        # 计算噪声统计
        noise_mean = noise.mean()
        noise_std = noise.std()
        
        results[epsilon] = {
            'original_mean': original_mean,
            'perturbed_mean': perturbed_mean,
            'original_std': original_std,
            'perturbed_std': perturbed_std,
            'precision': precision,
            'noise_mean': noise_mean,
            'noise_std': noise_std,
            'original': original,
            'perturbed': perturbed,
            'noise': noise
        }
        
        print(f"原始均值: {original_mean:.2f} 分钟")
        print(f"扰动后均值: {perturbed_mean:.2f} 分钟")
        print(f"精度: {precision:.4f}")
        print(f"噪声均值: {noise_mean:.2f}")
        print(f"噪声标准差: {noise_std:.2f}")
    
    return results

def visualize_time_series_dp(results):
    """
    可视化时间序列DP效果
    """
    fig, axes = plt.subplots(2, 2, figsize=(15, 12))
    
    # 1. 不同epsilon下的精度对比
    epsilons = list(results.keys())
    precisions = [results[eps]['precision'] for eps in epsilons]
    
    axes[0, 0].plot(epsilons, precisions, 'bo-', linewidth=2, markersize=8)
    axes[0, 0].set_xlabel('Epsilon (ε)')
    axes[0, 0].set_ylabel('Precision')
    axes[0, 0].set_title('时间序列DP精度 vs Epsilon')
    axes[0, 0].grid(True, alpha=0.3)
    
    # 2. 原始vs扰动时间差分布
    eps = list(results.keys())[1]  # 选择中等epsilon值
    original = results[eps]['original']
    perturbed = results[eps]['perturbed']
    
    axes[0, 1].hist(original, bins=50, alpha=0.7, label='原始', density=True)
    axes[0, 1].hist(perturbed, bins=50, alpha=0.7, label=f'扰动 (ε={eps})', density=True)
    axes[0, 1].set_xlabel('时间差 (分钟)')
    axes[0, 1].set_ylabel('密度')
    axes[0, 1].set_title('时间差分布对比')
    axes[0, 1].legend()
    axes[0, 1].grid(True, alpha=0.3)
    
    # 3. 噪声分布
    noise = results[eps]['noise']
    axes[1, 0].hist(noise, bins=50, alpha=0.7, color='red')
    axes[1, 0].set_xlabel('噪声值')
    axes[1, 0].set_ylabel('频次')
    axes[1, 0].set_title(f'Laplace噪声分布 (ε={eps})')
    axes[1, 0].grid(True, alpha=0.3)
    
    # 4. 均值误差对比
    original_means = [results[eps]['original_mean'] for eps in epsilons]
    perturbed_means = [results[eps]['perturbed_mean'] for eps in epsilons]
    
    x = np.arange(len(epsilons))
    width = 0.35
    
    axes[1, 1].bar(x - width/2, original_means, width, label='原始均值', alpha=0.8)
    axes[1, 1].bar(x + width/2, perturbed_means, width, label='扰动均值', alpha=0.8)
    axes[1, 1].set_xlabel('Epsilon (ε)')
    axes[1, 1].set_ylabel('时间差均值 (分钟)')
    axes[1, 1].set_title('均值对比')
    axes[1, 1].set_xticks(x)
    axes[1, 1].set_xticklabels(epsilons)
    axes[1, 1].legend()
    axes[1, 1].grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.savefig('time_series_dp_analysis.png', dpi=300, bbox_inches='tight')
    plt.show()

def generate_tikz_code(results):
    """
    生成TikZ代码用于LaTeX文档
    """
    tikz_code = r"""
\begin{figure}[htbp]
\centering
\begin{tikzpicture}
\begin{axis}[
    width=12cm,
    height=8cm,
    xlabel={Epsilon ($\varepsilon$)},
    ylabel={Precision},
    title={Time Series DP Precision Analysis},
    grid=major,
    legend pos=north west,
    xmin=0,
    ymin=0,
    ymax=1
]

% 精度数据点
\addplot[blue, thick, mark=*, mark size=3pt] coordinates {
"""
    
    epsilons = list(results.keys())
    precisions = [results[eps]['precision'] for eps in epsilons]
    
    for eps, prec in zip(epsilons, precisions):
        tikz_code += f"    ({eps}, {prec:.4f})\n"
    
    tikz_code += r"""};

\end{axis}
\end{tikzpicture}
\caption{时间序列差分隐私精度分析}
\label{fig:timeseries_dp_precision}
\end{figure}
"""
    
    return tikz_code

if __name__ == "__main__":
    # 读取数据
    print("正在读取Fire alarm数据集...")
    df = pd.read_csv('Fire/Fire_alarm.csv')
    
    # 分析DP效果
    print("开始分析时间序列DP效果...")
    results = analyze_time_series_dp(df)
    
    # 可视化结果
    print("生成可视化图表...")
    visualize_time_series_dp(results)
    
    # 生成TikZ代码
    print("生成TikZ代码...")
    tikz_code = generate_tikz_code(results)
    
    with open('time_series_dp_tikz.tex', 'w') as f:
        f.write(tikz_code)
    
    print("分析完成！")
    print("- 图表已保存为: time_series_dp_analysis.png")
    print("- TikZ代码已保存为: time_series_dp_tikz.tex") 