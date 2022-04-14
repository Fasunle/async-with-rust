# Async/await | Future in Rust

every async functions return a type Future. This by itself doe not do anything. It therefore requires an executor (runtime executor) to poll it to yield its result.

block_on is a function which blocks the executor runtime until the future resolves. block_on blocks the current thread until the future yields. Therefore, the code is executed sequentially.
