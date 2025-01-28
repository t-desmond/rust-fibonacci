## Containerizing fibonacci sequence in rust

The image was build and pushed to ghcr and can be pulled using
```bash
docker pull ghcr.io/t-desmond/rust-fibonacci:latest
```
once pulled, a container can employed to run the program using
```bash
docker run -it --rm --name fib-rs ghcr.io/t-desmond/rust-fibonacci
```

For more info about docker (contaners, docker-compose, dockerfile...), checkout  https://docs.docker.com/

The project was first realized using an a recursive aproach which made the containers consume above 90% system resources for small values such as fib(40)..
This issue was solved by implementing an iterative approach which considerably reduced system resource consumption but a problem persists.. calculation for fib(186) and above results in a value that exceeds the maximum maximum value that can be represented by the u128 data type in Rust.
This causes the program to [panic](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#:~:text=The%20panic!,your%20code%20could%20recover%20from.).

This will probably be corrected in time to come using libraries
