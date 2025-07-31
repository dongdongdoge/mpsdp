import pandas as pd
import numpy as np

def apply_rr_to_binary(data, epsilon, k=2):
    """
    对二进制数据应用RR机制
    
    Parameters:
    - data: 二进制数据 (0或1)
    - epsilon: 隐私预算
    - k: 类别数量 (对于二进制数据，k=2)
    """
    # RR参数计算
    p = np.exp(epsilon) / (np.exp(epsilon) + k - 1)
    
    # 应用RR
    rng = np.random.default_rng(42)  # 固定随机种子以便复现
    perturbed_data = []
    
    for value in data:
        if rng.random() < p:
            # 以概率p报告真实值
            perturbed_data.append(value)
        else:
            # 以概率1-p报告其他值
            other_value = 1 - value  # 对于二进制数据，其他值就是1-value
            perturbed_data.append(other_value)
    
    return np.array(perturbed_data)

def analyze_rr_results(original_data, perturbed_data, epsilon):
    """
    分析RR扰动结果
    """
    # 计算原始统计
    original_count_1 = np.sum(original_data)
    original_count_0 = len(original_data) - original_count_1
    original_proportion_1 = original_count_1 / len(original_data)
    
    # 计算扰动后统计
    perturbed_count_1 = np.sum(perturbed_data)
    perturbed_count_0 = len(perturbed_data) - perturbed_count_1
    perturbed_proportion_1 = perturbed_count_1 / len(perturbed_data)
    
    # 计算准确率
    accuracy = np.mean(original_data == perturbed_data)
    
    # 计算精度（proportion的接近程度）
    precision = max(0, 1 - abs(perturbed_proportion_1 - original_proportion_1) / original_proportion_1)
    
    return {
        'original_count_1': original_count_1,
        'original_count_0': original_count_0,
        'original_proportion_1': original_proportion_1,
        'perturbed_count_1': perturbed_count_1,
        'perturbed_count_0': perturbed_count_0,
        'perturbed_proportion_1': perturbed_proportion_1,
        'accuracy': accuracy,
        'precision': precision
    }

def main():
    # 读取数据
    print("正在读取Fire alarm数据集...")
    df = pd.read_csv('Fire/Fire_alarm.csv')
    
    # 获取Original_Binary列
    original_binary = df['Original_Binary'].values
    
    # 显示原始统计
    original_count_1 = np.sum(original_binary)
    total_count = len(original_binary)
    original_proportion = original_count_1 / total_count
    
    print(f"\n=== 原始数据统计 ===")
    print(f"总记录数: {total_count}")
    print(f"Original_Binary=1的记录数: {original_count_1}")
    print(f"Original_Binary=0的记录数: {total_count - original_count_1}")
    print(f"Original_Binary=1的比例: {original_proportion:.4f}")
    
    # 定义正确的epsilon和epsilon'对应关系
    # 期望的epsilon值
    epsilon_values = [0.3, 0.5, 0.7, 1.0, 1.2, 1.5, 1.7, 2.0]
    
    # amp-sdp的epsilon'对应关系
    epsilon_prime_amp = {
        0.3: 8.418,
        0.5: 9.440,
        0.7: 10.188,
        1.0: 11.086,
        1.2: 11.602,
        1.5: 12.285,
        1.7: 12.678,
        2.0: 13.219
    }
    
    # network shuffling的epsilon'对应关系
    epsilon_prime_net = {
        0.3: 5.612,
        0.5: 6.293,
        0.7: 6.792,
        1.0: 7.391,
        1.2: 7.735,
        1.5: 8.190,
        1.7: 8.452,
        2.0: 8.813
    }
    
    results_amp = {}
    results_net = {}
    
    print(f"\n=== amp-sdp RR扰动分析 ===")
    for epsilon in epsilon_values:
        epsilon_prime = epsilon_prime_amp[epsilon]
        print(f"\n--- Epsilon = {epsilon}, Epsilon' = {epsilon_prime} ---")
        
        # 应用RR扰动（使用epsilon'作为实际的隐私预算）
        perturbed_data = apply_rr_to_binary(original_binary, epsilon_prime)
        
        # 分析结果
        analysis = analyze_rr_results(original_binary, perturbed_data, epsilon_prime)
        results_amp[epsilon] = analysis
        
        print(f"原始比例 (Binary=1): {analysis['original_proportion_1']:.4f}")
        print(f"扰动后比例 (Binary=1): {analysis['perturbed_proportion_1']:.4f}")
        print(f"准确率: {analysis['accuracy']:.4f}")
        print(f"精度: {analysis['precision']:.4f}")
    
    print(f"\n=== network shuffling RR扰动分析 ===")
    for epsilon in epsilon_values:
        epsilon_prime = epsilon_prime_net[epsilon]
        print(f"\n--- Epsilon = {epsilon}, Epsilon' = {epsilon_prime} ---")
        
        # 应用RR扰动（使用epsilon'作为实际的隐私预算）
        perturbed_data = apply_rr_to_binary(original_binary, epsilon_prime)
        
        # 分析结果
        analysis = analyze_rr_results(original_binary, perturbed_data, epsilon_prime)
        results_net[epsilon] = analysis
        
        print(f"原始比例 (Binary=1): {analysis['original_proportion_1']:.4f}")
        print(f"扰动后比例 (Binary=1): {analysis['perturbed_proportion_1']:.4f}")
        print(f"准确率: {analysis['accuracy']:.4f}")
        print(f"精度: {analysis['precision']:.4f}")
    
    # 保存结果到CSV
    print(f"\n=== 保存结果 ===")
    
    # 创建包含原始和扰动数据的DataFrame
    result_df = pd.DataFrame({
        'Original_Binary': original_binary
    })
    
    # 添加amp-sdp扰动结果
    for epsilon in epsilon_values:
        epsilon_prime = epsilon_prime_amp[epsilon]
        perturbed_data = apply_rr_to_binary(original_binary, epsilon_prime)
        result_df[f'amp_sdp_eps_{epsilon}_prime_{epsilon_prime}'] = perturbed_data
    
    # 添加network shuffling扰动结果
    for epsilon in epsilon_values:
        epsilon_prime = epsilon_prime_net[epsilon]
        perturbed_data = apply_rr_to_binary(original_binary, epsilon_prime)
        result_df[f'net_shuffle_eps_{epsilon}_prime_{epsilon_prime}'] = perturbed_data
    
    # 保存到CSV
    result_df.to_csv('fire_alarm_rr_simulated_corrected.csv', index=False)
    print("扰动结果已保存到: fire_alarm_rr_simulated_corrected.csv")
    
    # 生成分析报告
    amp_report_data = []
    for epsilon in epsilon_values:
        epsilon_prime = epsilon_prime_amp[epsilon]
        amp_report_data.append({
            'Technique': 'amp-sdp',
            'Epsilon': epsilon,
            'Epsilon_Prime': epsilon_prime,
            'Original_Count_1': results_amp[epsilon]['original_count_1'],
            'Original_Count_0': results_amp[epsilon]['original_count_0'],
            'Original_Proportion_1': results_amp[epsilon]['original_proportion_1'],
            'Perturbed_Count_1': results_amp[epsilon]['perturbed_count_1'],
            'Perturbed_Count_0': results_amp[epsilon]['perturbed_count_0'],
            'Perturbed_Proportion_1': results_amp[epsilon]['perturbed_proportion_1'],
            'Accuracy': results_amp[epsilon]['accuracy'],
            'Precision': results_amp[epsilon]['precision']
        })
    
    net_report_data = []
    for epsilon in epsilon_values:
        epsilon_prime = epsilon_prime_net[epsilon]
        net_report_data.append({
            'Technique': 'network_shuffling',
            'Epsilon': epsilon,
            'Epsilon_Prime': epsilon_prime,
            'Original_Count_1': results_amp[epsilon]['original_count_1'],
            'Original_Count_0': results_amp[epsilon]['original_count_0'],
            'Original_Proportion_1': results_amp[epsilon]['original_proportion_1'],
            'Perturbed_Count_1': results_net[epsilon]['perturbed_count_1'],
            'Perturbed_Count_0': results_net[epsilon]['perturbed_count_0'],
            'Perturbed_Proportion_1': results_net[epsilon]['perturbed_proportion_1'],
            'Accuracy': results_net[epsilon]['accuracy'],
            'Precision': results_net[epsilon]['precision']
        })
    
    report_df = pd.DataFrame(amp_report_data + net_report_data)
    report_df.to_csv('fire_alarm_rr_analysis_corrected.csv', index=False)
    print("分析报告已保存到: fire_alarm_rr_analysis_corrected.csv")
    
    return results_amp, results_net, result_df

if __name__ == "__main__":
    results_amp, results_net, result_df = main() 