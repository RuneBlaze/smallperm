"""
A small library to generate permutations of a list of elements using pseudo-random permutations (PRP). 
Uses `O(1)` memory and `O(1)` time to generate the next element of the permutation.
"""

from typing import Iterator

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