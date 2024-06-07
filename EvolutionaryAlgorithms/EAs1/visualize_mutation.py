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
        mutation_prob, run = file_name.split('-')[-2:]
        mutation_prob = mutation_prob[0] + '.' + mutation_prob[1:]
        mut_run_df = pd.read_csv(file_path)
        mut_run_df["run"] = run
        mut_run_df["mutation"] = mutation_prob
        logs = pd.concat([logs, mut_run_df], ignore_index=True)
    return logs

def read_HoFs(directory_path: str, attribures: List[str] = ["genotype", "vertpos"]) -> pd.DataFrame:
    # Dict[str, Dict[str, Dict[str, str]]]
    hofs = pd.DataFrame()
    for file_path in glob(join(directory_path, "*.gen")):
        file_name = file_path.split('/')[-1].split('.')[0]
        mutation_prob, run = file_name.split('-')[-2:]
        mutation_prob = mutation_prob[0] + '.' + mutation_prob[1:]
        target_values, cols = [mutation_prob, run], ["mutation", "run"]
        with open(file_path, 'r') as f:
            for entry in f.readlines()[:-1]:
                k, v = entry.strip('\n').split(':')
                if k in attribures:
                    target_values.append(v)
                    cols.append(k)
        hofs = pd.concat([hofs, pd.DataFrame([target_values], columns=cols)], ignore_index=True)
    return hofs

def read_runtimes(directory_path: str) -> pd.DataFrame:
    runtimes = pd.DataFrame()
    for file_path in glob(join(directory_path, "*.txt")):
        file_name = file_path.split('/')[-1].split('.')[0]
        mutation_prob, run = file_name.split('-')[-2:]
        mutation_prob = mutation_prob[0] + '.' + mutation_prob[1:]
        with open(file_path, 'r') as f:
            runtimes = pd.concat([runtimes,
                                  pd.DataFrame([[mutation_prob, run, float(f.readline())]],
                                               columns=["mutation", "run", "runtime"])],
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
                                "fitness", "mutation",
                                "Best population fitness",
                                join(plots_path, "best_over_generations.png"))

    # Mean best population fitness
    mean_value_over_generations_plot(best_over_generations, "fitness", "mutation",
                                     "Mean best population fitness",
                                     join(plots_path, "mean_best_over_generations.png"))

    # Mean average population fitness
    avg_over_generations = logs.copy().rename(columns={"avg": "fitness"})
    mean_value_over_generations_plot(avg_over_generations, "fitness", "mutation",
                                     "Mean average population fitness",
                                     join(plots_path, "mean_avg_over_generations.png"))

    # HoF and Runtime boxplots
    hofs_path = join(directory_path, 'HoF')
    hofs = read_HoFs(hofs_path)
    hof_fitnesses = hofs.copy().rename(columns={"vertpos": "fitness"}).astype({"fitness": float})

    runtimes_path = join(directory_path, 'runtimes')
    runtimes = read_runtimes(runtimes_path)

    grouped_boxplots([hof_fitnesses, runtimes], "mutation", ["fitness", "runtime"], ["Vertical position", "Runtime"],
                     "Boxplots per mutation probability", join(plots_path, "grouped_hof_and_runtime_boxplots.png"))
