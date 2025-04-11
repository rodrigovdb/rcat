# Running CAT with Rust

[Follow this link](DOCKER.md) to start building the docker container.

tl;dr
```zsh
$ docker-compose run --build --rm app bash
```

Once inside the container,

```zsh
$ rcat <file1> [<file2> ... <filen>]
```

Running tests
```zsh
$ cargo test -- --show-output
```

# To revisit and learn more

- [Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Crates, Modules and Packages](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Generic Types](https://doc.rust-lang.org/book/ch10-00-generics.html)
