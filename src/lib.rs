use pyo3::prelude::*;
//use pyo3::wrap_pyfunction;

pub mod py_enum;

use py_enum::value::Value;
use py_enum::pyenum::PyEnum;


#[pymodule]
fn string_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Value>()?;
    m.add_class::<PyEnum>()?;

    Ok(())
}
