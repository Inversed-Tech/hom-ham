# Homomorphic Hamming Distance

This repository delves into various alternatives for homomorphically computing the Hamming distance. Our primary objective is to identify the most efficient scheme for handling input vectors with thousands of bits. Additionally, we aim to assess the computational costs of operations such as initial masking and final comparison within the context of an 80-bit security cryptosystem.

## Concrete Python Compiler

The `concrete-python` folder contains Python programs based on the Concrete library. A compiler is responsible for generating the underlying circuits. During the initial exploration phase, we observed that the masking operation significantly impacts performance, roughly doubling the timings, which are on the order of tens of seconds.

To run the code, you must install the Concrete library, as described [here](https://docs.zama.ai/concrete/getting-started/installing). After installation, execute `python FILENAME`, where `FILENAME` corresponds to one of the files in this folder.

## Scale Invariant NTRU-Based Scheme

The `si-ntru` folder houses the latest implementation, achieving performance in the range of hundreds of milliseconds. This implementation is based on the Scale Invariant NTRU-based [construction](https://eprint.iacr.org/2013/075.pdf). We introduced a simple trick to compute the Hamming distance without resorting to the XOR operation and, simultaneously, achieved a small optimization by avoiding one multiplication.

To run the code, installation of the GMP and NTL libraries is required. For GMP, download it and follow the steps [here](https://gmplib.org/manual/Installing-GMP). For NTL, refer to this [link](https://libntl.org/doc/tour-unix.html).

## Final Remarks

Some implementations can be adapted to include initial masking and final comparisons. For example, compilers for Fully Homomorphic Encryption (FHE) are powerful tools that allow the addition of more functionality to homomorphic systems. However, this generally comes at a high computational cost. To achieve optimal performance, tailoring the construction to our specific use case is essential.
