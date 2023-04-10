
First, compile Rust code to a dylib dynamic library.

```shell
cargo build --target=aarch64-apple-darwin --release	
```

Then, write a .h file to declare the functions in the dynamic library.

After that, use gcc to build c and lib to executable file.

```shell
gcc -o main main.c -L./target/aarch64-apple-darwin/release -ldemo
```

Done, you have obtained an executable file.
