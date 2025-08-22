import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
from scipy import stats

def load_and_prepare_data():
    """
    加载数据并准备Firefighter分析
    """
    print("正在加载retirement数据...")
    
    # 读取过滤后的retirement数据
    df = pd.read_csv('Retirement/retirement_filtered.csv')
    
    # 获取Firefighter数据
    firefighter_data = df[df['Job'] == 'Firefighter'].copy()
    other_data = df[df['Job'] != 'Firefighter'].copy()
    
    print(f"总记录数: {len(df):,}")
    print(f"Firefighter记录数: {len(firefighter_data):,}")
    print(f"其他Job类型记录数: {len(other_data):,}")
    
    return firefighter_data, other_data, df

def calculate_true_statistics(firefighter_data):
    """
    计算Firefighter的真实统计信息
    """
    print(f"\n=== Firefighter真实统计信息 ===")
    
    retirement_values = firefighter_data['Retirement'].values
    
    true_mean = np.mean(retirement_values)
    true_median = np.median(retirement_values)
    true_std = np.std(retirement_values)
    true_min = np.min(retirement_values)
    true_max = np.max(retirement_values)
    
    print(f"真实平均值: ${true_mean:,.2f}")
    print(f"真实中位数: ${true_median:,.2f}")
    print(f"真实标准差: ${true_std:,.2f}")
    print(f"真实最小值: ${true_min:,.2f}")
    print(f"真实最大值: ${true_max:,.2f}")
    print(f"记录数: {len(retirement_values):,}")
    
    return {
        'true_mean': true_mean,
        'true_median': true_median,
        'true_std': true_std,
        'true_min': true_min,
        'true_max': true_max,
        'count': len(retirement_values)
    }

def apply_laplace_mechanism(data, epsilon, sensitivity):
    """
    应用Laplace机制进行差分隐私扰动
    """
    # 计算Laplace分布的scale参数
    scale = sensitivity / epsilon
    
    # 生成Laplace噪声
    noise = np.random.laplace(0, scale, len(data))
    
    # 添加噪声到原始数据
    perturbed_data = data + noise
    
    return perturbed_data

def simulate_dp_analysis(firefighter_data, epsilon_values, num_simulations=100):
    """
    模拟差分隐私分析
    """
    print(f"\n=== 差分隐私分析模拟 ===")
    print(f"模拟次数: {num_simulations}")
    
    retirement_values = firefighter_data['Retirement'].values
    true_mean = np.mean(retirement_values)
    
    # 计算敏感度（单个记录的最大影响）
    sensitivity = np.max(retirement_values) / len(retirement_values)
    
    results = []
    
    for epsilon in epsilon_values:
        print(f"\n处理 epsilon = {epsilon}")
        
        estimated_means = []
        estimated_medians = []
        estimated_stds = []
        
        for sim in range(num_simulations):
            # 应用Laplace机制
            perturbed_values = apply_laplace_mechanism(retirement_values, epsilon, sensitivity)
            
            # 计算扰动后的统计量
            estimated_mean = np.mean(perturbed_values)
            estimated_median = np.median(perturbed_values)
            estimated_std = np.std(perturbed_values)
            
            estimated_means.append(estimated_mean)
            estimated_medians.append(estimated_median)
            estimated_stds.append(estimated_std)
        
        # 计算统计信息
        mean_error = np.mean(np.abs(np.array(estimated_means) - true_mean))
        mean_relative_error = np.mean(np.abs(np.array(estimated_means) - true_mean) / true_mean)
        precision = np.mean([max(0, 1 - abs(est - true_mean) / true_mean) for est in estimated_means])
        
        # 计算95%置信区间
        mean_ci_lower = np.percentile(estimated_means, 2.5)
        mean_ci_upper = np.percentile(estimated_means, 97.5)
        
        results.append({
            'epsilon': epsilon,
            'true_mean': true_mean,
            'estimated_mean_mean': np.mean(estimated_means),
            'estimated_mean_std': np.std(estimated_means),
            'mean_error': mean_error,
            'mean_relative_error': mean_relative_error,
            'precision': precision,
            'mean_ci_lower': mean_ci_lower,
            'mean_ci_upper': mean_ci_upper,
            'estimated_median_mean': np.mean(estimated_medians),
            'estimated_std_mean': np.mean(estimated_stds)
        })
    
    return pd.DataFrame(results)

def create_dummy_dataset(firefighter_data, other_data, epsilon_values):
    """
    创建包含dummy数据的完整数据集
    """
    print(f"\n=== 创建包含Dummy数据的完整数据集 ===")
    
    results = []
    
    for epsilon in epsilon_values:
        print(f"处理 epsilon = {epsilon}")
        
        # 获取Firefighter的真实数据
        firefighter_retirement = firefighter_data['Retirement'].values
        true_mean = np.mean(firefighter_retirement)
        
        # 计算敏感度
        sensitivity = np.max(firefighter_retirement) / len(firefighter_retirement)
        
        # 应用Laplace机制到Firefighter数据
        perturbed_firefighter = apply_laplace_mechanism(firefighter_retirement, epsilon, sensitivity)
        
        # 创建dummy数据（其他Job类型提交0）
        dummy_other = np.zeros(len(other_data))
        
        # 合并数据
        all_perturbed = np.concatenate([perturbed_firefighter, dummy_other])
        
        # 计算总体统计
        overall_mean = np.mean(all_perturbed)
        firefighter_only_mean = np.mean(perturbed_firefighter)
        
        # 计算误差
        error_vs_true = abs(overall_mean - true_mean)
        relative_error = error_vs_true / true_mean
        precision = max(0, 1 - relative_error)
        
        results.append({
            'epsilon': epsilon,
            'true_firefighter_mean': true_mean,
            'perturbed_firefighter_mean': firefighter_only_mean,
            'overall_mean_with_dummy': overall_mean,
            'error_vs_true': error_vs_true,
            'relative_error': relative_error,
            'precision': precision,
            'firefighter_count': len(firefighter_retirement),
            'other_count': len(dummy_other),
            'total_count': len(all_perturbed)
        })
    
    return pd.DataFrame(results)

def visualize_dp_results(dp_results, dummy_results):
    """
    可视化差分隐私分析结果
    """
    print("生成差分隐私分析可视化...")
    
    # 设置中文字体
    plt.rcParams['font.sans-serif'] = ['SimHei', 'Arial Unicode MS', 'DejaVu Sans']
    plt.rcParams['axes.unicode_minus'] = False
    
    # 创建图表
    fig, axes = plt.subplots(2, 2, figsize=(15, 12))
    
    # 1. Precision vs Epsilon
    axes[0, 0].plot(dp_results['epsilon'], dp_results['precision'], 'bo-', linewidth=2, markersize=8)
    axes[0, 0].set_xlabel('Epsilon')
    axes[0, 0].set_ylabel('Precision')
    axes[0, 0].set_title('Firefighter DP分析: Precision vs Epsilon')
    axes[0, 0].grid(True, alpha=0.3)
    axes[0, 0].set_xscale('log')
    
    # 2. 相对误差 vs Epsilon
    axes[0, 1].plot(dp_results['epsilon'], dp_results['mean_relative_error'], 'ro-', linewidth=2, markersize=8)
    axes[0, 1].set_xlabel('Epsilon')
    axes[0, 1].set_ylabel('相对误差')
    axes[0, 1].set_title('Firefighter DP分析: 相对误差 vs Epsilon')
    axes[0, 1].grid(True, alpha=0.3)
    axes[0, 1].set_xscale('log')
    
    # 3. 估计均值 vs Epsilon
    axes[1, 0].plot(dp_results['epsilon'], dp_results['estimated_mean_mean'], 'go-', linewidth=2, markersize=8, label='估计均值')
    axes[1, 0].axhline(y=dp_results['true_mean'].iloc[0], color='r', linestyle='--', label='真实均值')
    axes[1, 0].set_xlabel('Epsilon')
    axes[1, 0].set_ylabel('均值 ($)')
    axes[1, 0].set_title('Firefighter DP分析: 估计均值 vs Epsilon')
    axes[1, 0].grid(True, alpha=0.3)
    axes[1, 0].set_xscale('log')
    axes[1, 0].legend()
    
    # 4. Dummy数据对比
    axes[1, 1].plot(dummy_results['epsilon'], dummy_results['precision'], 'mo-', linewidth=2, markersize=8, label='包含Dummy数据')
    axes[1, 1].plot(dp_results['epsilon'], dp_results['precision'], 'bo-', linewidth=2, markersize=8, label='仅Firefighter数据')
    axes[1, 1].set_xlabel('Epsilon')
    axes[1, 1].set_ylabel('Precision')
    axes[1, 1].set_title('Dummy数据 vs 真实数据对比')
    axes[1, 1].grid(True, alpha=0.3)
    axes[1, 1].set_xscale('log')
    axes[1, 1].legend()
    
    plt.tight_layout()
    plt.savefig('firefighter_dp_analysis.png', dpi=300, bbox_inches='tight')
    plt.show()
    
    return fig

def generate_tikz_code(dp_results, dummy_results):
    """
    生成TikZ代码
    """
    tikz_code = r"""
\begin{figure}[htbp]
\centering
\begin{tikzpicture}
\begin{axis}[
    width=12cm,
    height=8cm,
    xlabel={Epsilon},
    ylabel={Precision},
    title={Firefighter Retirement DP Analysis: Precision vs Epsilon},
    grid=major,
    legend pos=north west,
    xmode=log,
    log basis x=10,
    ymode=normal
]

% DP分析结果
\addplot[blue, mark=*, thick] coordinates {
"""
    
    for _, row in dp_results.iterrows():
        tikz_code += f"    ({row['epsilon']}, {row['precision']:.4f})\n"
    
    tikz_code += r"""};
\addlegendentry{DP Analysis}

% Dummy数据对比
\addplot[red, mark=square, thick] coordinates {
"""
    
    for _, row in dummy_results.iterrows():
        tikz_code += f"    ({row['epsilon']}, {row['precision']:.4f})\n"
    
    tikz_code += r"""};
\addlegendentry{With Dummy Data}

\end{axis}
\end{tikzpicture}
\caption{Firefighter退休金差分隐私分析: Precision vs Epsilon对比}
\label{fig:firefighter_dp_analysis}
\end{figure}
"""
    
    return tikz_code

def create_summary_report(dp_results, dummy_results, true_stats):
    """
    创建总结报告
    """
    print(f"\n=== Firefighter DP分析总结报告 ===")
    
    print(f"真实统计信息:")
    print(f"  - 平均值: ${true_stats['true_mean']:,.2f}")
    print(f"  - 中位数: ${true_stats['true_median']:,.2f}")
    print(f"  - 标准差: ${true_stats['true_std']:,.2f}")
    print(f"  - 记录数: {true_stats['count']:,}")
    
    print(f"\n差分隐私分析结果:")
    for _, row in dp_results.iterrows():
        print(f"  Epsilon = {row['epsilon']}:")
        print(f"    - 估计均值: ${row['estimated_mean_mean']:,.2f}")
        print(f"    - Precision: {row['precision']:.4f}")
        print(f"    - 相对误差: {row['mean_relative_error']:.4f}")
    
    print(f"\nDummy数据对比:")
    for _, row in dummy_results.iterrows():
        print(f"  Epsilon = {row['epsilon']}:")
        print(f"    - 总体均值: ${row['overall_mean_with_dummy']:,.2f}")
        print(f"    - Precision: {row['precision']:.4f}")
        print(f"    - 相对误差: {row['relative_error']:.4f}")

def main():
    """
    主函数
    """
    print("开始Firefighter差分隐私分析...")
    
    # 加载数据
    firefighter_data, other_data, df = load_and_prepare_data()
    
    # 计算真实统计信息
    true_stats = calculate_true_statistics(firefighter_data)
    
    # 设置epsilon值
    epsilon_values = [0.1, 0.5, 1.0, 2.0, 5.0, 10.0]
    
    # 进行DP分析
    dp_results = simulate_dp_analysis(firefighter_data, epsilon_values)
    
    # 创建dummy数据集分析
    dummy_results = create_dummy_dataset(firefighter_data, other_data, epsilon_values)
    
    # 可视化结果
    visualize_dp_results(dp_results, dummy_results)
    
    # 生成TikZ代码
    tikz_code = generate_tikz_code(dp_results, dummy_results)
    with open('firefighter_dp_tikz.tex', 'w') as f:
        f.write(tikz_code)
    
    # 创建总结报告
    create_summary_report(dp_results, dummy_results, true_stats)
    
    # 保存结果
    dp_results.to_csv('firefighter_dp_results.csv', index=False)
    dummy_results.to_csv('firefighter_dummy_results.csv', index=False)
    
    print(f"\n分析完成！")
    print(f"- 图表已保存为: firefighter_dp_analysis.png")
    print(f"- TikZ代码已保存为: firefighter_dp_tikz.tex")
    print(f"- DP分析结果已保存为: firefighter_dp_results.csv")
    print(f"- Dummy数据结果已保存为: firefighter_dummy_results.csv")
    
    return dp_results, dummy_results, true_stats

if __name__ == "__main__":
    dp_results, dummy_results, true_stats = main() 