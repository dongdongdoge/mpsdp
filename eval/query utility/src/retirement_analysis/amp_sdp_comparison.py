import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
from scipy import stats

def load_and_prepare_data():
    """
    加载数据并准备分析
    """
    print("正在加载retirement数据...")
    
    # 读取过滤后的retirement数据
    df = pd.read_csv('Retirement/retirement_filtered.csv')
    
    print(f"数据形状: {df.shape}")
    print(f"总记录数: {len(df):,}")
    
    return df

def get_top_job_family_statistics(df):
    """
    获取Top Job Family统计信息
    """
    # 按Job Family统计
    job_family_stats = df.groupby('Job Family').agg({
        'Retirement': ['count', 'sum', 'mean', 'std']
    }).round(2)
    
    # 重命名列
    job_family_stats.columns = ['Count', 'Total_Retirement', 'Mean_Retirement', 'Std_Retirement']
    
    # 按记录数排序
    job_family_stats_sorted = job_family_stats.sort_values('Count', ascending=False)
    
    print(f"Top 5 Job Family统计:")
    print(job_family_stats_sorted.head(5))
    
    return job_family_stats_sorted

def apply_krr_to_job_family(data, epsilon_prime, k):
    """
    应用kRR到Job Family数据
    """
    # 计算p值
    p = np.exp(epsilon_prime) / (np.exp(epsilon_prime) + k - 1)
    
    # 生成随机数
    random_values = np.random.random(len(data))
    
    # 应用kRR机制
    perturbed_data = np.where(random_values < p, data, np.random.randint(0, k, len(data)))
    
    return perturbed_data

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

def one_step_estimation(df, epsilon_primes):
    """
    一步法：直接扰动Job Family和Retirement两列
    """
    print(f"\n=== 一步法估计方法 ===")
    
    # 获取Job Family统计
    job_family_stats = get_top_job_family_statistics(df)
    
    # 获取真实top 5
    true_top_5 = job_family_stats.head(5).index.tolist()
    true_top_5_means = job_family_stats.head(5)['Mean_Retirement'].values
    true_top_5_counts = job_family_stats.head(5)['Count'].values
    
    print(f"真实Top 5 Job Family: {true_top_5}")
    print(f"真实Top 5平均退休金: {true_top_5_means}")
    
    # 获取所有Job Family
    job_families = job_family_stats.index.tolist()
    k = len(job_families)  # kRR中的k值
    
    # 创建Job Family到索引的映射
    family_to_index = {family: i for i, family in enumerate(job_families)}
    
    # 将Job Family转换为数值索引
    df['Job_Family_Index'] = df['Job Family'].map(family_to_index)
    
    results = []
    
    for epsilon_prime in epsilon_primes:
        print(f"\n处理 epsilon' = {epsilon_prime}")
        
        # 扰动Job Family
        perturbed_family_indices = apply_krr_to_job_family(df['Job_Family_Index'].values, epsilon_prime, k)
        
        # 扰动Retirement（使用相同的epsilon'）
        retirement_values = df['Retirement'].values
        sensitivity = np.max(retirement_values) / len(retirement_values)
        perturbed_retirement = apply_laplace_mechanism(retirement_values, epsilon_prime, sensitivity)
        
        # 统计扰动后的Job Family分布
        perturbed_counts = np.bincount(perturbed_family_indices, minlength=k)
        
        # 获取top 5
        top_5_indices = np.argsort(perturbed_counts)[-5:][::-1]
        estimated_top_5 = [job_families[i] for i in top_5_indices]
        estimated_counts = [perturbed_counts[i] for i in top_5_indices]
        
        # 计算每个Job Family的precision
        individual_precisions = []
        for i, family in enumerate(estimated_top_5):
            # 获取该Job Family的扰动后退休金数据
            family_mask = np.array([job_families[j] == family for j in perturbed_family_indices])
            family_retirement = perturbed_retirement[family_mask]
            if len(family_retirement) > 0:
                estimated_mean = np.mean(family_retirement)
                # 找到对应的真实均值
                if family in true_top_5:
                    true_mean = true_top_5_means[true_top_5.index(family)]
                    precision = max(0, 1 - abs(estimated_mean - true_mean) / true_mean)
                else:
                    precision = 0  # 如果不在真实top 5中，precision为0
            else:
                precision = 0
            individual_precisions.append(precision)
        
        # 计算准确率（top 5中有多少个匹配）
        matches = len(set(estimated_top_5) & set(true_top_5))
        accuracy = matches / 5
        
        # 计算平均precision
        avg_precision = np.mean(individual_precisions)
        
        # 计算相对误差
        estimated_means = []
        for family in estimated_top_5:
            family_mask = np.array([job_families[j] == family for j in perturbed_family_indices])
            family_retirement = perturbed_retirement[family_mask]
            if len(family_retirement) > 0:
                estimated_means.append(np.mean(family_retirement))
            else:
                estimated_means.append(0)
        
        relative_error = abs(np.mean(estimated_means) - np.mean(true_top_5_means)) / np.mean(true_top_5_means)
        
        results.append({
            'epsilon_prime': epsilon_prime,
            'true_top_5': true_top_5,
            'estimated_top_5': estimated_top_5,
            'accuracy': accuracy,
            'true_means': true_top_5_means,
            'estimated_means': estimated_means,
            'individual_precisions': individual_precisions,
            'avg_precision': avg_precision,
            'relative_error': relative_error,
            'matches': matches
        })
        
        print(f"  估计Top 5: {estimated_top_5}")
        print(f"  准确率: {accuracy:.4f}")
        print(f"  平均Precision: {avg_precision:.4f}")
        print(f"  各Job Family Precision: {individual_precisions}")
        print(f"  相对误差: {relative_error:.4f}")
    
    return pd.DataFrame(results)

def two_phase_estimation(df, epsilon_primes):
    """
    两阶段方法：先找top-5，再计算每个Job Family的precision
    """
    print(f"\n=== 两阶段估计方法 ===")
    
    # 获取Job Family统计
    job_family_stats = get_top_job_family_statistics(df)
    
    # 获取真实top 5
    true_top_5 = job_family_stats.head(5).index.tolist()
    true_top_5_means = job_family_stats.head(5)['Mean_Retirement'].values
    true_top_5_counts = job_family_stats.head(5)['Count'].values
    
    print(f"真实Top 5 Job Family: {true_top_5}")
    print(f"真实Top 5平均退休金: {true_top_5_means}")
    
    # 获取所有Job Family
    job_families = job_family_stats.index.tolist()
    k = len(job_families)  # kRR中的k值
    
    # 创建Job Family到索引的映射
    family_to_index = {family: i for i, family in enumerate(job_families)}
    
    # 将Job Family转换为数值索引
    df['Job_Family_Index'] = df['Job Family'].map(family_to_index)
    
    results = []
    
    for epsilon_prime in epsilon_primes:
        print(f"\n处理 epsilon' = {epsilon_prime}")
        
        # 第一阶段：kRR扰动找top 5
        perturbed_indices = apply_krr_to_job_family(df['Job_Family_Index'].values, epsilon_prime, k)
        
        # 统计扰动后的Job Family分布
        perturbed_counts = np.bincount(perturbed_indices, minlength=k)
        
        # 获取top 5
        top_5_indices = np.argsort(perturbed_counts)[-5:][::-1]
        estimated_top_5 = [job_families[i] for i in top_5_indices]
        estimated_counts = [perturbed_counts[i] for i in top_5_indices]
        
        # 第二阶段：计算每个Job Family的precision
        individual_precisions = []
        for i, family in enumerate(estimated_top_5):
            # 获取该Job Family的真实退休金数据
            family_data = df[df['Job Family'] == family]['Retirement'].values
            if len(family_data) > 0:
                # 计算敏感度
                sensitivity = np.max(family_data) / len(family_data)
                # 应用Laplace机制
                perturbed_family_data = apply_laplace_mechanism(family_data, epsilon_prime, sensitivity)
                estimated_mean = np.mean(perturbed_family_data)
                # 找到对应的真实均值
                if family in true_top_5:
                    true_mean = true_top_5_means[true_top_5.index(family)]
                    precision = max(0, 1 - abs(estimated_mean - true_mean) / true_mean)
                else:
                    precision = 0  # 如果不在真实top 5中，precision为0
            else:
                precision = 0
            individual_precisions.append(precision)
        
        # 计算准确率（top 5中有多少个匹配）
        matches = len(set(estimated_top_5) & set(true_top_5))
        accuracy = matches / 5
        
        # 计算平均precision
        avg_precision = np.mean(individual_precisions)
        
        # 计算相对误差
        estimated_means = []
        for family in estimated_top_5:
            family_data = df[df['Job Family'] == family]['Retirement'].values
            if len(family_data) > 0:
                sensitivity = np.max(family_data) / len(family_data)
                perturbed_family_data = apply_laplace_mechanism(family_data, epsilon_prime, sensitivity)
                estimated_means.append(np.mean(perturbed_family_data))
            else:
                estimated_means.append(0)
        
        relative_error = abs(np.mean(estimated_means) - np.mean(true_top_5_means)) / np.mean(true_top_5_means)
        
        results.append({
            'epsilon_prime': epsilon_prime,
            'true_top_5': true_top_5,
            'estimated_top_5': estimated_top_5,
            'accuracy': accuracy,
            'true_means': true_top_5_means,
            'estimated_means': estimated_means,
            'individual_precisions': individual_precisions,
            'avg_precision': avg_precision,
            'relative_error': relative_error,
            'matches': matches
        })
        
        print(f"  估计Top 5: {estimated_top_5}")
        print(f"  准确率: {accuracy:.4f}")
        print(f"  平均Precision: {avg_precision:.4f}")
        print(f"  各Job Family Precision: {individual_precisions}")
        print(f"  相对误差: {relative_error:.4f}")
    
    return pd.DataFrame(results)

def visualize_comparison(one_step_results, two_phase_results):
    """
    可视化对比结果
    """
    print("生成对比可视化...")
    
    # 设置中文字体
    plt.rcParams['font.sans-serif'] = ['SimHei', 'Arial Unicode MS', 'DejaVu Sans']
    plt.rcParams['axes.unicode_minus'] = False
    
    # 创建图表
    fig, axes = plt.subplots(1, 2, figsize=(15, 6))
    
    # 1. 整体Precision对比
    epsilon_primes = one_step_results['epsilon_prime'].values
    one_step_avg_precisions = one_step_results['avg_precision'].values
    two_phase_avg_precisions = two_phase_results['avg_precision'].values
    
    axes[0].plot(epsilon_primes, one_step_avg_precisions, 'ro-', linewidth=2, markersize=8, label='一步法')
    axes[0].plot(epsilon_primes, two_phase_avg_precisions, 'bo-', linewidth=2, markersize=8, label='两阶段方法')
    axes[0].set_xlabel('Epsilon\'')
    axes[0].set_ylabel('整体Precision')
    axes[0].set_title('整体Precision对比')
    axes[0].grid(True, alpha=0.3)
    axes[0].legend()
    axes[0].set_xscale('log')
    
    # 2. 各Category Error对比（MSE）- 选择第二个和第四个epsilon'
    true_top_5 = one_step_results['true_top_5'].iloc[0]
    true_means = one_step_results['true_means'].iloc[0]
    
    # 选择第二个epsilon' (9.44) 和第四个epsilon' (11.086)
    epsilon_primes = one_step_results['epsilon_prime'].values
    selected_epsilons = [epsilon_primes[1], epsilon_primes[3]]  # 第二个和第四个
    selected_names = [f'ε\'={selected_epsilons[0]:.3f}', f'ε\'={selected_epsilons[1]:.3f}']
    
    # 计算每个category的MSE
    one_step_mse_1 = []
    one_step_mse_2 = []
    two_phase_mse_1 = []
    two_phase_mse_2 = []
    
    for i, family in enumerate(true_top_5):
        # 一步法的MSE - 第二个epsilon'
        one_step_errors_1 = []
        row_1 = one_step_results.iloc[1]  # 第二个epsilon'
        if i < len(row_1['estimated_means']):
            estimated_mean = row_1['estimated_means'][i]
            true_mean = true_means[i]
            error = (estimated_mean - true_mean) ** 2
            one_step_mse_1.append(error)
        else:
            one_step_mse_1.append(0)
        
        # 一步法的MSE - 第四个epsilon'
        one_step_errors_2 = []
        row_2 = one_step_results.iloc[3]  # 第四个epsilon'
        if i < len(row_2['estimated_means']):
            estimated_mean = row_2['estimated_means'][i]
            true_mean = true_means[i]
            error = (estimated_mean - true_mean) ** 2
            one_step_mse_2.append(error)
        else:
            one_step_mse_2.append(0)
        
        # 两阶段方法的MSE - 第二个epsilon'
        two_phase_errors_1 = []
        row_1_phase = two_phase_results.iloc[1]  # 第二个epsilon'
        if i < len(row_1_phase['estimated_means']):
            estimated_mean = row_1_phase['estimated_means'][i]
            true_mean = true_means[i]
            error = (estimated_mean - true_mean) ** 2
            two_phase_mse_1.append(error)
        else:
            two_phase_mse_1.append(0)
        
        # 两阶段方法的MSE - 第四个epsilon'
        two_phase_errors_2 = []
        row_2_phase = two_phase_results.iloc[3]  # 第四个epsilon'
        if i < len(row_2_phase['estimated_means']):
            estimated_mean = row_2_phase['estimated_means'][i]
            true_mean = true_means[i]
            error = (estimated_mean - true_mean) ** 2
            two_phase_mse_2.append(error)
        else:
            two_phase_mse_2.append(0)
    
    x = np.arange(len(true_top_5))
    width = 0.2
    
    # 绘制四个柱状图：一步法(ε'=9.44), 两阶段(ε'=9.44), 一步法(ε'=11.086), 两阶段(ε'=11.086)
    bars1 = axes[1].bar(x - 1.5*width, one_step_mse_1, width, label=f'一步法 ({selected_names[0]})', color='red', alpha=0.7)
    bars2 = axes[1].bar(x - 0.5*width, two_phase_mse_1, width, label=f'两阶段方法 ({selected_names[0]})', color='blue', alpha=0.7)
    bars3 = axes[1].bar(x + 0.5*width, one_step_mse_2, width, label=f'一步法 ({selected_names[1]})', color='orange', alpha=0.7)
    bars4 = axes[1].bar(x + 1.5*width, two_phase_mse_2, width, label=f'两阶段方法 ({selected_names[1]})', color='green', alpha=0.7)
    
    # 在柱状图上标注数字
    all_mse = one_step_mse_1 + two_phase_mse_1 + one_step_mse_2 + two_phase_mse_2
    max_height = max(all_mse) if all_mse else 1
    
    for bar in bars1:
        height = bar.get_height()
        axes[1].text(bar.get_x() + bar.get_width()/2., height + max_height * 0.01,
                     f'{height:.0f}', ha='center', va='bottom', fontsize=7)
    
    for bar in bars2:
        height = bar.get_height()
        axes[1].text(bar.get_x() + bar.get_width()/2., height + max_height * 0.01,
                     f'{height:.0f}', ha='center', va='bottom', fontsize=7)
    
    for bar in bars3:
        height = bar.get_height()
        axes[1].text(bar.get_x() + bar.get_width()/2., height + max_height * 0.01,
                     f'{height:.0f}', ha='center', va='bottom', fontsize=7)
    
    for bar in bars4:
        height = bar.get_height()
        axes[1].text(bar.get_x() + bar.get_width()/2., height + max_height * 0.01,
                     f'{height:.0f}', ha='center', va='bottom', fontsize=7)
    
    axes[1].set_xlabel('Job Family')
    axes[1].set_ylabel('MSE')
    axes[1].set_title('各Category MSE对比 (ε\'=9.44 vs ε\'=11.086)')
    axes[1].set_xticks(x)
    axes[1].set_xticklabels([name[:15] + '...' if len(name) > 15 else name for name in true_top_5], rotation=45)
    axes[1].legend(bbox_to_anchor=(1.05, 1), loc='upper left')
    axes[1].grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.savefig('two_phase_vs_one_step_comparison.png', dpi=300, bbox_inches='tight')
    plt.show()
    
    return fig

def generate_tikz_code(one_step_results, two_phase_results):
    """
    生成TikZ代码 - 并排两张图
    """
    # 获取数据
    epsilon_primes = one_step_results['epsilon_prime'].values
    one_step_avg_precisions = one_step_results['avg_precision'].values
    two_phase_avg_precisions = two_phase_results['avg_precision'].values
    
    true_top_5 = one_step_results['true_top_5'].iloc[0]
    true_means = one_step_results['true_means'].iloc[0]
    
    # 计算每个category的MSE - 选择第二个和第四个epsilon'
    epsilon_primes = one_step_results['epsilon_prime'].values
    selected_epsilons = [epsilon_primes[1], epsilon_primes[3]]  # 第二个和第四个
    
    one_step_mse_1 = []
    one_step_mse_2 = []
    two_phase_mse_1 = []
    two_phase_mse_2 = []
    
    for i, family in enumerate(true_top_5):
        # 一步法的MSE - 第二个epsilon'
        row_1 = one_step_results.iloc[1]  # 第二个epsilon'
        if i < len(row_1['estimated_means']):
            estimated_mean = row_1['estimated_means'][i]
            true_mean = true_means[i]
            error = (estimated_mean - true_mean) ** 2
            one_step_mse_1.append(error)
        else:
            one_step_mse_1.append(0)
        
        # 一步法的MSE - 第四个epsilon'
        row_2 = one_step_results.iloc[3]  # 第四个epsilon'
        if i < len(row_2['estimated_means']):
            estimated_mean = row_2['estimated_means'][i]
            true_mean = true_means[i]
            error = (estimated_mean - true_mean) ** 2
            one_step_mse_2.append(error)
        else:
            one_step_mse_2.append(0)
        
        # 两阶段方法的MSE - 第二个epsilon'
        row_1_phase = two_phase_results.iloc[1]  # 第二个epsilon'
        if i < len(row_1_phase['estimated_means']):
            estimated_mean = row_1_phase['estimated_means'][i]
            true_mean = true_means[i]
            error = (estimated_mean - true_mean) ** 2
            two_phase_mse_1.append(error)
        else:
            two_phase_mse_1.append(0)
        
        # 两阶段方法的MSE - 第四个epsilon'
        row_2_phase = two_phase_results.iloc[3]  # 第四个epsilon'
        if i < len(row_2_phase['estimated_means']):
            estimated_mean = row_2_phase['estimated_means'][i]
            true_mean = true_means[i]
            error = (estimated_mean - true_mean) ** 2
            two_phase_mse_2.append(error)
        else:
            two_phase_mse_2.append(0)
    
    # 原始epsilon值映射
    original_epsilons = [0.3, 0.5, 0.7, 1.0, 1.2, 1.5, 1.7, 2.0]
    
    tikz_code = r"""
\begin{figure}[htbp]
\centering
\subfloat[Overall Precision Comparison]{
\begin{tikzpicture}
\begin{axis}[
    width=6cm,
    height=5cm,
    xlabel={Epsilon},
    ylabel={Overall Precision},
    title={Overall Precision Comparison},
    title style={at={(0.5,0.95)}, anchor=north},
    xmode=log,
    log basis x=10,
    ymode=normal,
    grid=major,
    legend pos=north west
]

% One-step method results
\addplot[red, mark=square, thick] coordinates {
"""
    
    for i, epsilon in enumerate(original_epsilons):
        tikz_code += f"    ({epsilon}, {one_step_avg_precisions[i]:.4f})\n"
    
    tikz_code += r"""};
\addlegendentry{One-Step Method}

% Two-phase method results
\addplot[blue, mark=*, thick] coordinates {
"""
    
    for i, epsilon in enumerate(original_epsilons):
        tikz_code += f"    ({epsilon}, {two_phase_avg_precisions[i]:.4f})\n"
    
    tikz_code += r"""};
\addlegendentry{Two-Phase Method}

\end{axis}
\end{tikzpicture}
}
\hfill
\subfloat[Category MSE Comparison]{
\begin{tikzpicture}
\begin{axis}[
    width=8cm,
    height=5cm,
    xlabel={Job Family},
    ylabel={MSE},
    title={Category MSE Comparison (ε'=9.44 vs ε'=11.086)},
    title style={at={(0.5,0.95)}, anchor=north},
    ymode=normal,
    xtick={1,2,3,4,5},
    xticklabels={Nursing,Street Transit,Police Services,Journeyman Trade,Human Services},
    x tick label style={rotate=45, anchor=east},
    ybar,
    bar width=0.15,
    grid=major,
    legend pos=north west
]

% One-step MSE (ε'=9.44)
\addplot[fill=red!60] coordinates {
"""
    
    for i, mse in enumerate(one_step_mse_1, 1):
        tikz_code += f"    ({i-0.45}, {mse:.0f})\n"
    
    tikz_code += r"""};
\addlegendentry{One-Step (ε'=9.44)}

% Two-phase MSE (ε'=9.44)
\addplot[fill=blue!60] coordinates {
"""
    
    for i, mse in enumerate(two_phase_mse_1, 1):
        tikz_code += f"    ({i-0.15}, {mse:.0f})\n"
    
    tikz_code += r"""};
\addlegendentry{Two-Phase (ε'=9.44)}

% One-step MSE (ε'=11.086)
\addplot[fill=orange!60] coordinates {
"""
    
    for i, mse in enumerate(one_step_mse_2, 1):
        tikz_code += f"    ({i+0.15}, {mse:.0f})\n"
    
    tikz_code += r"""};
\addlegendentry{One-Step (ε'=11.086)}

% Two-phase MSE (ε'=11.086)
\addplot[fill=green!60] coordinates {
"""
    
    for i, mse in enumerate(two_phase_mse_2, 1):
        tikz_code += f"    ({i+0.45}, {mse:.0f})\n"
    
    tikz_code += r"""};
\addlegendentry{Two-Phase (ε'=11.086)}

\end{axis}
\end{tikzpicture}
}
\caption{One-Step vs Two-Phase Method: Overall Precision and Category MSE Comparison}
\label{fig:one_step_vs_two_phase}
\end{figure}
"""
    
    return tikz_code

def create_summary_report(one_step_results, two_phase_results):
    """
    创建总结报告
    """
    print(f"\n=== 一步法 vs 两阶段方法总结报告 ===")
    
    print(f"真实Top 5 Job Family: {one_step_results['true_top_5'].iloc[0]}")
    print(f"真实Top 5平均退休金: {one_step_results['true_means'].iloc[0]}")
    
    # 计算整体Precision对比
    avg_one_step_precision = one_step_results['avg_precision'].mean()
    avg_two_phase_precision = two_phase_results['avg_precision'].mean()
    
    print(f"\n整体Precision对比:")
    print(f"  一步法: {avg_one_step_precision:.4f}")
    print(f"  两阶段方法: {avg_two_phase_precision:.4f}")
    print(f"  改进: {((avg_two_phase_precision - avg_one_step_precision) / avg_one_step_precision * 100):.2f}%")
    
    # 计算各Category的MSE
    print(f"\n各Category MSE对比:")
    true_top_5 = one_step_results['true_top_5'].iloc[0]
    true_means = one_step_results['true_means'].iloc[0]
    
    for i, family in enumerate(true_top_5):
        # 一步法的MSE
        one_step_errors = []
        for _, row in one_step_results.iterrows():
            if i < len(row['estimated_means']):
                estimated_mean = row['estimated_means'][i]
                true_mean = true_means[i]
                error = (estimated_mean - true_mean) ** 2
                one_step_errors.append(error)
        one_step_mse = np.mean(one_step_errors)
        
        # 两阶段方法的MSE
        two_phase_errors = []
        for _, row in two_phase_results.iterrows():
            if i < len(row['estimated_means']):
                estimated_mean = row['estimated_means'][i]
                true_mean = true_means[i]
                error = (estimated_mean - true_mean) ** 2
                two_phase_errors.append(error)
        two_phase_mse = np.mean(two_phase_errors)
        
        print(f"  {family}:")
        print(f"    一步法MSE: {one_step_mse:.2f}")
        print(f"    两阶段方法MSE: {two_phase_mse:.2f}")
        print(f"    改进: {((one_step_mse - two_phase_mse) / one_step_mse * 100):.2f}%" if one_step_mse > 0 else "    改进: N/A")

def main():
    """
    主函数
    """
    print("开始一步法 vs 两阶段方法对比分析...")
    
    # 加载数据
    df = load_and_prepare_data()
    
    # 设置epsilon'值（amp-sdp等价）
    epsilon_primes = [8.418, 9.440, 10.188, 11.086, 11.602, 12.285, 12.678, 13.219]
    
    print(f"使用的epsilon'值: {epsilon_primes}")
    
    # 一步法
    one_step_results = one_step_estimation(df, epsilon_primes)
    
    # 两阶段方法
    two_phase_results = two_phase_estimation(df, epsilon_primes)
    
    # 可视化结果
    visualize_comparison(one_step_results, two_phase_results)
    
    # 生成TikZ代码
    tikz_code = generate_tikz_code(one_step_results, two_phase_results)
    with open('one_step_vs_two_phase_tikz.tex', 'w') as f:
        f.write(tikz_code)
    
    # 创建总结报告
    create_summary_report(one_step_results, two_phase_results)
    
    # 保存结果
    one_step_results.to_csv('one_step_results.csv', index=False)
    two_phase_results.to_csv('two_phase_results.csv', index=False)
    
    print(f"\n分析完成！")
    print(f"- 图表已保存为: two_phase_vs_one_step_comparison.png")
    print(f"- TikZ代码已保存为: one_step_vs_two_phase_tikz.tex")
    print(f"- 一步法结果已保存为: one_step_results.csv")
    print(f"- 两阶段方法结果已保存为: two_phase_results.csv")
    
    return one_step_results, two_phase_results

if __name__ == "__main__":
    one_step_results, two_phase_results = main() 