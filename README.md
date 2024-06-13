smallperm
=============================

Small library to generate permutations of a list of elements using pseudo-random permutations (PRP). Uses `O(1)` memory and `O(1)` time to generate the next element of the permutation.

```python
>>> from smallperm import PseudoRandomPermutation
>>> list(PseudoRandomPermutation(42, 0xDEADBEEF))
[14, 32, 25, 16, 0, 12, 5, 37, 30, 7, 40, 17, 27, 35, 21, 15, 1, 13, 38, 4, 9, 36, 20, 22, 24, 39, 41, 19, 3, 18, 8, 2, 29, 31, 6, 34, 11, 23, 26, 10, 28, 33]
```

## API

- **Initialization**: `PseudoRandomPermutation(length: int, seed: int)`
  - Generates a permutation of `[0, length)` using `seed`.

- **Usage**: Iterate over the instance to get the next element of the permutation.
  - Example: `list(PseudoRandomPermutation(42, 0xDEADBEEF))`

- **O(1) forward/backward mapping**:
  - `forward(i: int) -> int`: Returns the `i`-th element of the permutation.
  - `backward(el: int) -> int`: Returns the index of `el` in the permutation.


## Acknowledgements

Gratefully reuses code from https://github.com/asimihsan/permutation-iterator-rs which
does most of the heavy lifting. Because the heavy lifting is done in Rust, this library is very efficient.