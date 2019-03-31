#[macro_use] extern crate cpython;  // Needed for py_module_initializer! macro

// Module initialization, details explained at https://dgrunwald.github.io/rust-cpython/doc/cpython/macro.py_module_initializer.html
py_module_initializer!(python3_module_rs,                   // The module name as a Rust identifier
                       initpython3_module_rs,               // The same as first parameter, but with init prefix
                       PyInit_python3_module_rs, |py, m| {  // The same as first parameter, but with PyInit_ prefix
    m.add(py, "__doc__", "Python 3 module example")?;

    Ok(())
});
