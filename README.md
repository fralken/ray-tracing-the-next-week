## Ray tracing the next week, in Rust

This is an implementation in Rust of [Peter Shirley's "Ray Tracing The Next Week"](https://github.com/petershirley/raytracingthenextweek) book.

Every tagged commit is the code that generates a specific image. In this way it's easy to follow the progress in the book.

Instead of implementing my own `vec3`, I preferred using `Vector3` from [`nalgebra`](https://crates.io/crates/nalgebra) crate.
For random numbers I used [`rand`](https://crates.io/crates/rand). For image loading I used [`image`](https://crates.io/crates/image)

Hence dependencies are:
- [`nalgebra`](https://www.nalgebra.org)
- [`rand`](https://rust-random.github.io/book/)
- [`image`](https://github.com/image-rs/image)

![Ray Tracing](image.jpg)
