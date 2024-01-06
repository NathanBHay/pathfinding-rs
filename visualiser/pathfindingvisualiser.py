"""
This is a visualiser for pathfinding algorithms for more specific analysis of
"""

import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import json
from matplotlib import cm

viridis0 = cm.viridis(0.0)
viridis1 = cm.viridis(1.0)
viridis5 = cm.viridis(0.5)


def visualise_ground_truth(file_name: str):
    """
    Visualise the file_ground_truth.json file
    """
    with open(f'{file_name}_ground_truth.json') as f:
        ground_truth_obj = json.load(f)
        ground_truth = np.array(ground_truth_obj['grid']).astype(float).transpose()
        plt.imshow(ground_truth, cmap='gray', interpolation='nearest')
        if ground_truth_obj['start'] is not None:
            plt.scatter(ground_truth_obj['start'][1], ground_truth_obj['start'][0], color='b', s=200)
        if ground_truth_obj['goal'] is not None:
            plt.scatter(ground_truth_obj['goal'][1], ground_truth_obj['goal'][0], color='b', s=200)

def visualise_sample_grid(file_name: str, iteration: int):
    """
    Visualise the file_sample_grid.json file
    """
    with open(f'{file_name}_step_{iteration}.json') as f:
        sample_grid_obj = json.load(f)
        sample_grid = np.array(sample_grid_obj['sample_grid']).astype(float)
        for i in range(len(sample_grid)):
            for j in range(len(sample_grid[i])):

                plt.text(j, i, round(sample_grid[i, j], 4), ha="center", va="center", color=cm.Greys(sample_grid[i, j]))

        paths = sample_grid_obj['paths']
        for path in paths:
            for p in path[1]:
                # Cmap line colour based on p[1]
                plt.plot([path[0][0], p[0][0]], [path[0][1], p[0][1]], c=cm.viridis(p[1]), linewidth=4)
        
        if sample_grid_obj['start'] is not None:
            plt.scatter(sample_grid_obj['start'][0], sample_grid_obj['start'][1], color=viridis1, s=200)
        if sample_grid_obj['goal'] is not None:
            plt.scatter(sample_grid_obj['goal'][0], sample_grid_obj['goal'][0], color=viridis0, s=200)
        if sample_grid_obj['next'] is not None:
            plt.scatter(sample_grid_obj['next'][0], sample_grid_obj['next'][0], color=viridis5, s=200)

def visualise_final_path(file_name: str):
    """
    Visualise the file_final_path.json file
    """
    with open(f'{file_name}_final_path.json') as f:
        final_path_obj = json.load(f)
        if final_path_obj['start'] is not None:
            plt.scatter(final_path_obj['start'][1], final_path_obj['start'][0], color=viridis1, s=200)
        if final_path_obj['goal'] is not None:
            plt.scatter(final_path_obj['goal'][1], final_path_obj['goal'][0], color=viridis0, s=200)
        paths = final_path_obj['paths']
        for i in range(len(paths)-1):
            print(paths[i])
            plt.plot([paths[i][0], paths[i+1][0]], [paths[i][1], paths[i+1][1]], c=viridis5, linewidth=4)



visualise_ground_truth('test')
visualise_sample_grid('test', 22)
# visualise_final_path('test')
plt.show()
