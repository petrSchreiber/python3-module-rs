# python3-module-rs
Example of Python 3 module written in Rust.

The code attempts to cover the same functionality as [Boost Python example](https://github.com/avast/boost-python-examples/tree/master/boost-python-examples) by Lukáš Kučera.

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

print(python3_module_rs.__doc__)        # Doc string of the module    

circle = python3_module_rs.Circle(5)    # Instantiating Rust class with parameter
print(circle.get_area())                

cylinder = python3_module_rs.Cylinder(radius=5, height=6)   # Instantiating Rust class with named parameters
print(cylinder.get_area())

print(cylinder.get_values_as_dict())    # Returning Dict from method
```
