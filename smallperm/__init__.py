from smallperm.smallperm import (
    PseudoRandomPermutation as EfficientPseudoRandomPermutation,
)

from .extras import sample_ix, sample, shuffle
from .wrapper import PseudoRandomPermutation

__all__ = [
    "PseudoRandomPermutation",
    "sample_ix",
    "sample",
    "shuffle",
    "EfficientPseudoRandomPermutation",
]
