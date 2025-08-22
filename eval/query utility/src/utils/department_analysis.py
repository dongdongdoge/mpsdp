import pandas as pd
import numpy as np

# Load data
print("Loading data...")
df = pd.read_csv('retirement_dept.csv')

codes = df['Department Code'].values
unique_codes = sorted(df['Department Code'].unique())
k = len(unique_codes)
print(f"Number of unique department codes (k): {k}")

# Epsilon' values (amp-sdp)
epsilon_primes_amp = [4.545, 7.575, 10.605, 15.150, 18.180, 22.725, 25.755, 30.300]
# Epsilon' values (network shuffling)
epsilon_primes_net = [5.612, 6.293, 6.792, 7.391, 7.735, 8.190, 8.452, 8.813]

# Prepare output DataFrame
out_df = pd.DataFrame({'Original': codes})

# kRR mechanism
rng = np.random.default_rng(seed=42)
def krr_column(eps, codes, unique_codes, k, rng):
    p = np.exp(eps) / (np.exp(eps) + k - 1)
    noisy_codes = []
    for c in codes:
        if rng.random() < p:
            noisy_codes.append(c)
        else:
            other_codes = [x for x in unique_codes if x != c]
            noisy_codes.append(rng.choice(other_codes))
    return noisy_codes

# amp-sdp columns
for eps in epsilon_primes_amp:
    out_df[f'kRR_amp_eps_{eps}'] = krr_column(eps, codes, unique_codes, k, rng)
    print(f"Done kRR for amp-sdp epsilon'={eps}")

# network shuffling columns
for eps in epsilon_primes_net:
    out_df[f'kRR_net_eps_{eps}'] = krr_column(eps, codes, unique_codes, k, rng)
    print(f"Done kRR for network shuffling epsilon'={eps}")

# Save to CSV
out_df.to_csv('department_krr_simulated.csv', index=False)
print("Saved department_krr_simulated.csv with both amp-sdp and network shuffling kRR columns.")

# Show a preview
print(out_df.head()) 