# Homomorphic Hamming Distance

This repository delves into various alternatives for homomorphically computing the Hamming distance. Our primary objective is to identify the most efficient scheme for handling input vectors with thousands of bits. Additionally, we aim to assess the computational costs of operations such as initial masking and final comparison within the context of an 80-bit security cryptosystem.

## Using Concrete compiler

The `concrete-python` folder contains Python programs based on the Concrete library. A compiler is responsible for generating the underlying circuits. During the initial exploration phase, we observed that the masking operation significantly impacts performance, roughly doubling the timings, which are on the order of tens of seconds.

To run the code, you must install the Concrete library, as described [here](https://docs.zama.ai/concrete/getting-started/installing). After installation, execute `python FILENAME`, where `FILENAME` corresponds to one of the files in this folder.

## Using Concrete in rust

We explored the implementation of the Hamming distance using concrete-rs. We tried to encode in different ways, using bit or integer encoding. We also explored lookup tables, which are only useful for small input size. 

Programmable bootstrapping is an interesting line of investigation. 

While we were exploring this path, we found out a construction called SortingHat, 

## SortingHat

This [paper](https://eprint.iacr.org/2022/757.pdf) shows how compute the Hamming distance and the final threshold comparison using Concrete.

## Scale Invariant NTRU-Based Scheme

The `si-ntru` folder houses the latest implementation, achieving performance of a few milliseconds. This implementation is based on the Scale Invariant NTRU-based [construction](https://eprint.iacr.org/2013/075.pdf). We introduced a simple trick to compute the Hamming distance without resorting to the XOR operation and, simultaneously, achieved a small optimization by avoiding one multiplication.

To run the code, you must install GMP and NTL libraries. For GMP, download it and follow the steps [here](https://gmplib.org/manual/Installing-GMP). For NTL, refer to this [link](https://libntl.org/doc/tour-unix.html). Then execute `make all` to compile and run it by calling the executable as follows: 

```
./test/ltvtest
```

## MPC Experiments

Folder `mpc-experiments` contains some experiments using [MP-SPDZ](https://mp-spdz.readthedocs.io/en/latest/index.html).



## Final Remarks

Some implementations can be adapted to include initial masking and final comparisons. For example, compilers for Fully Homomorphic Encryption (FHE) are powerful tools that allow the addition of more functionality to homomorphic systems. However, this generally comes at a high computational cost. To achieve optimal performance, tailoring the construction to our specific use case is essential.
