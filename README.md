# python3-module-rs
Example of Python 3 module written in Rust.

The code attempts to mirror the [Boost Python example](https://github.com/avast/boost-python-examples/tree/master/boost-python-examples) by Lukáš Kučera.

_Currently work in progress_

## How to build
The code is designed to run with Rust from stable channel.

Before building:
- ensure you have Python 3.3 or newer installed on build machine
- ensure PATH to contain the path to `python.exe`

Build the library by `cargo build --release`.

The binary can be found in `target/release` afterwards:
- on Windows, change the binary extension from `.dll` to `.pyd`
- on Mac, change the binary extension from `.dylib` to `.so`

## How to try
Copy the binary with appropriate extension to the same directory as your script or along your Python binary.

Then, you can do:
```
import python3_module_rs

print(python3_module_rs.__doc__)
```
