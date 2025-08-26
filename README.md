# ABE Cubed

## General Info and Versions
The artifact was developed and tested on Ubuntu 22.04.5 LTS with `cargo` 1.85.1 for the Rust implementation.

## Artifact structure

### Code
This folder contains the Rust implementation of the scheme with `arkworks` and `criterion` benchmarking setup.
The `Makefile` provides an easy point of entry to build and run things.

#### Run the tests
*Warning*: running all tests may take a while (~20 minutes on my laptop)
```shell
$ make test
```

#### Run the benchmarks
*Warning*: running all benchmarks may take a while (~3 days on our server)
This produces `out.txt` files which can be converted to CSV with `report_to_csv.jl` in `eval`.
```shell
$ make bench
```

We employ 3 benchmarking strategies, namely 1a, 1b and 2. You can run them individuall using
the make targets `bench_01a`, `bench_01b` and `bench_02` respectively.

#### Run the example in `main.rs`
*Info*: when successfull, the example has **no** output/prints.
```shell
$ make run
```

### Eval
This folder contains our measured benchmark data and utility scripts to parse output, generate plots
and evaluate the schemes.

- `strat01a`: the data for benchmarking strategy 01a as presented in the paper
- `strat01b`: the data for benchmarking strategy 0b as presented in the paper
- `strat02`: the data for benchmarking strategy 02 as presented in the paper

- `report_to_csv.jl`: a Julia script to convert `out.txt` into a CSV file for further processing
- `plot_csv.jl`: a Julia script to re-create all plots under `eval`
- `*.ipynb`: Jupyter (Python) notebooks to perform further evaluations

### Scripts
Here are additional scripts used during/for the development of the implementation.

- `deduplication.py`: an experimental script to model the behaviour of tau/iota functions
- `plot_dedup_mappings.py`: a script to plot the behaviour of tau/iota for various policies
- `generate_test_cases.py`: a script to auto-generate test cases for the scheme implementation
- `input_gen.py`: an experiment script to explore strategies to generate the inputs for our benchmarks
