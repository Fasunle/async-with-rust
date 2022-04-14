# Asynchronous programming in rust

Asynchronous programming (concurrency and parallelism) in rust is either handled with async/await or thread

One is not better than the other. It all depends on use case. However, more resources are used when threads are used "Almdahl's rule". One the contrary, async/await uses less resources and can handle tasks in a concurrent manner.

Rust core does not have async runtime but it provides future module which provides feature trait and other helper functions. Thus, every asynchronous application will depend on a third party async runtime. The common examples are Tokio (most porpular and full blown!), future (which implement executor but not reactor, and is therefore referred to as half-runtime), async-block etc

Tokio crate implements both single-threaded runtime and multi-threaded runtime. Single thread supports concurrency and even if the operating system has more capacity, it will not be useful since all the application in run on a single thread! so a lot of work will be available to the single thread to do; other available threads will not be able to help.

The Multi-threaded runtime allows for more than one thread executing operations and are coordinated in a manner that their is a main thread which will spawn the other threads and when they are done, they could be yeilded to the main thread.

NOTE: without the executor, async code does not do anything. The work of the executor is to constantly poll the async function for data.

Using futures::executor::block function, this allows to block the executor on a future. This resumes the execution of other lines when the future yields.
