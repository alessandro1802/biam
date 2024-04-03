import json
from glob import glob
from os.path import join
import numpy as np
import matplotlib.pyplot as plt


def read_results(data_path):
    results = dict()
    for intance in glob(join(data_path, "*")):
        instance_name = intance.split('/')[-1]
        results[instance_name] = dict()
        for algorithm in glob(join(intance, "*.json")):
            algorithm_name = algorithm.split('/')[-1].split('.')[0]
            with open(algorithm, 'r') as file:
                results[instance_name][algorithm_name] = json.load(file)
    return results

def quality_plot(algorithm_order, save_path):
    rows = 2
    cols = int(len(results) / 2)
    bar_width = 1

    fig, axes = plt.subplots(rows, cols, figsize = (16, 9))
    fig.tight_layout(h_pad=5)
    plt.subplots_adjust(top=0.9)

    for i, (instance_name, algrorithms) in enumerate(results.items()):
        row = i // 4
        col = i - row * 4

        pos = np.arange(len(algorithm_order)) * bar_width

        diff_best = [algrorithms[algorithm_name]["best_distance"] - optima[instance_name] for algorithm_name in algorithm_order]
        diff_avg = [np.mean(algrorithms[algorithm_name]["distances"]) - optima[instance_name] for algorithm_name in algorithm_order]

        axes[row, col].bar(pos, diff_best, edgecolor='white', width=bar_width)
        axes[row, col].bar(pos, diff_avg, bottom=diff_best, edgecolor='white', width=bar_width)

        axes[row, col].set_title(instance_name)
        axes[row, col].set_xticks(pos)
        axes[row, col].set_xticklabels(algorithm_order, fontsize=7)
        axes[row, col].tick_params(axis='y', labelsize=7)

    fig.suptitle("Distance to optimum", fontsize = 32)
    fig.legend(["Best", "Average"], loc='upper right')
    plt.savefig(save_path)

def runtime_plot(algorithm_order, save_path):
    rows = 2
    cols = int(len(results) / 2)
    bar_width = 1

    fig, axes = plt.subplots(rows, cols, figsize = (16, 9))
    fig.tight_layout(h_pad=5)
    plt.subplots_adjust(top=0.9)

    for i, (instance_name, algrorithms) in enumerate(results.items()):
        row = i // 4
        col = i - row * 4

        pos = np.arange(len(algorithm_order)) * bar_width

        runtimes_avg = [np.mean(algrorithms[algorithm_name]["runtimes"]) for algorithm_name in algorithm_order]

        axes[row, col].bar(pos, runtimes_avg, edgecolor='white', width=bar_width)

        axes[row, col].set_title(instance_name)
        axes[row, col].set_xticks(pos)
        axes[row, col].set_xticklabels(algorithm_order, fontsize=7)
        axes[row, col].tick_params(axis='y', labelsize=7)

    fig.suptitle("Runtime", fontsize = 32)
    plt.savefig(save_path)

def efficiency_plot(algorithm_order, save_path, weight_runtime = 0.5, weight_score = 0.5):
    def normilize(array):
        return (array - array.min()) / (array.max() - array.min())

    rows = 2
    cols = int(len(results) / 2)
    bar_width = 1

    fig, axes = plt.subplots(rows, cols, figsize = (16, 9))
    fig.tight_layout(h_pad=5)
    plt.subplots_adjust(top=0.9)

    for i, (instance_name, algrorithms) in enumerate(results.items()):
        row = i // 4
        col = i - row * 4

        pos = np.arange(len(algorithm_order)) * bar_width

        score_avg = normilize(np.array([np.mean(algrorithms[algorithm_name]["distances"]) for algorithm_name in algorithm_order]))
        runtimes_avg = normilize(np.array([np.mean(algrorithms[algorithm_name]["runtimes"]) for algorithm_name in algorithm_order]))
        efficiency_scores = 1 - ((weight_runtime * runtimes_avg) + (weight_score * score_avg))

        axes[row, col].bar(pos, efficiency_scores, edgecolor='white', width=bar_width)

        axes[row, col].set_title(instance_name)
        axes[row, col].set_xticks(pos)
        axes[row, col].set_xticklabels(algorithm_order, fontsize=7)
        axes[row, col].tick_params(axis='y', labelsize=7)

    fig.suptitle(f"Efficiency with Runtime weight = {weight_runtime} and Score weight = {weight_score}", fontsize = 32)
    plt.savefig(save_path)


data_path = "./results"
optima = {"berlin52": 7_542,
          "kroA100": 21_282,
          "vm1084": 239_297,
          "rat99": 1_211,
          "rat195": 2_323,
          "rat575": 6_773,
          "a280": 2_579,
          "p654": 34_643,
          "d1291": 50_801}
algorithm_order = ['greedy', 'steepest', 'heuristic', 'random_walk', 'random']


if __name__ == "__main__":
    results = read_results(data_path)
    
    # Quality plots
    save_path = './plots/quality_all.svg'
    quality_plot(algorithm_order, save_path)
    
    save_path = './plots/quality_no-R.svg'
    quality_plot(algorithm_order[:-1], save_path)
    
    # Runtime plot
    save_path = './plots/runtime.svg'
    runtime_plot(algorithm_order, save_path)

    # Efficiency plot
    save_path = './plots/efficiency.svg'
    efficiency_plot(algorithm_order, save_path)
