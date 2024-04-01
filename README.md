# Biologically-inspired algorithms and models

Repository to store the code and results of the biologically-inspired algorithms and models course.
The course is part of the master's degree in Artificial Intelligence at the Poznan University of Technology.

## Table of contents

* [Local search](#local-search)
    * [Instances](#instances)

## Local search TSP

This part of the project is focused on solving the Traveling Salesman Problem (TSP) using local search algorithms.
The algorithms implemented are:
* Greedy
* Random
* Random Walk
* Steepest Descent
* Heuristic

### Instances

The instances used in the local search algorithms are taken from the [TSPLIB](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/).
All the optimal solution distances are calculated using the euclidean distance (EUC_2D) between the nodes.

Appropriate files are provided in the `data` directory.
| Name | Problem Type | Dimension | Optimal solution |
|------|-------------|------|------------------------|
| berlin52 | TSP | 52 | 7542 |
| kroA100 | TSP | 100 | 21282 |
| vm1084 | TSP Edge | 1084 | 239297 |
| rat99 | Grid | 99 | 1211 |
| rat195 | Grid | 195 | 2323 |
| rat575 | Grid | 575 | 6773 |
| a280 | Drilling | 280 | 2579 |
| p654 | Drilling | 654 | 34643 |
| d1291 | Drilling | 1291 | 50801 |
