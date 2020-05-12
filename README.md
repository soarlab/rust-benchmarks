# Rust Software Verification Benchmarks

This repository provides a collection of Rust verification benchmarks and the
accompanying verifier crate. The crate provides a basic verification interface,
such as for introducing assertions, assumptions, and nondeterministic values.
The interface was inspired by SVCOMP.  We hope it enables for various verifiers
to be easily applied on the benchmarks without having to make any updates to
the benchmarks themselves.

This is work in progress and feedback and contributions would be greatly
appreciated!


This is how you run MIRAI:
```
mirai add_fail.rs --extern verifier=/Users/zrakamaric/projects/rust-benchmarks/verifier/target/debug/deps/libverifier-a0e8df43af99bd6d.rlib --extern mirai_annotations=/Users/zrakamaric/projects/rust-benchmarks/verifier/target/debug/deps/libmirai_annotations-02aa2d91ba500cf6.rlib
```

