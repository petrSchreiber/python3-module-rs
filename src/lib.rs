#[macro_use] extern crate cpython;  // Needed for py_module_initializer! macro

// This is cpython magic to generate both Python class and Rust class
py_class!(class Circle |py| {
    data radius: f32;
    def __new__(_cls, radius: f32) -> cpython::PyResult<Circle> {
        Circle::create_instance(py, radius)
    }

    def get_area(&self) -> cpython::PyResult<f32> {        
        Ok(std::f32::consts::PI * self.radius(py).powi(2))
    }
});

py_class!(class Cylinder |py| {
    data radius: f32;
    data height: f32;

    def __new__(_cls, radius: f32, height: f32) -> cpython::PyResult<Cylinder> {
        Cylinder::create_instance(py, radius, height)
    }

    def get_area(&self) -> cpython::PyResult<f32> {
        Ok(2.0 * std::f32::consts::PI * self.radius(py) * (self.radius(py) + self.height(py)))
    }

    def get_values_as_dict(&self) -> cpython::PyResult<cpython::PyDict> {
        let values = cpython::PyDict::new(py);
        values.set_item(py, "radius", self.radius(py))?;
        values.set_item(py, "height", self.height(py))?;
        Ok(values)
    }
});

// Module initialization, details explained at https://dgrunwald.github.io/rust-cpython/doc/cpython/macro.py_module_initializer.html
py_module_initializer!(python3_module_rs,                   // The module name as a Rust identifier
                       initpython3_module_rs,               // The same as first parameter, but with init prefix
                       PyInit_python3_module_rs, |py, m| {  // The same as first parameter, but with PyInit_ prefix
    m.add(py, "__doc__", "Python 3 module example")?;
    m.add_class::<Circle>(py)?;
    m.add_class::<Cylinder>(py)?;    

    Ok(())
});
