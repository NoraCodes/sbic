# sbic

`sbic` is the [SBrain](https://github.com/silverwingedseraph/sbrain) Interpreter/Compiler. Currently it only interpretes SBrain programs but will eventually be extended to compile and optimize them.

A program and its input are specified in a toml file, like so:

```
# A simple program that emits its input as its output
source = ",[.>,]"

# Some input
input = [1, 2, 3, 4, 5, 4, 3, 2, 1]

# If you don't provide a maximum runtime, the program can run forever.
max_runtime = 128
``` 

To run, you can either `cargo run --release <config file>`, or `cargo build --release` and grab the binary from the `target` directory.