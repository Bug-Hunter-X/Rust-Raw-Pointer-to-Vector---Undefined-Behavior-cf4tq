# Rust Raw Pointer to Vector - Undefined Behavior

This repository demonstrates a common error in Rust involving the use of raw pointers with vectors.  Modifying a vector's contents through a raw pointer after the vector's capacity might change can lead to undefined behavior and crashes.

The `bug.rs` file contains the erroneous code, while `bugSolution.rs` shows a corrected and safer approach.

**Key takeaway:** Always avoid modifying vectors through raw pointers after potentially changing the vector's capacity.  Use safe Rust abstractions whenever possible.