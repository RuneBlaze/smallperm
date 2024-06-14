from smallperm import PseudoRandomPermutation
from tqdm import tqdm
import numpy as np
from random_permutation import RandomPermutation


# n = 2**20
# p = RandomPermutation(1_000_000_000)
# for x in tqdm(p):
#     pass

# Generate a random permutation of 1_000_000_000 elements
# arr = np.arange(1_000_000_000)
# # np.random.seed(42)
# # np.random.shuffle(arr)
# for i, _ in enumerate(tqdm(arr)):
#     x = np.random.randint(i, 1_000_000_000)
#     arr[[i, x]] = arr[[x, i]]
#     pass

perm = PseudoRandomPermutation(1_000_000_000, 42)
for _ in tqdm(perm):
    pass
