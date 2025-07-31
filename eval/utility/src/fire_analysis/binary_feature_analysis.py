import pandas as pd
import numpy as np
from datetime import datetime

def create_binary_time_diff_column(df):
    """
    创建一个新的二进制列，表示时间差值是否小于1分钟
    """
    # 转换时间格式
    received = pd.to_datetime(df['Received DtTm'], format='%m/%d/%Y %I:%M:%S %p')
    dispatch = pd.to_datetime(df['Dispatch DtTm'], format='%m/%d/%Y %I:%M:%S %p')
    
    # 计算时间差（分钟）
    delta_min = (dispatch - received).dt.total_seconds() / 60
    
    # 创建二进制列：1表示时间差<=1分钟，0表示>1分钟
    binary_column = (delta_min <= 1).astype(int)
    
    return binary_column, delta_min

def apply_krr_to_binary(data, epsilon, k=2):
    """
    对二进制数据应用kRR机制
    
    Parameters:
    - data: 二进制数据 (0或1)
    - epsilon: 隐私预算
    - k: 类别数量 (对于二进制数据，k=2)
    """
    # kRR参数计算
    p = np.exp(epsilon) / (np.exp(epsilon) + k - 1)
    
    # 应用kRR
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

def analyze_binary_krr_results(original_data, perturbed_data, epsilon):
    """
    分析kRR扰动结果
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
    
    # 创建二进制列
    print("创建二进制时间差列...")
    binary_column, delta_min = create_binary_time_diff_column(df)
    
    # 显示原始统计
    original_count_1 = np.sum(binary_column)
    total_count = len(binary_column)
    original_proportion = original_count_1 / total_count
    
    print(f"\n=== 原始数据统计 ===")
    print(f"总记录数: {total_count}")
    print(f"时间差<=1分钟的记录数: {original_count_1}")
    print(f"时间差>1分钟的记录数: {total_count - original_count_1}")
    print(f"时间差<=1分钟的比例: {original_proportion:.4f}")
    
    # 测试不同的epsilon值
    epsilon_values = [0.1, 0.5, 1.0, 2.0, 5.0, 10.0]
    results = {}
    
    print(f"\n=== kRR扰动分析 ===")
    for epsilon in epsilon_values:
        print(f"\n--- Epsilon = {epsilon} ---")
        
        # 应用kRR扰动
        perturbed_data = apply_krr_to_binary(binary_column, epsilon)
        
        # 分析结果
        analysis = analyze_binary_krr_results(binary_column, perturbed_data, epsilon)
        results[epsilon] = analysis
        
        print(f"原始比例 (<=1min): {analysis['original_proportion_1']:.4f}")
        print(f"扰动后比例 (<=1min): {analysis['perturbed_proportion_1']:.4f}")
        print(f"准确率: {analysis['accuracy']:.4f}")
        print(f"精度: {analysis['precision']:.4f}")
    
    # 保存结果到CSV
    print(f"\n=== 保存结果 ===")
    
    # 创建包含原始和扰动数据的DataFrame
    result_df = pd.DataFrame({
        'Original_Binary': binary_column,
        'Time_Diff_Minutes': delta_min
    })
    
    # 添加不同epsilon值的扰动结果
    for epsilon in epsilon_values:
        perturbed_data = apply_krr_to_binary(binary_column, epsilon)
        result_df[f'kRR_eps_{epsilon}'] = perturbed_data
    
    # 保存到CSV
    result_df.to_csv('fire_alarm_binary_krr_results.csv', index=False)
    print("结果已保存到: fire_alarm_binary_krr_results.csv")
    
    # 生成分析报告
    report_df = pd.DataFrame([
        {
            'Epsilon': eps,
            'Original_Count_1': results[eps]['original_count_1'],
            'Original_Count_0': results[eps]['original_count_0'],
            'Original_Proportion_1': results[eps]['original_proportion_1'],
            'Perturbed_Count_1': results[eps]['perturbed_count_1'],
            'Perturbed_Count_0': results[eps]['perturbed_count_0'],
            'Perturbed_Proportion_1': results[eps]['perturbed_proportion_1'],
            'Accuracy': results[eps]['accuracy'],
            'Precision': results[eps]['precision']
        }
        for eps in epsilon_values
    ])
    
    report_df.to_csv('fire_alarm_binary_krr_analysis.csv', index=False)
    print("分析报告已保存到: fire_alarm_binary_krr_analysis.csv")
    
    return results, result_df

if __name__ == "__main__":
    results, result_df = main() 