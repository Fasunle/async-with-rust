# Async/await | Future in Rust

every async functions return a type Future. This by itself doe not do anything. It therefore requires an executor (runtime executor) to poll it to yield its result.

block_on is a function which blocks the executor runtime until the future resolves. block_on blocks the current thread until the future yields. Therefore, the code is executed sequentially.

Instead of block_on, join! macro from future could be used. This allows futures to be pulled concurrently and the result is returned as a tuple

// `join!` is like `.await` but can wait for multiple futures concurrently.
// If we're temporarily blocked in the `learn_and_sing` future, the `dance`
// future will take over the current thread. If `dance` becomes blocked,
// `learn_and_sing` can take back over. If both futures are blocked, then
// `async_main` is blocked and will yield to the executor.

## Differences between await and join!

await works on one future and return a value while join! works on multiple futures
