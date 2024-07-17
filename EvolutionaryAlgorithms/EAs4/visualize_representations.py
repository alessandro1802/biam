import sys
from glob import glob
from os.path import join
from typing import List
import pandas as pd
import numpy as np

sys.path.append("../")
from visualize import *

import seaborn as sns
import matplotlib.pyplot as plt


def read_logs(directory_path: str) -> pd.DataFrame:
    logs = pd.DataFrame()
    for file_path in glob(join(directory_path, "*.csv")):
        file_name = file_path.split('/')[-1].split('.')[0]
        representation, run = file_name.split('-')[-2:]
        params_run_df = pd.read_csv(file_path)
        params_run_df["run"] = run
        params_run_df["representation"] = representation
        logs = pd.concat([logs, params_run_df], ignore_index=True)
    return logs


def read_HoFs(directory_path: str, attribures: List[str] = ["genotype", "vertpos"]) -> pd.DataFrame:
    # Dict[str, Dict[str, Dict[str, str]]]
    hofs = pd.DataFrame()
    for file_path in glob(join(directory_path, "*.gen")):
        file_name = file_path.split('/')[-1].split('.')[0]
        genformat, run = file_name.split('-')[-2:]
        # target_values, cols = [genformat, run], ["genformat", "run"]
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
        df["genformat"] = genformat
        df["run"] = run
        hofs = pd.concat([hofs, df], ignore_index=True)
    return hofs


def read_runtimes(directory_path: str) -> pd.DataFrame:
    runtimes = pd.DataFrame()
    for file_path in glob(join(directory_path, "*.txt")):
        file_name = file_path.split('/')[-1].split('.')[0]
        representation, run = file_name.split('-')[-2:]
        with open(file_path, 'r') as f:
            runtimes = pd.concat([runtimes,
                                  pd.DataFrame([[representation, run, float(f.readline())]],
                                               columns=["representation", "run", "runtime"])],
                                 ignore_index=True)
    return runtimes


def read_mutants(directory_path: str, remove_parent=True) -> pd.DataFrame:
    mutants = pd.DataFrame()
    for file_path in glob(join(directory_path, "*.csv")):
        file_name = file_path.split('/')[-1].split('.')[0]
        representation, run = file_name.split('-')[-2:]

        params_run_df = pd.read_csv(file_path)
        params_run_df["run"] = run
        params_run_df["representation"] = representation
        params_run_df["Parent Fitness"] = params_run_df["Fitness"].iloc[0]
        # Arrange step column to start from 0 till the end
        params_run_df["Step"] = range(len(params_run_df))
        if remove_parent:
            params_run_df = params_run_df.iloc[1:]
        mutants = pd.concat([mutants, params_run_df], ignore_index=True)
    return mutants


def read_crossover(directory_path: str) -> pd.DataFrame:
    crossover = pd.DataFrame()
    for file_path in glob(join(directory_path, "*.csv")):
        representation = file_path.split('/')[-1].split('.')[0]

        crossover_run_df = pd.read_csv(file_path)
        crossover_run_df["representation"] = representation
        # Replace the Parent 1 Fitness and Parent 2 Fitness with the average of the two
        crossover_run_df["Average Parent Fitness"] = (crossover_run_df["Parent 1 Fitness"] +
                                                      crossover_run_df["Parent 2 Fitness"]) / 2
        crossover_run_df = crossover_run_df.drop(columns=["Parent 1 Fitness", "Parent 2 Fitness"])
        crossover = pd.concat([crossover, crossover_run_df], ignore_index=True)
    # Drop the rows where the fitness for the child is -1
    crossover = crossover[crossover["Offspring Fitness"] != -1]
    return crossover


def scatter_plot_parent_child(data: pd.DataFrame, x_name: str, y_name: str, hue_name: str,
                              save_paths: List[str]) -> None:
    fig = plt.figure(figsize=(16, 9))
    ax = sns.scatterplot(data=data, x=x_name, y=y_name,
                         hue=hue_name, hue_order=sorted(data[hue_name].unique()))
    # y = x line
    x = np.linspace(*ax.get_xlim())
    ax.plot(x, x, color='black', linestyle='--')

    fig.suptitle("Parent vs Child fitness", fontsize=20)
    plt.savefig(save_paths[0])

    # Do the same plot but for every hue value
    for hue in data[hue_name].unique():
        fig = plt.figure(figsize=(16, 9))
        ax = sns.scatterplot(data=data[data[hue_name] == hue], x=x_name, y=y_name)
        # y = x line
        x = np.linspace(*ax.get_xlim())
        ax.plot(x, x, color='black', linestyle='--')

        fig.suptitle(f"Parent vs Child fitness for {hue} representation", fontsize=20)
        plt.savefig(save_paths[1].format(hue))


def random_walk_progress_plot(data: pd.DataFrame, x_name: str, y_name: str, color_name: str, hue_name: str) -> None:
    data[color_name] = None
    names = data[hue_name].unique()
    for name in names:
        fig, ax = plt.subplots(1, 1, figsize=(16, 9))
        data_name = data[data[hue_name] == name].copy()
        # Assign colors based on the Parent Fitness value
        # best 25% – green, next 25% – blue, next 25% – orange, worst 25% – red
        try:
            data_name[color_name] = pd.qcut(data_name["Parent Fitness"], 4,
                                            labels=["red", "orange", "blue", "green"], duplicates="drop")
        except ValueError:
            cut_result = pd.qcut(data_name["Parent Fitness"], 4, duplicates='drop')
            num_bins = cut_result.nunique()
            labels = ["red", "orange", "blue", "green"][:num_bins]
            if num_bins == 0:  # Same starting fitness
                data_name[color_name] = "red"
            else:
                data_name[color_name] = pd.qcut(data_name["Parent Fitness"], num_bins,
                                                labels=labels, duplicates='drop')

        # Iterate over each row and plot it with the corresponding color, the label is stored in the 'run' column
        for run in data_name["run"].unique():
            run_data = data_name[data_name["run"] == run]
            sns.lineplot(data=run_data, x=x_name, y=y_name, color=run_data[color_name].iloc[0], label=run, ax=ax)

        handles, labels = ax.get_legend_handles_labels()
        ax.legend(handles,
                  sorted(data_name["run"].unique(), key=lambda x: int(x.split('-')[-1])),
                  bbox_to_anchor=(1, 1))
        fig.suptitle("Random walk mutation progress", fontsize=20)
        plt.savefig(join(plots_path, f"random_walk_mutation_{name}.png"))


if __name__ == '__main__':
    directory_path = sys.argv[1]
    plots_path = join(directory_path, 'plots')

    # Best population fitness
    logs_path = join(directory_path, 'logs')
    logs = read_logs(logs_path)
    runs = 20

    best_over_generations = logs.copy().rename(columns={"max": "fitness"})
    best_over_generations = best_over_generations[best_over_generations["run"].astype(int) <= runs]
    # Mean best population fitness
    mean_value_over_generations_plot(best_over_generations, "fitness", "representation",
                                     "Mean best population fitness",
                                     join(plots_path, "mean_best_over_generations.png"))

    # Mean average population fitness
    avg_over_generations = logs.copy().rename(columns={"avg": "fitness"})
    avg_over_generations = avg_over_generations[avg_over_generations["run"].astype(int) <= runs]
    mean_value_over_generations_plot(avg_over_generations, "fitness", "representation",
                                     "Mean average population fitness",
                                     join(plots_path, "mean_avg_over_generations.png"))

    # HoF and Runtime boxplots
    hofs_path = join(directory_path, 'HoF')
    hofs = read_HoFs(hofs_path)
    hof_fitnesses = hofs.copy().rename(columns={"vertpos": "fitness",
                                                "genformat": "representation"}).astype({"fitness": float})

    runtimes_path = join(directory_path, 'runtimes')
    runtimes = read_runtimes(runtimes_path)

    grouped_boxplots([hof_fitnesses, runtimes], "representation", ["fitness", "runtime"],
                     ["Vertical position", "Runtime"],
                     "Boxplots per representation", join(plots_path, "grouped_hof_and_runtime_boxplots.png"))

    # Mutations fitness
    mutations_path = join(directory_path, 'mutants')
    mutations = read_mutants(mutations_path)

    # Percentages of better children
    child_better = mutations[mutations["Fitness"] > mutations["Parent Fitness"]]
    n_child_better = child_better.groupby(["representation"]).agg(['count'])[('Fitness', 'count')]
    n_total_mutants = mutations.groupby(["representation"]).agg(['count'])[('Fitness', 'count')]
    print("Percentages of children better than parents:")
    print("On average:", len(child_better.index) / len(mutations.index))
    print(n_child_better / n_total_mutants)
    print()

    scatter_plot_parent_child(mutations, "Parent Fitness", "Fitness", "representation",
                              [join(plots_path, "mutatant_fitnesses.png"),
                               join(plots_path, "mutatant_fitnesses_{}.png")])

    # Cross-over fitness
    cross_over_path = join(directory_path, 'offspring')
    cross_over = read_crossover(cross_over_path)

    # AFG
    cross_over["AFG"] = cross_over["Offspring Fitness"] - cross_over["Average Parent Fitness"]
    print("Average AFG:", cross_over["AFG"].mean())
    print(cross_over.groupby(["representation"]).mean())

    scatter_plot_parent_child(cross_over, "Average Parent Fitness", "Offspring Fitness", "representation",
                              [join(plots_path, "cross-over_fitnesses.png"),
                               join(plots_path, "cross-over_fitnesses_{}.png")])

    # Random Walk mutation
    selection_path = join(directory_path, 'mutants_sequence')
    selection = read_mutants(selection_path, remove_parent=False)
    selection["run"] = selection["representation"] + '-' + selection["run"]

    random_walk_progress_plot(selection, "Step", "Fitness",
                              "color", "representation")
