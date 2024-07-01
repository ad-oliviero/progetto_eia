#!/bin/env python
import sys, os, subprocess as sp, gzip as gz, random as rd, time
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

def get_random_states(dataset):
    lines = ""
    with gz.open(f'{DATA_DIR}/{dataset}', 'rt') as f:
        lines = f.read().split("\n")
    lines = [line.replace('\t', ' ').replace(',', ' ') for line in lines if not line.startswith("#")]
    max_idx = len(lines) - 1
    start_idx = rd.randint(0, max_idx)
    start = lines[start_idx].split(" ")
    end = lines[rd.randint(0, max_idx)].split(" ")
    for i in range(0, max_idx):
        if lines[i].startswith(start[1]) and lines[i].split(" ")[1] != start[0]:
            end = lines[i].split(" ")
            break

    return (start[0], end[0])

def write_latex(grafici, risultati):
    with open('doc/grafici.tex', 'w') as f:
        f.write("""\\documentclass{article}
\\usepackage{graphicx}
\\usepackage{listings}
\\usepackage{xcolor}
\\usepackage{hyperref}
\\usepackage{geometry}
\\geometry{a4paper, margin=1in}
\\title{Grafici dell'utilizzo di memoria heap da parte degli algoritmi di ricerca}
\\author{Adriano Oliviero}
\\date{\today}
\\begin{document}
\\maketitle
\\tableofcontents
\\newpage
\\section{Grafici e Risultati}
Di seguito sono riportati alcuni grafici che visualizzano i risultati sperimentali degli algoritmi di ricerca applicati ai dataset:
""")
        f.write(grafici)
    with open('doc/risultati.tex', 'w') as f:
        f.write(risultati)


def main():
    latex_grafici = ""
    latex_risultati = ""
    sp.run(['cargo', 'build', '--release'])
    sp.run(['bash', './download-datasets.sh', DATA_DIR])
    datasets = [(f, os.stat(os.path.join(DATA_DIR, f)).st_size) for f in os.listdir(DATA_DIR)]
    datasets.sort(key=lambda x: x[1]) # sort by size
    datasets = [f[0] for f in datasets]
    selected_datasets = []
    available_searches = ["breadth-first", "uniform-cost", "depth-limited", "iterative-deepening", "bi-directional"]
    selected_searches = []
    if len(sys.argv) > 1:
        for arg in sys.argv[1:]:
            if arg in available_searches:
                selected_searches.append(arg)
            else:
                selected_datasets.append(arg.replace(DATA_DIR + '/', ''))
    if not selected_datasets:
        selected_datasets = datasets
    if not selected_searches:
        selected_searches = available_searches
    os.makedirs(MASSIF_OUTPUT_DIR, exist_ok=True)
    os.makedirs(SEARCH_OUTPUT_DIR, exist_ok=True)
    os.makedirs(PLOTS_DIR, exist_ok=True)
    all_executed = True
    start_time = time.time()
    for dataset in selected_datasets:
        latex_grafici += f"\\subsection{{Dataset: {dataset}}}\n"
        latex_risultati_tabella = ""
        latex_risultati_info = ["", "", ""]
        stati = get_random_states(dataset)
        for search in selected_searches:
            massif_output_file = f'{MASSIF_OUTPUT_DIR}/{dataset.replace(".txt", "").replace(".gz", "")}_{search}'
            dataset_file = f'{DATA_DIR}/{dataset}'
            search_output_file = f'{SEARCH_OUTPUT_DIR}/{dataset.replace(".txt", "").replace(".gz", "")}_{search}.txt'
            save_file = f'{PLOTS_DIR}/{dataset.replace(".txt", "").replace(".gz", "")}_{search}.png'
            if not os.path.exists(massif_output_file):
                valgrind_command = [
                    'valgrind',
                    '--tool=massif',
                    '--time-unit=ms',
                    f'--massif-out-file={massif_output_file}',
                ]
                command = [
                    './target/release/eia',
                    '-F',
                    dataset_file,
                    '-i',
                    stati[0],
                    '-f',
                    stati[1],
                    '-r',
                    search,
                ]

                spr = sp.check_output(valgrind_command + command).decode('utf-8')
                with open(search_output_file, 'w') as f:
                    f.write(spr)
            else:
                spr = open(search_output_file, 'r').read()
                all_executed = False

            for l in spr.split("\n"):
                if l.startswith(search):
                    l = l.replace(' ', '').split("|")
                    latex_risultati_tabella += f"{l[0]} & {l[1]} & {l[2]} & {l[3]} & {l[4]} \\\\\n"

            for l in spr.split("\n"):
                if l.startswith("Tipo di Grafo: "):
                    latex_risultati_info[0] = f"{l}\n\n"
                elif l.startswith("Durata caricamento: "):
                    latex_risultati_info[1] = f"{l}\n\n"
                elif l.startswith("Inizio ricerca da: "):
                    latex_risultati_info[2] = f"Nodi cercati: "
                    l = l.replace("Inizio ricerca da: ", "").replace("verso: ", "").split(" ")
                    latex_risultati_info[2] += f"{l[0]} e {l[1]}\n\n"

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
            latex_grafici += f"\\subsubsection{{Algoritmo di ricerca: {search}}}\n"
            latex_grafici += f"\\begin{{figure}}[h]"
            latex_grafici += f"\\centering\n\\includegraphics[width=\\textwidth]{{../{save_file}}}\n\\caption{{Grafico: breadth-first su com-lj.ungraph}}\n"
            latex_grafici += f"\\end{{figure}}\n"
        latex_risultati += f"\\subsection{{{dataset}}}\n"
        for info in latex_risultati_info:
            if not len(info) == 0:
                latex_risultati += info
        latex_risultati += f"\\begin{{table}}[h]\n\\centering\n\\begin{{tabular}}{{|l|l|r|r|r|}}\n\\hline\n"
        latex_risultati += f"\\textbf{{Algoritmo}} & \\textbf{{Risultato}} & \\textbf{{Profondità}} & \\textbf{{Costo}} & \\textbf{{Tempo}} \\\\\n \\hline\n"
        latex_risultati += latex_risultati_tabella
        latex_risultati += f"\\hline\n\\end{{tabular}}\n\\caption{{{dataset}}}\n\\end{{table}}\n"
    end_time = time.time()
    latex_grafici += "\\end{document}"
    if all_executed:
        latex_risultati = f'L\'esecuzione in totale è durata {round(end_time - start_time, 3)} secondi\n\n' + latex_risultati
    write_latex(latex_grafici, latex_risultati)

if __name__ == "__main__":
    main()
