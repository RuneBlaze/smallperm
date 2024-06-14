import numpy as np
from scipy.stats import chisquare
import itertools
from smallperm import PseudoRandomPermutation
from random_permutation import RandomPermutation


# Fisher-Yates shuffle function
def fisher_yates_shuffle(lst):
    array = lst.copy()
    for i in range(len(array) - 1, 0, -1):
        j = np.random.randint(0, i + 1)
        array[i], array[j] = array[j], array[i]
    return array


# Function to generate permutations and count occurrences
def generate_permutations_and_counts(shuffle_func, lst, num_shuffles):
    observed_counts = {}
    for _ in range(num_shuffles):
        shuffled = tuple(shuffle_func(lst))
        if shuffled in observed_counts:
            observed_counts[shuffled] += 1
        else:
            observed_counts[shuffled] = 1
    return observed_counts


# List and number of shuffles
lst = list(range(4))
num_shuffles = 100000

# Generate permutations using PseudoRandomPermutation
prp_func = lambda lst: PseudoRandomPermutation(len(lst), np.random.randint(0, 2**32))
prp_counts = generate_permutations_and_counts(prp_func, lst, num_shuffles)

# Generate permutations using Fisher-Yates shuffle
fy_counts = generate_permutations_and_counts(fisher_yates_shuffle, lst, num_shuffles)

# Generate all possible permutations
all_permutations = list(itertools.permutations(lst))
num_permutations = len(all_permutations)

# Convert observed counts to arrays for comparison
prp_observed = np.array([prp_counts.get(perm, 0) for perm in all_permutations])
fy_observed = np.array([fy_counts.get(perm, 0) for perm in all_permutations])

# Expected uniform distribution
expected_uniform = np.full(num_permutations, num_shuffles / num_permutations)

# Perform chi-square test against uniform distribution for PseudoRandomPermutation
chi_square_stat_prp, p_value_prp = chisquare(f_obs=prp_observed)
print(f"PseudoRandomPermutation vs. Uniform Distribution")
print(f"Chi-square statistic: {chi_square_stat_prp}")
print(f"P-value: {p_value_prp}")

# Perform chi-square test against uniform distribution for Fisher-Yates
chi_square_stat_fy, p_value_fy = chisquare(f_obs=fy_observed)
print(f"Fisher-Yates vs. Uniform Distribution")
print(f"Chi-square statistic: {chi_square_stat_fy}")
print(f"P-value: {p_value_fy}")
