# Ballistic Deposition
This repo contains my current attempt at creating a ballistic deposition tool/program in Rust. I originally wrote a similar tool in Python as part the module 'Scientific Computing' at the University of Nottingham. However, a tool written in a compiled language such as Rust should prove to be considerably more time-efficient.

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

Otherwise known to me as 'sad Tetris'. See [this paper by Meakin, Ramanlal, Sander, and Ball](https://sci-hub.st/10.1103/PhysRevA.34.5091) for a comprehensive introduction on the topic. Another excellent read on the topic is Dr. Arne Schwettmann's [Master's Thesis on Ballistic Deposition](https://digital.library.unt.edu/ark:/67531/metadc4392/m2/1/high_res_d/problieu).

## How to use

Running the simulations is fairly straightforward. Assuming [Rust is installed on the system](https://www.rust-lang.org/tools/install), first set the simulation parameters in `config\config_sim.ini` such as in the following example:

![Example simulation parameters](/images/example_sim_config.png "Example simulation parameters")

The parameters are parsed by the Rust code. Parameters within `simulation_params` are parsed as vectors; so if you would like to run multiple simulations of different parameters, supply a comma-separated list as shown above. The parameters are as follows:

- `substrate_lengths`: The total number of sites on the 1D substrate on which blocks can fall. Accepted values are (at time of writing) powers of two from 8 to 16384 (2^14) inclusive.
- `k_neighbours`: Defines the amount of correlations between distant columns. For example, k = 0 would result in no sticking of particles to adjacent columns, k = 1, would allow particles to stick to adjacent columns (nearest neighbours), and so on. Any integer values are accepted.
- `seeds`: Defines the total number of random number seeds over which to average our ensemble. A greater number of random number seeds reduces the overall noise of the plotted results; see below. 

<img src="/images/seeds_comparison.png" alt="Comparison between 1 and 1000 seeds" width="400"/>
  

## Scope/Stretch Goals
The current goal is to get an end-to-end program running for ballistic deposition, to saturation, for substrate lengths L = 8 to L = 16384. This includes relevant analysis and Python code to generate graphs and conclusions.

If I have time, there are stretch goals to add functionality for firing blocks at fixed angles, or adding impurity blocks (or some kind of simulation of arbitrary shape and size blocks).

Create a simulation for Tetriminos?

![Picture of tetrominos](/images/tetrominos.webp "How hard could it be?")


## Acknowledgements

- The Ballistic Deposition logic used within this code has been adapted, with permission, from code written by Dr. Arne Schwettmann as part of his [Master's Thesis on Ballistic Deposition](https://digital.library.unt.edu/ark:/67531/metadc4392/m2/1/high_res_d/problieu.pdf). I would recommend
- This code includes a port into Rust of the `ran2()` random number generator, which can be found in the book 
[Numerical Recipes in C](http://s3.amazonaws.com/nrbook.com/book_C210.html), page 282.
- The community at [URLO](users.rust-lang.org), who are always (very patiently!) happy to help out new learners of the language such as myself.
- The [Rust Book](https://doc.rust-lang.org/stable/book/), which may be the single best beginner's guide to a programming language that I have ever seen.

## License

The project is hosted on GitHub under an MIT License. Please see the license file for more information.

## TODO
- Clean up `config.ini` parsing until I am happy with its behaviour
- Get a working simulation running and tested for L = something small
- Successfully export simulation data to .csv files, where filenames are generated from the input params
- Write some Python code (in this repo) to analyse the generated data, likely Jupyter Notebooks
- Write a more complete handling of errors, such as if an invalid substrate length is selected
- Investigate/write a Python wrapper to run simulations in Rust and analyse the data(?)
- Write a more complete `Theory` section rather than just having a link to a paper
