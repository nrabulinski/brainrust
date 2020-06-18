⚠️ THIS ONLY WORKS WITH A NIGHTLY COMPILER AND IN RELEASE MODE BECAUSE I COULDN'T BOTHER WRAPPING EVERYTHING IN `Wrapping` SO THIS INTERPRETER RELIES HEAVILY ON THE FACT THAT IN RELEASE RUST DOES NOT CHECK FOR OVERFLOWS ⚠️
# Brainrust
This is an interpreter for brainf\*\*k I've written in Rust in like a single afternoon.

## Project structure
The `main.rs` is very messy but the project *so far* contains two main modules: `naive.rs` and `optimized.rs` where the former contains the most basic, slow but 100% working implementation and the former has parsing and a few optimizations which can lead up to even 5x performance boost on programs which do not take any user input.

## Plans for this project
I have already achieved what I wanted to achieve but I feel like I can do better. If I feel like it I might in the future implement AOT/JIT and even const evaluation.
