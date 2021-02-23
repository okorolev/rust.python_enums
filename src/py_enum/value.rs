use pyo3::prelude::*;
use pyo3::class::PyObjectProtocol;
//use pyo3::types::PyType;
use pyo3::class::basic::CompareOp;
use pyo3::AsPyPointer;

#[pyclass]
pub struct Value {
    pub classname: String,
    pub name: String,
    pub value: PyObject,
}

#[pymethods]
impl Value {
    #[new]
    fn new(obj: &PyRawObject, name: String, value: PyObject, classname: String) -> () {
        obj.init(Value { name, value, classname });
    }

    #[getter(value)]
    fn get_value(&self) -> PyResult<&PyObject> {
        Ok(&self.value)
    }

    #[getter(name)]
    fn get_name(&self) -> PyResult<&String> {
        Ok(&self.name)
    }
}

#[pyproto]
impl PyObjectProtocol for Value {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}.{}", &self.classname, &self.name))
    }

    fn __hash__(&self) -> PyResult<isize> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let py_object_str = self.name.clone().into_object(py).as_ptr();
        let obj_hash = unsafe { pyo3::ffi::PyObject_Hash(py_object_str) };

        match obj_hash {
            -1 => Err(PyErr::fetch(py)),
            _ => Ok(obj_hash)
        }
    }

    fn __richcmp__(&self, other: PyObject, op: CompareOp) -> PyResult<bool> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let maybe_value = other.getattr::<&str>(py, "value");
        let other_value = match maybe_value {
            Ok(value) => value,
            Err(_) => return Ok(false), // TODO: raise exc: unsupported operation
        };

        let py_compare = |curr_op: CompareOp|
            unsafe {
                pyo3::ffi::PyObject_RichCompareBool(
                    self.value.as_ptr(),
                    other_value.as_ptr(),
                    curr_op as i32,
                )
            };

        let result = match op {
            cur_op @ CompareOp::Eq => py_compare(cur_op), // ==
            cur_op @ CompareOp::Lt => py_compare(cur_op), // <
            cur_op @ CompareOp::Le => py_compare(cur_op), // <=
            cur_op @ CompareOp::Ne => py_compare(cur_op), // !=
            cur_op @ CompareOp::Gt => py_compare(cur_op), // >
            cur_op @ CompareOp::Ge => py_compare(cur_op), // >=
            unexpected @ _ => panic!(format!("Unsupported op: {:?}", unexpected))
        };

        match result {
            0 => Ok(false),
            1 => Ok(true),
            _ => panic!("Unexpected error!"),
        }
    }
}


// build struct based on typehints (macros)
