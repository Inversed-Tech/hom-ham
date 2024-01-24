# Exploring the Sorting Hat strategy

This [paper](https://eprint.iacr.org/2022/757.pdf) proposes an efficient solution to homomorphically evaluate a decision tree, where the Hamming distance is used along the process, and it is computed "in the exponent". In other words, the output of the homomorphic calculation is a polynomial `X^h` such that `h` is precisely the Hamming distance. In order to compute `X^h`, it is required to follow a sequencial procedure where at each step we multiply by a monomial `X` if both input vector differ at the current position, or multiply by `1` otherwise. Note that if the computation occurred in the clear, multiplying by 1 would be not needed, but when we do it homomorphically we must do it.

The paper proposes an efficient method to homomorphically compute comparisons. Namely, it takes 98 microseconds per comparison. However, computation of XOR must happen sequentially, and since each XOR takes 600 microseconds, it turns out that, for input having thousands of bits, this is the bottleneck.

## Dependencies

For Rust, install [`libclang`](https://clang.llvm.org/docs/LibClang.html) to use [`concrete-fftw`](https://github.com/zama-ai/concrete-fftw). `libclang` might have different names depending on your package manager. Typical names are `libclang`, `libclang-dev`, `llvm`, or `llvm-dev`.

(The install instructions and links in SortingHat/README.md are outdated.)

### Source Code

The code in the `SortingHat` directory was copied from the original [repo's](https://github.com/KULeuven-COSIC/SortingHat) commit [6358bf8ba741f2ba678283c657c03975b17851ca](https://github.com/KULeuven-COSIC/SortingHat/tree/6358bf8ba741f2ba678283c657c03975b17851ca).

## Experiments

Next code is a unit test added to file rlwe.rs from . The purpose of this unit test is to measure time for comparisons, XORs, and additions.

XOR operations were computed using CMUX, and a current line of work is to optimize this computation.

### Running the Experiments

To run the experiments, use these commands:
```sh
cd SortingHat/src/rust_pdte
cargo test --release -- test_less_eq_ham --nocapture
```

### Experimental Outcomes

With this information in our hands, we can already quickly sketch scenarios and estimate time for the homomorphic evaluations. For example, using fast comparisons (98 micsecs) and 1000 additions (400 ms) for counting bits, and 1000 XORs (600 ms, the bottleneck). Total is 1 second.
 If after filtering we have smaller features, and consequently more comparisons, then we can already estimate the cost. Consider 5 features of 200 bits, then 5 comparisons. Meaning the cost is `5*98 micsecs + ~80ms + ~120 ms = 200.5 ms`. As a second example, using lots of small features, say 64 features of 16 bits (closer to what was explored in the underlying paper), then the cost is: `64*98 micsecs + 16*0.4 ms + 16*0.6 ms = 6.3 ms + 16 ms = 22.3 ms`.

Hence it is possible to reduce orders of magnitude just by finding the best approach and using the right tools for each step. A qualitative conclusion is that low resolution matching or filtering could benefit a lot from fast comparisons. But as the size of the input grows this situation quickly changes. For thousands of bits, i.e. for high resolution matching, we may need to use a different approach, like a taylored construction that can take advantage of batching techniques in order to avoid the penalty that comes from the inherently sequential method proposed by Sorting Hat.
