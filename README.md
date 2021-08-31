# Ballistic Deposition
This repo contains my current attempt at creating a ballistic deposition tool/program in Rust. I originally wrote a similar tool in Python during a module entitled 'Scientific Computing' as part of my ongoing Physics degree at the University of Nottingham. However, it has been my summer project to learn a compiled language, and Rust should prove to be considerably faster at such simulations.

## Table of Contents
- [Ballistic Deposition](#ballistic-deposition)
  - [Table of Contents](#table-of-contents)
  - [Theory of Ballistic Deposition](#theory-of-ballistic-deposition)
  - [How to use](#how-to-use)
  - [Scope/Stretch Goals](#scopestretch-goals)
  - [Acknowledgements](#acknowledgements)
  - [License](#license)
  - [TODO](#todo)

## Theory of Ballistic Deposition

See [this paper by Meakin, Ramanlal, Sander, and Ball](https://sci-hub.st/10.1103/PhysRevA.34.5091) for a comprehensive introduction on the topic. Another excellent read on the topic is Dr. Arne Schwettmann's [Master's Thesis on Ballistic Deposition](https://digital.library.unt.edu/ark:/67531/metadc4392/m2/1/high_res_d/problieu). Arne's paper has also served as the foundation of the logic used within the simulations in this repo -- see [Acknowledgements](#acknowledgements).

## How to Use
### Parameters

Running a simulation is a fairly straightforward process. Assuming [Rust is installed on the system](https://www.rust-lang.org/tools/install) and code successfully compiles, first set the simulation parameters in `config\config_sim.ini` such as in the following example:

![Example simulation parameters](/images/example_sim_config.png "Example simulation parameters")

The parameters are parsed by the Rust code. Parameters within `simulation_params` are parsed as vectors; so if you would like to run multiple simulations of different parameters, supply a comma-separated list as shown above. The parameters are as follows:

- `simulation_params`
    - `substrate_lengths`: The total number of sites on the 1D substrate on which blocks can fall. Accepted values are (at time of writing) powers of two from 8 to 16384 (2^14) inclusive.
    - `k_neighbours`: Defines the amount of correlations between distant columns. For example, k = 0 would result in no sticking of particles to adjacent columns, k = 1, would allow particles to stick to adjacent columns (nearest neighbours), and so on. Any integer values are accepted.
    - `seeds`: Defines the total number of random number seeds over which to average our ensemble. Any integer values are accept.ed A greater number of random number seeds reduces the overall noise of the plotted results; see below. 

<img src="/images/seeds_comparison.png" alt="Comparison between 1 and 1000 seeds" width="720"/>

- `options`
    - `periodic_bc`: Set periodic boundary conditions on or off. Periodic boundary conditions are cylindrical; for example, for a substrate length L = 128, a block at site 0 (the far left of the substrate) could stick to a column at site 127 (the far right). `true` and `false` are accepted.
    - `init_seed`: The initial random number seed fed to the `ran2` random number generator. The choice of number here is largely arbitary and shouldn't affect results of simulations if sufficient numbers of seeds are selected. Any integer values are accepted.

### Running Simulations

Like running any Rust binary, to run the simulations navigate a command prompt to the repository's root folder and run the following:

```
cargo run --release
```

N.B. The `--release` parameter is optional but recommended. It removes some debug functionality but is _significantly_ faster than running in debug mode. See [this page](https://nnethercote.github.io/perf-book/build-configuration.html) for more information.

The results of any simulation are automatically saved to `/data/`, where the .csv filename is auto-generated based on the parameters for that simulation. Be careful to note that any old .csv files will be automatically overwritten.

## Simulation Speed 

The main reason I wrote this program! My old Python script/tool was significantly slower than my Rust code. This is due to a combination of the old code being poorly optimised, the sizeable difference in speed between Python and Rust; and the ease by which Rust makes [parallelisation](https://docs.rs/rayon/1.3.0/rayon/) possible.

Running on my HP EliteBook 840 G1 (Intel i5-4210U 2 core/4 threads), the Python code takes around 52 seconds to complete:

<img src="/images/speed_python_l32.png" alt="Comparison between 1 and 1000 seeds" width="720"/>

Whereas, on the same computer the Rust code only takes 6 seconds--a speedup of roughly a factor of 8x or 9x.

<img src="/images/speed_rust_l32.png" alt="Comparison between 1 and 1000 seeds" width="720"/>

The difference in speedup would be even more stark on a machine with a large number of distributed cores, as the Rust code will parallelise its simulation across the ensemble of random number seeds.

## Scope/Stretch Goals
The current goal is to get an end-to-end program running for ballistic deposition, to saturation, for substrate lengths L = 8 to L = 16384. This includes relevant analysis and Python code to generate graphs and conclusions.

If I have time, there are stretch goals to add functionality for firing blocks at fixed angles, or adding impurity blocks (or some kind of simulation of arbitrary shape and size blocks).

Create a simulation for Tetriminos?

![Picture of tetrominos](/images/tetrominos.webp "How hard could it be?")


## Acknowledgements

- The Ballistic Deposition logic used within this code has been adapted, with permission, from code written by Dr. Arne Schwettmann as part of his [Master's Thesis on Ballistic Deposition](https://digital.library.unt.edu/ark:/67531/metadc4392/m2/1/high_res_d/problieu.pdf). The simulation logic within my `sim` module almost entirely a Rust port (with minor modifications) of his C++ code in Appendix A of the aforementioned paper. Thank you to him.
- This code includes a port into Rust of the `ran2()` random number generator, which can be found in the book 
[Numerical Recipes in C](http://s3.amazonaws.com/nrbook.com/book_C210.html), page 282.
- The community at [URLO](users.rust-lang.org), who are always (very patiently!) happy to help out new learners of the language such as myself.
- The [Rust Book](https://doc.rust-lang.org/stable/book/), which may be the single best beginner's guide to a programming language that I have ever seen.

## License

The project is hosted on GitHub under an MIT License. Please see the license file for more information.

## TODO
- ~~Clean up `config.ini` parsing until I am happy with its behaviour~~
    - Done! Although how the Jupyter notebooks source their config may be something I want to take a look at.
- ~~Get a working simulation running and tested for L = something small~~
    - Done! Tested up to L = 8096
- ~~Successfully export simulation data to .csv files, where filenames are generated from the input params~~
    - Done!
- Write some Python code (in this repo) to analyse the generated data, likely Jupyter Notebooks
    - Ongoing.
- Write a more complete handling of errors, such as if an invalid substrate length is selected
    - Partially done. The code will now `panic!` if a substrate length provided isn't a power of 2 from 8 to 16384 inclusive. I've not decided yet whether to provision time for making the error handling closer to something more 'production-ready'
- Investigate/write a Python wrapper to run simulations in Rust and analyse the data(?)
    - Possible. Though not necessary. May just be more convenient to leave the Rust program as command-line-only.
- Include a complete dataset of output .csv files 
- Write a more complete `Theory` section rather than just having a link to a paper