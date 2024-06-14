"""
A small library to generate permutations of a list of elements using pseudo-random permutations (PRP). 
A PRP uses `O(1)` memory in total and expected `O(1)` time to generate the next element of the permutation.
"""

from collections.abc import Sequence
from typing import List, TypeVar, Optional, Iterator

T = TypeVar("T")

class PseudoRandomPermutation:
    def __init__(self, length: int, seed: int):
        """
        Initialize a pseudo-random permutation generator.

        Args:
            length (int): The length of the permutation. It is an unsigned 128-bit integer.
            seed (int): The seed for the pseudo-random generator. It is an unsigned 64-bit integer.
        """
        pass

    def __iter__(self) -> Iterator[int]:
        """
        Return an iterator for the pseudo-random permutation.

        Returns:
            Iterator[int]: An iterator that yields the elements of the permutation.
        """
        pass

    def __next__(self) -> int:
        """
        Return the next element in the permutation.

        Returns:
            int: The next element in the permutation.
        """
        pass

    def forward(self, ix: int) -> int:
        """
        Return the element at the given index in the permutation.
        This takes expected `O(1)` time.

        Args:
            ix (int): The index of the element in the permutation.

        Returns:
            int: The element at the given index in the permutation.
        """
        pass

    def backward(self, el: int) -> int:
        """
        Return the index of the given element in the permutation,
        in other words, the inverse of the permutation.
        This takes expected `O(1)` time.

        Args:
            el (int): The element in the permutation.

        Returns:
            int: The index of the given element in the permutation.
        """
        pass

def sample_ix(n: int, k: int, seed: Optional[int] = None) -> List[int]:
    """Return a list of k unique integers from 0 to n-1."""
    ...

def sample(seq: Sequence[T], k: int, seed: Optional[int] = None) -> List[T]:
    """Return a list of k unique elements from the input sequence."""
    ...

def shuffle(seq: Sequence[T], seed: Optional[int] = None) -> List[T]:
    """Return a shuffled copy of the input sequence."""
    ...
