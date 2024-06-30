#!/bin/env python
import sys, os
import pandas as pd
import matplotlib.pyplot as plt

def parse_massif_output(filename):
    snapshots = []
    with open(filename, 'r') as file:
        snapshot = {}
        for line in file:
            if line.startswith("snapshot="):
                if snapshot:
                    snapshots.append(snapshot)
                snapshot = {"time": 0, "mem_heap_B": 0, "mem_heap_extra_B": 0, "mem_stacks_B": 0}
            if "=" in line:
                key, value = line.split('=')
                key = key.strip()
                value = value.strip()
                if value.isdigit():
                    snapshot[key] = int(value)
        if snapshot:
            snapshots.append(snapshot)
    return snapshots

def main():
    for arg in sys.argv[1:]:
        print("Processing file: " + arg)
        massif_data = parse_massif_output(arg)
        
        df = pd.DataFrame(massif_data)
        
        plt.figure(figsize=(10, 6))
        plt.plot(df['time'], df['mem_heap_B'], label='Heap Memory')
        plt.plot(df['time'], df['mem_heap_extra_B'], label='Heap Extra Memory')
        plt.plot(df['time'], df['mem_stacks_B'], label='Stack Memory')
        plt.xlabel('Time (ms)')
        plt.ylabel('Memory (bytes)')
        plt.title('Memory Usage Over Time')
        plt.legend()
        plt.grid(True)
        if not os.path.exists('plots'):
            os.makedirs('plots')
        plt.savefig('plots/' + arg.replace('massif/', '') + '.png')

if __name__ == "__main__":
    main()

