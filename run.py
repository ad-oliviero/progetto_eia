#!/bin/env python
import sys, os, subprocess as sp, shutil
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

DATA_DIR = 'data'
MASSIF_OUTPUT_DIR = 'massif'
SEARCH_OUTPUT_DIR = 'search_output'
PLOTS_DIR = 'plots'

def parse_massif_output(filename):
    snapshots = []
    with open(filename, 'r') as file:
        snapshot = {}
        for line in file:
            if line.startswith("snapshot="):
                if snapshot:
                    snapshots.append(snapshot)
                snapshot = {"time": 0, "mem_heap_B": 0, "mem_heap_extra_B": 0}
            if "=" in line:
                spline = line.split('=')
                if len(spline) == 2:
                    key, value = spline
                    key = key.strip()
                    value = value.strip()
                    if value.isdigit():
                        snapshot[key] = int(value)
        if snapshot:
            snapshots.append(snapshot)
    for snapshot in snapshots:
        snapshot["mem_heap_B"] /= (1024 * 1024)
        snapshot["mem_heap_extra_B"] /= (1024 * 1024)
        snapshot["time"] /= 1000
    return snapshots

def main():
    for directory in [MASSIF_OUTPUT_DIR, SEARCH_OUTPUT_DIR, PLOTS_DIR]:
        if os.path.exists(directory):
            shutil.rmtree(directory)
    sp.run(['cargo', 'build', '--release'])
    sp.run(['bash', './download-datasets.sh', DATA_DIR])
    datasets = os.listdir(DATA_DIR)
    available_searches = ["breadth-first", "uniform-cost", "depth-limited", "iterative-deepening", "bi-directional"]
    if len(sys.argv) > 1:
        for arg in sys.argv[1:]:
            if arg in available_searches:
                available_searches = [arg]
            else:
                datasets = [arg.replace(DATA_DIR + '/', '')]
    os.makedirs(MASSIF_OUTPUT_DIR, exist_ok=True)
    os.makedirs(SEARCH_OUTPUT_DIR, exist_ok=True)
    os.makedirs(PLOTS_DIR, exist_ok=True)
    for dataset in datasets:
        for search in available_searches:
            massif_output_file = f'{MASSIF_OUTPUT_DIR}/{dataset.replace(".txt", "").replace(".gz", "")}_{search}'
            dataset_file = f'{DATA_DIR}/{dataset}'
            search_output_file = f'{SEARCH_OUTPUT_DIR}/{dataset.replace(".txt", "").replace(".gz", "")}_{search}.txt'
            save_file = f'{PLOTS_DIR}/{dataset.replace(".txt", "").replace(".gz", "")}_{search}.png'
            spr = sp.check_output([
                'valgrind',
                '--tool=massif',
                '--time-unit=ms',
                f'--massif-out-file={massif_output_file}',
                './target/release/eia',
                '-F',
                dataset_file,
                '-r',
                search])
            with open(search_output_file, 'w') as f:
                f.write(spr.decode('utf-8'))

            print(dataset, end=' ')
            massif_output = parse_massif_output(massif_output_file)
            df = pd.DataFrame(massif_output)
            sns.set_style("whitegrid")
            sns.set_context("talk")

            _, ax = plt.subplots(figsize=(19, 10))

            ax.plot(df['time'], df['mem_heap_B'], label='Heap Size')
            ax.plot(df['time'], df['mem_heap_extra_B'], label='Heap Extra Size')

            ax.set_title(f'Memory Usage Over Time for search {search} on {dataset}')
            ax.set_xlabel('Time (s)')
            ax.set_ylabel('Memory (MB)')

            ax.legend(loc='upper right', frameon=True, fontsize=12)

            ax.grid(True, linestyle='--', alpha=0.5)
            plt.savefig(save_file, bbox_inches='tight', dpi=300)
            print('saved to ' + save_file)

if __name__ == "__main__":
    main()
