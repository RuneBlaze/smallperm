"""
A small library to generate permutations of a list of elements using pseudo-random permutations (PRP). 
A PRP uses `O(1)` memory in total and expected `O(1)` time to generate the next element of the permutation.
"""

from smallperm.smallperm import PseudoRandomPermutation as _PRP
from random import randint
from typing import Iterator, overload
from collections.abc import Sequence


class PseudoRandomPermutation(Sequence[int]):
    """
    A class representing a pseudo-random permutation generator.

    This class generates a pseudo-random permutation of a given length using a seed value.

    Attributes:
        _inner (_PRP): The inner pseudo-random permutation generator.
        _n (int): The length of the permutation.
        _seed (int): The seed for the pseudo-random generator.

    Methods:
        __init__(self, n: int, seed: int = None): Initialize a pseudo-random permutation generator.
        __iter__(self) -> Iterator[int]: Return an iterator for the pseudo-random permutation.
        __len__(self) -> int: Return the length of the permutation.
        __getitem__(self, i): Get the element(s) at the given index(es) in the permutation.
        forward(self, i: int) -> int: Return the element at the given index in the permutation.
        backward(self, i: int) -> int: Return the index of the given element in the permutation.
        inner(self) -> _PRP: Get the inner pseudo-random permutation generator.
    """

    def __init__(self, n: int, seed: int = None):
        """
        Initialize a pseudo-random permutation generator.

        Args:
            n (int): The length of the permutation. It is an unsigned 128-bit integer.
            seed (int | None): The seed for the pseudo-random generator. It is an unsigned 64-bit integer.
                If `None`, a random seed using builtin `random` is chosen.
        """
        if seed is None:
            seed = randint(0, 2**64 - 1)
        self._inner = _PRP(n, seed)
        self._n = n
        self._seed = seed

    def __iter__(self) -> Iterator[int]:
        """
        Return an iterator for the pseudo-random permutation.

        Returns:
            Iterator[int]: An iterator that yields the elements of the permutation.
        """
        return self._inner

    def __len__(self) -> int:
        """
        Return the length of the permutation.

        Returns:
            int: The length of the permutation.
        """
        return self._n

    @overload
    def __getitem__(self, i: int) -> int:
        ...

    @overload
    def __getitem__(self, i: slice) -> Sequence[int]:
        ...

    def __getitem__(self, i):
        if isinstance(i, slice):
            rng = range(len(self))[i]
            return [self[j] for j in rng]
        if isinstance(i, int):
            if i < 0:
                i += len(self)
                if i < 0:
                    raise IndexError("index out of range")
            return self._inner[i]
        raise TypeError("indices must be integers or slices")

    def forward(self, i: int) -> int:
        """
        Return the element at the given index in the permutation.
        This takes expected `O(1)` time.

        Args:
            ix (int): The index of the element in the permutation.

        Returns:
            int: The element at the given index in the permutation.
        """
        return self._inner.forward(i)

    def backward(self, i: int) -> int:
        """
        Return the index of the given element in the permutation,
        in other words, the inverse of the permutation.
        This takes expected `O(1)` time.

        Args:
            el (int): The element in the permutation.

        Returns:
            int: The index of the given element in the permutation.
        """
        return self._inner.backward(i)

    @property
    def inner(self) -> _PRP:
        """
        Get the inner pseudo-random permutation generator.

        Returns:
            _PRP: The inner pseudo-random permutation generator.
        """
        return self._inner
