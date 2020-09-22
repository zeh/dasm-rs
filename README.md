# dasm-rs

Welcome. This is **dasm-rs**, a [Rust language](https://www.rust-lang.org/) version of [dasm](https://github.com/dasm-assembler/dasm), the popular macro assembler for 8-bit microprocessors, originally written in C.

This is a personal project, meant as an exercise both in C reading and Rust writing.

This project was created by initially converting the original dasm c code to Rust using [C2Rust](https://c2rust.com/), and then manually rewriting it into idiomatic Rust. This rewriting is still a work-in-progress. Check the [dev](https://github.com/zeh/dasm-rs/tree/dev) branch for the latest commits.

The project already compiles and creates binary files that match the original dasm version. However, that was somewhat trivial. The goal, instead, is to have a final code in idiomatic Rust that uses its features - memory safety, stricter ownership, etc - to full advantage.

As such, this is somewhat of a flexible rewrite: the file structure has changed significantly from the original, and will continue to change. Variables and function names are changed for clarity and to better conform to Rust's style standards. And while the basic architecture of the application is somewhat similar, I'm loosely implementing refactors of my own that I believe will make the code easier to understand and follow.

Please notice that once deemed complete, this project will not be maintained. Take the efforts here with a grain of salt; once again, it's just an exercise.

I'm writing about the experience and should have a collection of notes published at the end.

Finally, please read [the original README](README_ORIGINAL.txt) for more information on dasm itself, or just head to the [official project's repository](https://github.com/dasm-assembler/dasm).
