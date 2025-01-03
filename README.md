# Rust Data Race Example

This repository demonstrates a simple example of a data race in Rust. Data races occur when multiple threads or parts of code attempt to modify the same piece of memory concurrently without proper synchronization. This can lead to unpredictable behavior and program crashes.

The `bug.rs` file contains code that causes a data race. The `bugSolution.rs` file shows how to fix the data race using appropriate synchronization mechanisms (mutex).