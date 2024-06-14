from typing import List, Sequence, TypeVar, Optional
from random import randrange

from smallperm.smallperm import PseudoRandomPermutation as PRP

T = TypeVar("T")


def _default_seed() -> int:
    return randrange(2**32)


def sample_ix(n: int, k: int, seed: Optional[int] = None) -> List[int]:
    """Return a list of k unique integers from 0 to n-1."""
    prp = PRP(n, seed if seed is not None else _default_seed())
    return [prp[i] for i in range(k)]


def sample(seq: Sequence[T], k: int, seed: Optional[int] = None) -> List[T]:
    """Return a list of k unique elements from the input sequence."""
    return [seq[i] for i in sample_ix(len(seq), k, seed)]


def shuffle(seq: Sequence[T], seed: Optional[int] = None) -> List[T]:
    """Return a shuffled copy of the input sequence."""
    prp = PRP(len(seq), seed if seed is not None else _default_seed())
    return [seq[prp[i]] for i in range(len(seq))]
