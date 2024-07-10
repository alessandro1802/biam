import os
from os.path import join
from glob import glob

import sys
import argparse

from typing import List
from itertools import combinations

from tqdm import tqdm
import pandas as pd
from FramsticksLib import FramsticksLib


def read_HoFs(directory_path: str, attribures: List[str] = ["genotype", "vertpos"]) -> pd.DataFrame:
    # Dict[str, Dict[str, Dict[str, str]]]
    hofs = pd.DataFrame()
    for file_path in glob(join(directory_path, "*.gen")):
        file_name = file_path.split('/')[-1].split('.')[0]
        genformat, run = file_name.split('-')[-2:]
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


def frams_mutate(frams_lib, genotype):
    return frams_lib.mutate([genotype])[0]


def frams_crossover(frams_lib, geno1, geno2):
    individual1 = frams_lib.crossOver(geno1, geno2)
    individual2 = frams_lib.crossOver(geno1, geno2)
    return individual1, individual2


def genotype_within_constraint(genotype, dict_criteria_values, criterion_name, constraint_value):
    REPORT_CONSTRAINT_VIOLATIONS = False
    if constraint_value is not None:
        actual_value = dict_criteria_values[criterion_name]
        if actual_value > constraint_value:
            # if REPORT_CONSTRAINT_VIOLATIONS:
            # print('Genotype "%s" assigned low fitness because it violates constraint "%s": %s exceeds threshold %s' % (genotype, criterion_name, actual_value, constraint_value))
            return False
    return True


def frams_evaluate(frams_lib, genotype):
    BAD_FITNESS = [-1] * len(
        OPTIMIZATION_CRITERIA)  # fitness of -1 is intended to discourage further propagation of this genotype via selection ("this genotype is very poor")
    data = frams_lib.evaluate([genotype])
    # print("Evaluated '%s'" % genotype, 'evaluation is:', data)
    valid = True
    try:
        first_genotype_data = data[0]
        evaluation_data = first_genotype_data["evaluations"]
        default_evaluation_data = evaluation_data[""]
        fitness = [default_evaluation_data[crit] for crit in OPTIMIZATION_CRITERIA]
    except (KeyError,
            TypeError) as e:  # the evaluation may have failed for an invalid genotype (such as X[@][@] with "Don't simulate genotypes with warnings" option), or because the creature failed to stabilize, or for some other reason
        valid = False
    # print('Problem "%s" so could not evaluate genotype "%s", hence assigned it low fitness: %s' % (str(e), genotype, BAD_FITNESS))
    if valid:
        default_evaluation_data['numgenocharacters'] = len(genotype)  # for consistent constraint checking below
        valid &= genotype_within_constraint(genotype, default_evaluation_data, 'numparts', parsed_args.max_numparts)
        valid &= genotype_within_constraint(genotype, default_evaluation_data, 'numjoints', parsed_args.max_numjoints)
        valid &= genotype_within_constraint(genotype, default_evaluation_data, 'numneurons', parsed_args.max_numneurons)
        valid &= genotype_within_constraint(genotype, default_evaluation_data, 'numconnections',
                                            parsed_args.max_numconnections)
        valid &= genotype_within_constraint(genotype, default_evaluation_data, 'numgenocharacters',
                                            parsed_args.max_numgenochars)
    if not valid:
        fitness = BAD_FITNESS
    return fitness[0]


def ensureDir(string):
    if os.path.isdir(string):
        return string
    else:
        raise NotADirectoryError(string)


def parseArguments():
    parser = argparse.ArgumentParser(
        description='Run this program with "python -u %s" if you want to disable buffering of its output.' % sys.argv[
            0])
    parser.add_argument('-path', type=ensureDir, required=True,
                        help='Path to Framsticks library without trailing slash.')
    parser.add_argument('-lib', required=False,
                        help='Library name. If not given, "frams-objects.dll" (or .so or .dylib) is assumed depending on the platform.')
    parser.add_argument('-sim', required=False, default="eval-allcriteria.sim",
                        help="The name of the .sim file with settings for evaluation, mutation, crossover, and similarity estimation. If not given, \"eval-allcriteria.sim\" is assumed by default. Must be compatible with the \"standard-eval\" expdef. If you want to provide more files, separate them with a semicolon ';'.")

    parser.add_argument('-genformat', required=False,
                        help='Genetic format for the simplest initial genotype, for example 4, 9, or B. If not given, f1 is assumed.')
    parser.add_argument('-initialgenotype', required=False,
                        help='The genotype used to seed the initial population. If given, the -genformat argument is ignored.')

    parser.add_argument('-opt', required=True,
                        help='optimization criteria: vertpos, velocity, distance, vertvel, lifespan, numjoints, numparts, numneurons, numconnections (or other as long as it is provided by the .sim file and its .expdef). For multiple criteria optimization, separate the names by the comma.')
    parser.add_argument('-popsize', type=int, default=50, help="Population size, default: 50.")
    parser.add_argument('-generations', type=int, default=5, help="Number of generations, default: 5.")
    parser.add_argument('-tournament', type=int, default=5, help="Tournament size, default: 5.")
    parser.add_argument('-pmut', type=float, default=0.9, help="Probability of mutation, default: 0.9")
    parser.add_argument('-pxov', type=float, default=0.2, help="Probability of crossover, default: 0.2")
    parser.add_argument('-execution_time_savefile', required=False,
                        help='If set, the elapsed execution time will be saved (recommended extension *.txt).')
    parser.add_argument('-hof_size', type=int, default=10, help="Number of genotypes in Hall of Fame. Default: 10.")
    parser.add_argument('-hof_savefile', required=False,
                        help='If set, Hall of Fame will be saved in Framsticks file format (recommended extension *.gen).')
    parser.add_argument('-log_savefile', required=False,
                        help='If set, the log of evolution will be saved (recommended extension *.csv).')
    parser.add_argument('-input_path', required=True)
    parser.add_argument('-output_path', required=True)

    parser.add_argument('-max_numparts', type=int, default=None, help="Maximum number of Parts. Default: no limit")
    parser.add_argument('-max_numjoints', type=int, default=None, help="Maximum number of Joints. Default: no limit")
    parser.add_argument('-max_numneurons', type=int, default=None, help="Maximum number of Neurons. Default: no limit")
    parser.add_argument('-max_numconnections', type=int, default=None,
                        help="Maximum number of Neural connections. Default: no limit")
    parser.add_argument('-max_numgenochars', type=int, default=None,
                        help="Maximum number of characters in genotype (including the format prefix, if any). Default: no limit")
    return parser.parse_args()


if __name__ == "__main__":
    # random.seed(123)  # see FramsticksLib.DETERMINISTIC below, set to True if you want full determinism
    FramsticksLib.DETERMINISTIC = False  # must be set before FramsticksLib() constructor call
    parsed_args = parseArguments()
    OPTIMIZATION_CRITERIA = parsed_args.opt.split(",")

    framsLib = FramsticksLib(parsed_args.path, parsed_args.lib, parsed_args.sim)
    # Read HoFs
    hofs_path = parsed_args.input_path

    hofs = read_HoFs(hofs_path)
    hof_fitnesses = hofs.copy().rename(columns={"vertpos": "fitness"}).astype({"fitness": float})
    # Select only the 1st run
    hof_fitnesses = hof_fitnesses[hof_fitnesses["run"] == '1']

    output_dir_path = parsed_args.output_path

    ### Mutations
    output_path = join(output_dir_path, "mutants")
    runs = 20
    prev_representation = ""
    for index, row in hof_fitnesses.iterrows():
        # Update HoF number for next representation
        if prev_representation != row["genformat"]:
            hof_number = 0
        prev_representation = row["genformat"]
        # First row = original fitness
        fitnesses = [row["fitness"]]
        # Get mutants and calculate their fitnesses
        for mutant_number in range(runs):
            mutant = frams_mutate(framsLib, row["genotype"])
            mutant_fitness = frams_evaluate(framsLib, mutant)
            if mutant_fitness != -1:
                fitnesses.append(mutant_fitness)
        df = pd.DataFrame(fitnesses, columns=["Fitness"])

        hof_number += 1
        df.to_csv(join(output_path, f'{row["genformat"]}-{hof_number}.csv'), index=False)


    ### Cross-over
    output_path = join(output_dir_path, "offspring")
    runs = 15
    representations = pd.unique(hof_fitnesses["genformat"])
    for representation in tqdm(representations, desc="Representation"):
        # Select single representation and first `runs` HoFs
        representation_df = hof_fitnesses[hof_fitnesses["genformat"] == representation].reset_index(drop=True).iloc[:runs]
        pairs = pd.DataFrame(combinations(representation_df['genotype'], 2), columns=['Parent 1', 'Parent 2'])
        fitnesses = []
        for index, row in pairs.iterrows():
            offspring1, offspring2 = frams_crossover(framsLib, row["Parent 1"], row["Parent 2"])
            fitnesses.append([representation_df[representation_df['genotype'] == row["Parent 1"]]["fitness"].values[0],
                              representation_df[representation_df['genotype'] == row["Parent 2"]]["fitness"].values[0],
                              frams_evaluate(framsLib, offspring1),
                              frams_evaluate(framsLib, offspring2)])
        df = pd.DataFrame(fitnesses, columns=["Parent 1 Fitness", "Parent 2 Fitness",
                                              "Offspring 1 Fitness", "Offspring 2 Fitness"])
        df.to_csv(join(output_path, f'{representation}.csv'), index=False)


    ### Sequence of mutations
    output_path = join(output_dir_path, "mutants_sequence")
    runs = 15
    representations = pd.unique(hof_fitnesses["genformat"])
    for representation in tqdm(representations, desc="Representation"):
        # Select single representation and first `runs` HoFs
        representation_df = hof_fitnesses[hof_fitnesses["genformat"] == representation].reset_index(drop=True).iloc[:runs]
        for index, row in representation_df.iterrows():
            # First row = original fitness
            fitnesses = [row["fitness"]]
            # Get mutants and calculate their fitnesses
            current_genotype = row["genotype"]
            for mutant_number in range(runs):
                while True:
                    mutant = frams_mutate(framsLib, current_genotype)
                    mutant_fitness = frams_evaluate(framsLib, mutant)
                    if mutant_fitness != -1:
                        fitnesses.append(mutant_fitness)
                        current_genotype = mutant
                        break
            df = pd.DataFrame(fitnesses, columns=["Fitness"])
            df.to_csv(join(output_path, f'{row["genformat"]}-{index + 1}.csv'), index=False)
