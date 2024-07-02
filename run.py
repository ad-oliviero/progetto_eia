#!/bin/env python
import sys, os, subprocess as sp, gzip as gz, random as rd, threading as th, time as tm
import pandas as pd
import matplotlib as mpl
import matplotlib.pyplot as plt
import seaborn as sns

VALGRIND_ENABLED = os.environ.get('VALGRIND_ENABLED', 'true').lower() == 'true'
RUN_ENABLED = os.environ.get('RUN_ENABLED', 'true').lower() == 'true'

DATA_DIR = 'data'
MASSIF_OUTPUT_DIR = 'massif'
SEARCH_OUTPUT_DIR = 'search_output'
PLOTS_DIR = 'plots'

latex_grafici = ""
latex_risultati = ""

latex_risultati_mutex = th.Lock()
latex_grafici_mutex = th.Lock()
plot_mutex = th.Lock()

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
    max_idx = 500 # limit to 500 lines as it should be faster to search
    start_idx = rd.randint(0, max_idx)
    start = lines[start_idx].split(" ")
    end = start
    while start[0] == end[0]:
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

def searches_on_dataset(dataset: str, searches, stati):
    global latex_grafici, latex_risultati
    risultati_tabella = ""
    risultati_info = ["", "", ""]
    grafici_local = ""
    risultati_local = f"\\subsection{{{dataset}}}\n"
    dataset_file = f'{DATA_DIR}/{dataset}'
    for search in searches:
        massif_output_file = f'{MASSIF_OUTPUT_DIR}/{dataset.replace(".txt", "").replace(".gz", "")}_{search}'
        search_output_file = f'{SEARCH_OUTPUT_DIR}/{dataset.replace(".txt", "").replace(".gz", "")}_{search}.txt'
        save_file = f'{PLOTS_DIR}/{dataset.replace(".txt", "").replace(".gz", "")}_{search}.png'
        if stati[0] == -1 and stati[1] == -1:
            content = ""
            with open(massif_output_file, 'r') as f:
                content = f.read().split("\n")
            for l in content:
                if l.startswith("cmd: ./target/release/eia"):
                    stati = (l.split("-i ")[1].split(" ")[0], l.split("-f ")[1].split(" ")[0])
                    print(f"Using previously selected states: {stati[0]} -> {stati[1]}")
            
        if RUN_ENABLED:
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

            if VALGRIND_ENABLED:
                spr = sp.run(valgrind_command + command, stdout=sp.PIPE, stderr=sp.PIPE, universal_newlines=True).stdout
            else:
                spr = sp.run(command, stdout=sp.PIPE, stderr=sp.PIPE, universal_newlines=True).stdout
            with open(search_output_file, 'w') as f:
                f.write(spr)
        else:
            spr = open(search_output_file, 'r').read()

        for l in spr.split("\n"):
            if l.startswith(search):
                l = l.replace(' ', '').split("|")
                risultati_tabella += f"{l[0]} & {l[1]} & {l[2]} & {l[3]} & {l[4]} \\\\\n"

        for l in spr.split("\n"):
            if l.startswith("Tipo di Grafo: "):
                risultati_info[0] = f"{l}\n\n"
            elif l.startswith("Durata caricamento: "):
                risultati_info[1] = f"{l}\n\n"
            elif l.startswith("Inizio ricerca da: "):
                risultati_info[2] = f"Nodi cercati: "
                l = l.replace("Inizio ricerca da: ", "").replace("verso: ", "").split(" ")
                risultati_info[2] += f"{l[0]} e {l[1]}\n\n"
        massif_output = parse_massif_output(massif_output_file)
        df = pd.DataFrame(massif_output)
        sns.set_style("whitegrid")
        sns.set_context("talk")

        with plot_mutex:
            fig, ax = plt.subplots(figsize=(19, 10))

            ax.plot(df['time'], df['mem_heap_B'], label='Heap Size')
            ax.plot(df['time'], df['mem_heap_extra_B'], label='Heap Extra Size')

            ax.set_title(f'Memory Usage Over Time for search {search} on {dataset}')
            ax.set_xlabel('Time (s)')
            ax.set_ylabel('Memory (MB)')

            ax.legend(loc='upper right', frameon=True, fontsize=12)

            ax.grid(True, linestyle='--', alpha=0.5)
            plt.savefig(save_file, bbox_inches='tight', dpi=300)
            plt.close(fig)
        grafici_local += f"\\subsubsection{{Algoritmo di ricerca: {search}}}\n"
        grafici_local += f"\\begin{{figure}}[h]"
        grafici_local += f"\\centering\n\\includegraphics[width=\\textwidth]{{../{save_file}}}\n\\caption{{Grafico: breadth-first su com-lj.ungraph}}\n"
        grafici_local += f"\\end{{figure}}\n"
        print(f"[\x1b[32m{dataset}\x1b[0m]: completed [{search}]", flush=True)
    for info in risultati_info:
        if not len(info) == 0:
            risultati_local += info
    risultati_local += f"\\begin{{table}}[h]\n\\centering\n\\begin{{tabular}}{{|l|l|r|r|r|}}\n\\hline\n"
    risultati_local += f"\\textbf{{Algoritmo}} & \\textbf{{Risultato}} & \\textbf{{Profondità}} & \\textbf{{Costo}} & \\textbf{{Tempo}} \\\\\n \\hline\n"
    risultati_local += risultati_tabella
    risultati_local += f"\\hline\n\\end{{tabular}}\n\\caption{{{dataset}}}\n\\end{{table}}\n"
    with latex_risultati_mutex:
        latex_risultati += risultati_local
    with latex_grafici_mutex:
        latex_grafici += grafici_local
    print(f"[\x1b[31m{dataset}\x1b[0m]: \x1b[32mCOMPLETED\x1b[0m {tm.strftime('%Y-%m-%d %H:%M:%S', tm.localtime())}", flush=True)


def main():
    global latex_grafici, latex_risultati
    sp.run(['cargo', 'build', '--release'])
    sp.run(['bash', './download-datasets.sh', DATA_DIR])
    datasets = [(f, os.stat(os.path.join(DATA_DIR, f)).st_size) for f in os.listdir(DATA_DIR)]
    datasets.sort(key=lambda x: x[1]) # sort by size
    datasets = [f[0] for f in datasets]
    selected_datasets = []
    available_searches = ["breadth-first", "uniform-cost", "depth-limited", "iterative-deepening", "bi-directional"]
    selected_searches = []
    mpl.use('Agg') # non-interactive backend
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
    plt.rcParams['figure.max_open_warning'] = len(selected_datasets) * len(selected_searches)
    os.makedirs(MASSIF_OUTPUT_DIR, exist_ok=True)
    os.makedirs(SEARCH_OUTPUT_DIR, exist_ok=True)
    os.makedirs(PLOTS_DIR, exist_ok=True)
    threads = []
    for dataset in selected_datasets:
        latex_grafici += f"\\subsection{{Dataset: {dataset}}}\n"
        if RUN_ENABLED and VALGRIND_ENABLED:
            stati = get_random_states(dataset)
            print(f"Selected {stati[0]} -> {stati[1]} for dataset {dataset}")
        else:
            stati = (-1, -1)
        threads.append(th.Thread(target=searches_on_dataset, args=(dataset, selected_searches, stati)))

    for t in threads:
        t.start()
    for t in threads:
        t.join()
    latex_grafici += "\\end{document}"
    write_latex(latex_grafici, latex_risultati)

if __name__ == "__main__":
    main()
