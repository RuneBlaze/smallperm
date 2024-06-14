smallperm
=============================

Small library to generate permutations of a list of elements using pseudo-random permutations (PRP). Uses `O(1)` memory and `O(1)` time to generate the next element of the permutation.

```python
>>> from smallperm import PseudoRandomPermutation
>>> list(PseudoRandomPermutation(42, 0xDEADBEEF))
[30, 11, 23, 21, 39, 9, 26, 5, 27, 38, 15, 37, 31, 35, 6, 13, 34, 10, 7, 0, 12, 22, 33, 17, 41, 29, 18, 20, 3, 40, 25, 4, 19, 24, 32, 16, 36, 14, 1, 28, 2, 8]
```

## API

- **Initialization**: `PseudoRandomPermutation(length: int, seed: int)`
  - Generates a permutation of `[0, length)` using `seed`. We impose no restriction on `length` (except it fits under an unsigned 128-bit integer).

- **Usage**: Iterate over the instance to get the next element of the permutation.
  - Example: `list(PseudoRandomPermutation(42, 0xDEADBEEF))`

- **O(1) forward/backward mapping**:
  - `forward(i: int) -> int`: Returns the `i`-th element of the permutation (regardless of the current state of the iterator).
  - `backward(el: int) -> int`: Returns the index of `el` in the permutation.

## How

We use a (somewhat) weak albeit fast symmetric cipher to generate the permutation. The resulting shuffle quality is not as high as Fisher-Yates shuffle, but it is extremely efficient. Compared to Fisher-Yates, we use `O(1)` memory (as opposed to `O(n)`, `n` the length of the shuffle); fix $\sigma$ a permutation (i.e., `PseudoRandomPermutation(n, seed)`) which maps $\{0, 1, \ldots, n-1\}$ to itself, we have $O(1)$ $\sigma(x)$ and $\sigma^{-1}(y)$, which can be very desirable properties in distributed ML training.

Compared to naive Fisher-Yates (paired with a decent PRNG) in Python, our "give me the next card in the shuffled deck" operation is also constant-time faster. Statistically, our shuffle is not as high quality as Fisher-Yates, but it is still good enough for most applications.

## Acknowledgements

Gratefully modifies and reuses code from https://github.com/asimihsan/permutation-iterator-rs which
does most of the heavy lifting. Because the heavy lifting is done in Rust, this library is very efficient.