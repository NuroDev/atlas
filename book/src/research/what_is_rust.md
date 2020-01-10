# 🦀 What Is Rust?
Rust is a new low-level programming language that is focused on safety, primarily safe concurrency.
The language is focused on being able to offer high performance but also offer safety in parallel with that performance.
This means Rust is perfect for applications that require either maxiumum performance, like creating games, or mission-critical systems that require maximum safety.

## Safety
One of Rust's big selling points that makes the language so much safer than other languages like C++ where you have to perform manual memory management, or C# where you rely on a garbage collector to manage your memory, Rust instead uses its ownership model to guarantee memory safety, along with thread safety for data parallelism work.

## Performance
Since Rust uses its ownership model for memory management, that means there is no garbage collector present. This dramatically improve performance as any application will not have to waste resources on checking if there is memory that is scheduled to be freed up.
