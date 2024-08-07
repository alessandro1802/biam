import sys
from glob import glob
from os.path import join
from typing import List
import pandas as pd

sys.path.append("../")
from visualize import *


def read_logs(directory_path: str) -> pd.DataFrame:
    logs = pd.DataFrame()
    for file_path in glob(join(directory_path, "*.csv")):
        file_name = file_path.split('/')[-1].split('.')[0]
        parameters, run = file_name.split('-')[-2:]
        parameters = int(parameters)
        params_run_df = pd.read_csv(file_path)
        params_run_df["run"] = run
        params_run_df["parameters"] = parameters
        logs = pd.concat([logs, params_run_df], ignore_index=True)
    return logs


def read_HoFs(directory_path: str, attribures: List[str] = ["genotype", "vertpos"]) -> pd.DataFrame:
    # Dict[str, Dict[str, Dict[str, str]]]
    hofs = pd.DataFrame()
    for file_path in glob(join(directory_path, "*.gen")):
        file_name = file_path.split('/')[-1].split('.')[0]
        parameters, run = file_name.split('-')[-2:]
        parameters = int(parameters)
        target_values = [[] for _ in attribures]
        with open(file_path, 'r') as f:
            f0 = False
            for entry in f.readlines()[:-1]:
                # Skip empty rows between solutions
                if entry == '\n':
                    continue
                # Read f0 genotype rows
                if f0:
                    # Finish reading f0 genotype rows
                    if '~' in entry:
                        f0 = False
                        col_idx = attribures.index("genotype")
                        target_values[col_idx].append(genotype)
                    else:
                        genotype += entry
                    continue
                k, v = entry.strip('\n').split(':', 1)
                # Start reading f0 genotype rows
                if v == '~':
                    f0 = True
                    genotype = ""
                    continue
                if k in attribures:
                    col_idx = attribures.index(k)
                    target_values[col_idx].append(v)
        df = pd.DataFrame(zip(*target_values), columns=attribures)
        df["parameters"] = parameters
        df["run"] = run
        hofs = pd.concat([hofs, df], ignore_index=True)
    return hofs


def read_runtimes(directory_path: str) -> pd.DataFrame:
    runtimes = pd.DataFrame()
    for file_path in glob(join(directory_path, "*.txt")):
        file_name = file_path.split('/')[-1].split('.')[0]
        parameters, run = file_name.split('-')[-2:]
        parameters = int(parameters)
        with open(file_path, 'r') as f:
            runtimes = pd.concat([runtimes,
                                  pd.DataFrame([[parameters, run, float(f.readline())]],
                                               columns=["parameters", "run", "runtime"])],
                                 ignore_index=True)
    return runtimes


if __name__ == '__main__':
    directory_path = sys.argv[1]
    plots_path = join(directory_path, 'plots')

    # Best population fitness
    logs_path = join(directory_path, 'logs')
    logs = read_logs(logs_path)

    best_over_generations = logs.copy().rename(columns={"max": "fitness"})
    value_over_generations_plot(best_over_generations,
                                "fitness", "parameters",
                                "Best population fitness",
                                join(plots_path, "best_over_generations.png"))

    # Mean best population fitness
    mean_value_over_generations_plot(best_over_generations, "fitness", "parameters",
                                     "Mean best population fitness",
                                     join(plots_path, "mean_best_over_generations.png"))

    # Mean average population fitness
    avg_over_generations = logs.copy().rename(columns={"avg": "fitness"})
    mean_value_over_generations_plot(avg_over_generations, "fitness", "parameters",
                                     "Mean average population fitness",
                                     join(plots_path, "mean_avg_over_generations.png"))

    # HoF and Runtime boxplots
    hofs_path = join(directory_path, 'HoF')
    hofs = read_HoFs(hofs_path)
    hof_fitnesses = hofs.copy().rename(columns={"vertpos": "fitness"}).astype({"fitness": float})

    runtimes_path = join(directory_path, 'runtimes')
    runtimes = read_runtimes(runtimes_path)

    grouped_boxplots([hof_fitnesses, runtimes], "parameters", ["fitness", "runtime"],
                     ["Fitness", "Runtime"],
                     "Boxplots per representation", join(plots_path, "grouped_hof_and_runtime_boxplots.png"))
