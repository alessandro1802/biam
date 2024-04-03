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

def efficiency_plot(algorithm_order, save_path):
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
    
    # Efficiency plots
    save_path = './plots/quality_all.svg'
    efficiency_plot(algorithm_order, save_path)
    
    save_path = './plots/quality_no-R.svg'
    efficiency_plot(algorithm_order[:-1], save_path)
    