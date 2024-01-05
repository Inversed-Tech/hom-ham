# Scale Invariant NTRU-Based Scheme

This folder the implementation based on the Scale Invariant NTRU-based [construction](https://eprint.iacr.org/2013/075.pdf). 

We introduced a simple trick to compute the Hamming distance without resorting to the XOR operation and, simultaneously, achieved a small optimization by avoiding one multiplication.

To run the code, installation of the GMP and NTL libraries is required. For GMP, download it and follow the steps [here](https://gmplib.org/manual/Installing-GMP). For NTL, refer to this [link](https://libntl.org/doc/tour-unix.html).

To compile: 

```
make all
```

```
./test/ltvtest
```
