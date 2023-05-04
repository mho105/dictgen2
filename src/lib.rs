mod dictgen;

use pyo3::{prelude::*, types::PyDict};
use dictgen::{
    DictGen, 
    settings::Settings, generators::RandomGenerator};

#[pyfunction]
#[pyo3(signature=(
    max_height=3,
    max_depth=3
))]
fn generate(
    py: Python,
    max_height: u32,
    max_depth: u32
) -> PyResult<PyObject> {
    let settings = Settings { 
        max_height,
        max_depth,  
        key_generators: &vec![
            RandomGenerator::random_string],
        value_generators: &vec![
            RandomGenerator::random_int,
            RandomGenerator::random_string, 
            RandomGenerator::random_float,
            RandomGenerator::random_bool],
        nested_generators: &vec![
            RandomGenerator::random_array,
            RandomGenerator::random_dict],
    };
    let map = DictGen::generate(&settings).unwrap();

    let py_dict = PyDict::new(py);
    for (key, value) in map.iter() {
        py_dict.set_item(key, value)?;
    }

    Ok(py_dict.into())
}

#[pymodule]
fn dictgen2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    Ok(())
}