# Data Directory

This directory contains the datasets used in the shuffle DP analysis project.

## Directory Structure

```
data/
├── retirement/                    # Retirement datasets
│   ├── employee-compensation.csv  # Original employee compensation data
│   ├── retirement_filtered.csv    # Filtered retirement data (Retirement > 0)
│   └── retirement_dept.csv        # Department code data
└── fire/                          # Fire alarm datasets
    ├── Fire_alarm.csv             # Fire alarm data
    └── Fire_Department_Calls_for_Service.csv  # Fire department calls data
```

## Data Sources

### Retirement Data
- **Source**: Employee compensation dataset
- **Size**: ~150MB (original), ~131MB (filtered)
- **Records**: 579,199 (original), filtered for Retirement > 0
- **Columns**: 22 columns including Job Family, Retirement, etc.

### Fire Alarm Data
- **Source**: Fire department calls dataset
- **Size**: ~329KB (Fire_alarm.csv), ~1.6GB (Fire_Department_Calls_for_Service.csv)
- **Records**: 112,437 (Fire_alarm.csv)
- **Columns**: Time stamps, location data, response times

## Data Processing

### Retirement Data Processing
1. **Filtering**: Extract records where Retirement > 0
2. **Job Family Analysis**: Identify top 5 job families by count
3. **Department Analysis**: Analyze department code distributions

### Fire Alarm Data Processing
1. **Time Difference Calculation**: Compute time differences between received and dispatch times
2. **Binary Classification**: Create binary features for time differences < 1 minute
3. **Range Queries**: Analyze time difference distributions

## Privacy Considerations

All data processing implements differential privacy mechanisms:
- **k-Randomized Response (kRR)**: For categorical data
- **Laplace Mechanism**: For numerical data
- **Randomized Response (RR)**: For binary data

## Usage

To use these datasets:

1. Place the data files in the appropriate subdirectories
2. Update file paths in the analysis scripts
3. Run the analysis scripts from the project root

## Note

The actual data files are not included in this repository due to size constraints and privacy considerations. Users should obtain the datasets from appropriate sources and place them in this directory structure. 