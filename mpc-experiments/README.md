# MPC Experiments

Let `[MP-SPDZ-INSTALL-PATH]` be the installation path of MP-SPDZ library. Follow the instructions shown [here](https://mp-spdz.readthedocs.io/en/latest/readme.html#tl-dr-binary-distribution-on-linux-or-source-distribution-on-macos) to install it. 

To execute any experiment it is necessary to have input data for each party in folder `Player_Data`. Thus each of the corresponding subsections will describe how to generate it randomly. 

We are going to show how execute all experiments using protocol `hemi`. You can replace it by other protocols implemented in MP-SPDZ, but some protocols will not run properly without certain configurations, like for example the SSL setup, which is described in [here](https://mp-spdz.readthedocs.io/en/latest/readme.html#running-computation). We executed the following set of experiments: `semi2k`, `semi`, `hemi`, `temi`, `soho`. 

## Pure Hamming

Since we are using vectors of size 1000, we need 1000 bits for each party. 

### Input

If the input is given by 1000 thousand bits for both parties, then you can run the following commands to generate the input randomly: 

```
for i in {1..1000}; do echo $((RANDOM % 2)); done > Player-Data/Input-P0-0
for i in {1..1000}; do echo $((RANDOM % 2)); done > Player-Data/Input-P1-0

```

## Executing the experiment:

This experiment is implemented in file `hamming.mpc` and you can execute the exeperiment using protocol `hemi` running the following command: 

```
[MP-SPDZ-INSTALL-PATH]/Scripts/compile-run.py -E hemi hamming
```


## Masked Hamming

Each participant has 2 input vectors, therefore 2000 bits in total per participant. 

### Input

You can run the following commands to generate the input randomly: 

```
for i in {1..2000}; do echo $((RANDOM % 2)); done > Player-Data/Input-P0-0
for i in {1..2000}; do echo $((RANDOM % 2)); done > Player-Data/Input-P1-0

```

## Executing the experiment:

This experiment is implemented in file `masked-hamming.mpc` and you can execute exeperiment using protocol `hemi` running the following command: 

```
[MP-SPDZ-INSTALL-PATH]/Scripts/compile-run.py -E hemi masked-hamming
```

## Integer encoding

Similarly, it is possible to execute experiments with the integer encoding. See files `hamming-int.mpc` and `masked-hamming-int.mpc`
