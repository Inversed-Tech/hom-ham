# Concrete experiments in rust

Here there are a couple of experiment using Concrete in rust. For thousands of bits we have that it takes tens of seconds to finish a basic building block, therefore a naive approach isn't practical.

Other approaches:

* [Lookup tables](https://docs.zama.ai/concrete/tutorials/table_lookups). It allows to optimize certain function by using lookup tables. Current constructions allows us to construct lookup tables for relatively small input size. This technique may be useful for low resolution filtering/matching, but not appropriate for high resolution unless hybrid approach is used together with the lookup table. 
* [Programmable bootstrapping](https://whitepaper.zama.ai/). Bootstrapping is expensive and it is not needed for computing the Hamming distance. Programmable bootstrapping allows us to control which function is going to be evaluated before the bootstrapping operation. Ideally we would like to avoid all bootstrapping, then we want to configure Concrete such that no bootstrapping is required. 
* Batching. TFHE has poor support for batching. This paper shows that is only has batching for the FHE implementation and not for the LHE (leveled scheme, the one we need).
* Polynomial encoding. Ideas proposed in this [paper](https://github.com/KULeuven-COSIC/SortingHat) achieve a very efficient implementation of the homomorphic Hamming distance (with the final comparison), but the process is inherently sequential, therefore is appropriate for small input vectors. 
