"""
This is a visualiser for pathfinding algorithms for more specific analysis of
"""

import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import json
from matplotlib import cm, ticker

viridis1 = cm.viridis(1.0)

def visualise_ground_truth(ax: plt.Axes, file_name: str) -> plt.Axes:
    """
    Visualise the file_ground_truth.json file
    """
    ax.set_title(f'{file_name.capitalize()} Ground Truth')
    with open(f'{file_name}_ground_truth.json') as f:
        ground_truth_obj = json.load(f)
        ground_truth = np.array(ground_truth_obj['grid']).astype(bool).transpose()
        ax.imshow(ground_truth, cmap='gray')
        if ground_truth_obj['start'] is not None:
            ax.scatter(ground_truth_obj['start'][0], ground_truth_obj['start'][1], color=viridis1, s=200, facecolors='none')
        if ground_truth_obj['goal'] is not None:
            ax.scatter(ground_truth_obj['goal'][0], ground_truth_obj['goal'][1], color=viridis1, s=200, marker='s')
    return ax

def visualise_sample_grid(ax: plt.Axes, file_name: str, iteration: int):
    """
    Visualise the file_sample_grid.json file
    """
    ax.set_title(f'{file_name.capitalize()} Sample Grid')
    with open(f'{file_name}_step_{iteration}.json') as f:
        sample_grid_obj = json.load(f)
        sample_grid = np.array(sample_grid_obj['sample_grid']).astype(float)
        for i in range(len(sample_grid)):
            for j in range(len(sample_grid[i])):
                ax.text(i, j, round(sample_grid[i, j], 3), ha="center", va="center", color=cm.Greys(sample_grid[i, j]))

        paths = sample_grid_obj['paths']
        for path in paths:
            for p in path[1]:
                ax.plot([path[0][0], p[0][0]], [path[0][1], p[0][1]], c=cm.viridis(p[1]), linewidth=4)

        if sample_grid_obj['start'] is not None:
            ax.scatter(sample_grid_obj['start'][0], sample_grid_obj['start'][1], color=viridis1, s=200)
        if sample_grid_obj['goal'] is not None:
            ax.scatter(sample_grid_obj['goal'][0], sample_grid_obj['goal'][1], color=viridis1, s=200, marker='s')
        if sample_grid_obj['next'] is not None:
            marker = '*'
            if sample_grid_obj['start'] is not None:
                diff = (sample_grid_obj['next'][0] - sample_grid_obj['start'][0], sample_grid_obj['next'][1] - sample_grid_obj['start'][1])
                if diff == (0, 0): marker = 'o'
                elif diff == (0, -1): marker = '^'
                elif diff == (0, 1): marker = 'v'
                elif diff == (1, 0): marker = '>'
                elif diff == (-1, 0): marker = '<'
            ax.scatter(sample_grid_obj['next'][0], sample_grid_obj['next'][1], color=viridis1, s=200, marker=marker)

def visualise_final_path(ax: plt.Axes, file_name: str):
    """
    Visualise the file_final_path.json file
    """
    ax.set_title(f'{file_name.capitalize()} Final Path')
    with open(f'{file_name}_final_path.json') as f:
        final_path_obj = json.load(f)
        if final_path_obj['start'] is not None:
            ax.scatter(final_path_obj['start'][0], final_path_obj['start'][1], color=viridis1, s=200)
        if final_path_obj['goal'] is not None:
            ax.scatter(final_path_obj['goal'][0], final_path_obj['goal'][1], color=viridis1, s=200, marker='s')
        paths = final_path_obj['paths']

        for path in paths:
            for p in path[1]:
                ax.plot([path[0][0], p[0][0]], [path[0][1], p[0][1]], c=cm.viridis(p[1]), linewidth=4)

def visualise_steps(file_name: str, limit: int = 1000):
    _, ax = plt.subplots()
    visualise_ground_truth(ax, file_name)
    plt.savefig(f'{file_name}_ground_truth.png')
    ax.cla()
    for i in range(1, limit+1):
        visualise_ground_truth(ax, file_name)
        visualise_sample_grid(ax, file_name, i)
        plt.savefig(f'{file_name}_step_{i}.png')
        ax.cla()
    visualise_ground_truth(ax, file_name)
    visualise_final_path(ax, file_name)
    plt.savefig(f'{file_name}_final_path.png')
    ax.cla()

visualise_steps('test', 0)
# _, ax = plt.subplots()
# visualise_ground_truth(ax, 'test')
# visualise_sample_grid(ax, 'test', 1)
# plt.show()