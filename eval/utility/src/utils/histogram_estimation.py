import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from collections import Counter
import seaborn as sns

# Load data
print("Loading data...")
df = pd.read_csv('retirement_dept.csv')

print(f"Data shape: {df.shape}")
print(f"Column names: {list(df.columns)}")

# Analyze department codes
department_counts = df['Department Code'].value_counts()
print(f"\nUnique department codes: {len(department_counts)}")
print(f"Total records: {len(df)}")

print("\nTop 10 department codes:")
print(department_counts.head(10))

# Original histogram (ground truth)
original_histogram = department_counts.to_dict()
total_records = len(df)

# Function to calculate histogram estimation precision
def calculate_histogram_precision(estimated_hist, true_hist):
    """Calculate precision for histogram estimation"""
    total_error = 0
    total_true = sum(true_hist.values())
    
    for dept in true_hist.keys():
        estimated_count = estimated_hist.get(dept, 0)
        true_count = true_hist[dept]
        error = abs(estimated_count - true_count) / true_count if true_count > 0 else 0
        total_error += error * true_count
    
    precision = 1 - (total_error / total_true)
    return max(0, precision)

# Define epsilon values
epsilon_primes_original = [4.545, 7.575, 10.605, 15.150, 18.180, 22.725, 25.755, 30.300]
epsilon_primes_network = [5.612, 6.293, 6.792, 7.391, 7.735, 8.190, 8.452, 8.813]

print(f"\nOriginal epsilon' values: {epsilon_primes_original}")
print(f"Network shuffling epsilon' values: {epsilon_primes_network}")

# Simulate kRR for different epsilon values
def simulate_krr_histogram(epsilon_prime, true_histogram, k=2):
    """Simulate k-Randomized Response histogram estimation"""
    # kRR parameters
    p = np.exp(epsilon_prime) / (np.exp(epsilon_prime) + k - 1)
    q = 1 / (np.exp(epsilon_prime) + k - 1)
    
    estimated_histogram = {}
    
    for dept, true_count in true_histogram.items():
        # Simulate kRR mechanism
        # For each true count, apply kRR
        estimated_count = 0
        for _ in range(int(true_count)):
            # Apply kRR mechanism
            if np.random.random() < p:
                # Report truth
                estimated_count += 1
            else:
                # Report random value (simplified)
                estimated_count += np.random.random() * 0.5
        
        estimated_histogram[dept] = estimated_count
    
    return estimated_histogram

# Analyze original amplification technique
print("\n" + "="*60)
print("Original Amplification Technique Analysis")
print("="*60)

original_results = []
for eps in epsilon_primes_original:
    estimated_hist = simulate_krr_histogram(eps, original_histogram)
    precision = calculate_histogram_precision(estimated_hist, original_histogram)
    
    original_results.append({
        'epsilon': eps,
        'precision': precision,
        'estimated_histogram': estimated_hist
    })
    
    print(f"ε'={eps}: Precision={precision:.4f}")

# Analyze network shuffling technique
print("\n" + "="*60)
print("Network Shuffling Technique Analysis")
print("="*60)

network_results = []
for eps in epsilon_primes_network:
    estimated_hist = simulate_krr_histogram(eps, original_histogram)
    precision = calculate_histogram_precision(estimated_hist, original_histogram)
    
    network_results.append({
        'epsilon': eps,
        'precision': precision,
        'estimated_histogram': estimated_hist
    })
    
    print(f"ε'={eps}: Precision={precision:.4f}")

# Create comparison plots
fig, ((ax1, ax2), (ax3, ax4)) = plt.subplots(2, 2, figsize=(15, 10))

# Plot 1: Original technique precision
ax1.plot([r['epsilon'] for r in original_results], 
         [r['precision'] for r in original_results], 
         'ro-', linewidth=2, markersize=8, label='Original Technique')
ax1.set_xlabel('ε\'')
ax1.set_ylabel('Precision')
ax1.set_title('Original Amplification Technique')
ax1.grid(True, alpha=0.3)
ax1.legend()

# Plot 2: Network shuffling precision
ax2.plot([r['epsilon'] for r in network_results], 
         [r['precision'] for r in network_results], 
         'bo-', linewidth=2, markersize=8, label='Network Shuffling')
ax2.set_xlabel('ε\'')
ax2.set_ylabel('Precision')
ax2.set_title('Network Shuffling Technique')
ax2.grid(True, alpha=0.3)
ax2.legend()

# Plot 3: Comparison of all techniques
ax3.plot([r['epsilon'] for r in original_results], 
         [r['precision'] for r in original_results], 
         'ro-', linewidth=2, markersize=8, label='Original')
ax3.plot([r['epsilon'] for r in network_results], 
         [r['precision'] for r in network_results], 
         'bs-', linewidth=2, markersize=8, label='Network Shuffling')
ax3.set_xlabel('ε\'')
ax3.set_ylabel('Precision')
ax3.set_title('Comparison of Techniques')
ax3.grid(True, alpha=0.3)
ax3.legend()

# Plot 4: Histogram comparison for one epsilon value
sample_eps = 7.575  # Choose a value that exists in original technique
original_sample = next(r for r in original_results if abs(r['epsilon'] - sample_eps) < 0.1)
# Use a similar epsilon value for network shuffling
network_sample = network_results[1]  # Use the second result (epsilon around 6.293)

# Get top 10 departments for visualization
top_depts = list(original_histogram.keys())[:10]
x = np.arange(len(top_depts))

width = 0.35
ax4.bar(x - width/2, [original_histogram[dept] for dept in top_depts], 
        width, label='True Count', alpha=0.7)
ax4.bar(x + width/2, [original_sample['estimated_histogram'].get(dept, 0) for dept in top_depts], 
        width, label=f'Estimated (ε\'={sample_eps:.1f})', alpha=0.7)

ax4.set_xlabel('Department Code')
ax4.set_ylabel('Count')
ax4.set_title('Histogram Comparison')
ax4.set_xticks(x)
ax4.set_xticklabels(top_depts, rotation=45)
ax4.legend()

plt.tight_layout()
plt.savefig('department_histogram_analysis.png', dpi=300, bbox_inches='tight')
print("\nAnalysis chart saved as: department_histogram_analysis.png")

# Generate TikZ coordinates
print("\n" + "="*60)
print("TikZ Coordinates for Histogram Analysis")
print("="*60)

print("% Original technique coordinates")
print("\\addplot[red, mark=*, mark size=3pt, thick] coordinates {")
for r in original_results:
    print(f"    ({r['epsilon']}, {r['precision']:.4f})")
print("};")

print("\n% Network shuffling coordinates")
print("\\addplot[blue, mark=square*, mark size=3pt, thick] coordinates {")
for r in network_results:
    print(f"    ({r['epsilon']}, {r['precision']:.4f})")
print("};")

# Summary statistics
print("\n" + "="*60)
print("Summary Statistics")
print("="*60)

print(f"\nOriginal Technique:")
print(f"  Average Precision: {np.mean([r['precision'] for r in original_results]):.4f}")
print(f"  Min Precision: {min([r['precision'] for r in original_results]):.4f}")
print(f"  Max Precision: {max([r['precision'] for r in original_results]):.4f}")

print(f"\nNetwork Shuffling:")
print(f"  Average Precision: {np.mean([r['precision'] for r in network_results]):.4f}")
print(f"  Min Precision: {min([r['precision'] for r in network_results]):.4f}")
print(f"  Max Precision: {max([r['precision'] for r in network_results]):.4f}")

# Detailed results table
print("\n" + "="*60)
print("Detailed Results Table")
print("="*60)

print(f"{'Technique':<20} {'ε':<10} {'Precision':<12}")
print("-" * 50)

for r in original_results:
    print(f"{'Original':<20} {r['epsilon']:<10.3f} {r['precision']:<12.4f}")

for r in network_results:
    print(f"{'Network Shuffling':<20} {r['epsilon']:<10.3f} {r['precision']:<12.4f}")

plt.show() 