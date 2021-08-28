- [Ballistic Deposition](#ballistic-deposition)
  - [Theory of Ballistic Deposition](#theory-of-ballistic-deposition)
  - [Scope/Stretch Goals](#scopestretch-goals)
  - [License](#license)
  - [TODO](#todo)

# Ballistic Deposition
This repo contains my current attempt at creating a ballistic deposition tool/program in Rust.

## Theory of Ballistic Deposition

Otherwise known to me as 'sad Tetris'. See [this paper by Meakin, Ramanlal, Sander, and Ball](https://sci-hub.st/10.1103/PhysRevA.34.5091) for a comprehensive introduction on the topic.

## Scope/Stretch Goals
The current goal is to get an end-to-end program running for ballistic deposition, to saturation, for substrate lengths L = 8 to L = 4096. This includes relevant analysis and Python code to generate graphs and conclusions.

I have stretch goals to add functionality for firing blocks at fixed angles, or adding impurity blocks (or some kind of simulation of arbitrary shape and size blocks).

Create a simulation for Tetriminos?

![picture of tetrominos](/images/tetrominos.webp "How hard could it be?")

## License

The project is hosted on GitHub under an MIT License. Please see the license file for more information.

## TODO
- Clean up `config.ini` parsing until I am happy with it
- Get a working simulation running and tested for L = something small
- Successfully export simulation data to .csv files, where filenames are generated from the input params
- Write some Python code (in this repo) to analyse the generated data, likely Jupyter Notebooks
- Investigate/write a Python wrapper to run simulations in Rust and analyse the data(?)
- Write a more complete `Theory` section rather than just having a link to a paper
