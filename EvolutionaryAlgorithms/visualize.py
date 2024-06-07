from typing import List
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns


def value_over_generations_plot(logs: pd.DataFrame, value_name: str, hue_name: str, title: str, save_path: str) -> None:
    fig = plt.figure(figsize=(16, 9))
    sns.lineplot(data=logs, x="generation", y=value_name,
                 hue=hue_name, hue_order=sorted(logs[hue_name].unique()),
                 units="run", estimator=None)
    fig.suptitle(title, fontsize=20)
    plt.savefig(save_path)


def mean_value_over_generations_plot(logs: pd.DataFrame, value_name: str, hue_name: str,
                                     title: str, save_path: str, std_scale: float = 0.1) -> None:
    fig = plt.figure(figsize=(16, 9))
    sns.lineplot(data=logs, x="generation", y=value_name,
                 hue=hue_name, hue_order=sorted(logs[hue_name].unique()),
                 errorbar=("sd", std_scale))
    fig.suptitle(title, fontsize=20)
    plt.savefig(save_path)


def grouped_boxplots(data: List[pd.DataFrame], vairiants: str, target_values: List[str],
                     data_names: List[str], title: str, save_path: str) -> None:
    fig, axes = plt.subplots(1, len(data), figsize=(16, 9))
    for i, ax in enumerate(axes.flat):
        sns.boxplot(data=data[i], x=vairiants, y=target_values[i],
                    order=sorted(data[i][vairiants].unique()), ax=ax)
        ax.set_title(data_names[i], fontsize=14)
    fig.suptitle(title, fontsize=20)
    plt.savefig(save_path)
