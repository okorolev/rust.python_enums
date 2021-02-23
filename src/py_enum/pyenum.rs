use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::class::{
    PyObjectProtocol,
    PyIterProtocol,
    PySequenceProtocol,
    PyMappingProtocol,
};
use pyo3::AsPyPointer;
use pyo3::exceptions::{AttributeError, ValueError, KeyError};

use crate::py_enum::value::Value;

#[pyclass]
pub struct PyEnumIterator {
    iter: Box<Iterator<Item=PyObject> + Send>,
}

#[pyclass(subclass)]
pub struct PyEnum {
    classname: String,
    values_map: HashMap<String, Py<Value>>,
}

#[pymethods]
impl PyEnum {
    #[new]
    fn __new__(obj: &PyRawObject, py: Python, classname: String, values: Py<PyDict>) -> () {
        let members = collect_members(
            py,
            &classname,
            values.as_ref(py),
        );

        obj.init(PyEnum {
            classname,
            values_map: members,
        });
    }

    #[call]
    fn __call__(&self, value: PyObject) -> PyResult<Py<Value>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        // FIXME: broken -- NewENum(NewENumClone.black)
        let is_value = py.is_instance::<Value, PyObject>(&value)?;
        if is_value {
            return Ok(unsafe { Py::from_borrowed_ptr(value.as_ptr()) });
        }

        let value = value.extract::<i32>(py)?;
        for (_, enum_value) in &self.values_map {
            let exist_value = enum_value.as_ref(py).value.extract::<i32>(py)?;
            if exist_value == value {
                return Ok(enum_value.clone_ref(py));
            }
        }

        Err(ValueError::py_err(format!("{} is not a valid {}", &value, &self.classname)))
    }

    fn get_member(&self, name: String) -> PyResult<Py<Value>> {
        match self.values_map.get(&name) {
            Some(x) => Ok(unsafe { Py::from_borrowed_ptr(x.as_ptr()) }),
            None => Err(AttributeError::py_err(format!("{}", &name))),
        }
    }
}

#[pyproto]
impl PyMappingProtocol for PyEnum {
    fn __getitem__(&self, key: String) -> PyResult<Py<Value>> {
        match self.values_map.get(&key) {
            Some(x) => Ok(unsafe { Py::from_borrowed_ptr(x.as_ptr()) }),
            None => Err(KeyError::py_err(format!("{}", &key))),
        }
    }
}

#[pyproto]
impl PySequenceProtocol for PyEnum {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.values_map.len())
    }

    fn __contains__(&self, item: PyObject) -> PyResult<bool> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let is_value_cls = py.is_instance::<Value, PyObject>(&item)?;

        // TODO: miss check: NewENum is not NewENumClone.black
        if is_value_cls {
//            let item_name = unsafe { py.from_borrowed_ptr::<Value>(item.as_ptr()) };
            let item_name = item
                .getattr::<&str>(py, "name")?
                .extract::<String>(py)?;

            return match self.values_map.get(&item_name) {
                Some(_) => Ok(true),
                None => Ok(false),
            };
        }

        Ok(false)
    }
}

#[pyproto]
impl PyIterProtocol for PyEnumIterator {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<PyEnumIterator>> {
        Ok(slf.into())
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<PyObject>> {
        Ok(slf.iter.next())
    }
}

#[pyproto]
impl PyIterProtocol for PyEnum {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<PyEnumIterator>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let iter_values: Vec<PyObject> = slf.values_map
            .iter()
            .map(|(_, value)| value.into())
            .collect();

        let iterator = PyEnumIterator { iter: Box::new(iter_values.into_iter()) };

        Py::new(py, iterator)
    }
}


//
//#[pyproto]
//impl PyObjectProtocol for PyEnum {
//    fn __getattr__(&self, name: String) -> PyResult<Py<PyObject>> {
//        // TODO: use clone_ref
//        // TODO: raise AttributeError if not exist
//        let gil = Python::acquire_gil();
//        let py = gil.python();
//
//        match self.values.get(&name) {
//            Some(x) => Ok(unsafe { Py::from_borrowed_ptr(x.as_ptr()) }),
//            None => Ok(
//                unsafe {
//                    let a = self.into_object(py);
//                    let s = PyString::new(py, &name);
//
//                    Py::from_owned_ptr(
//                        pyo3::ffi::PyObject_GetAttr(a.as_ptr(), s.as_ptr())
//                    )
//                }
//            ),
//        }
////        need tp_getattr
////        Ok(unsafe { Py::from_borrowed_ptr(self.values.get(&name).unwrap().as_ptr()) })
//    }
//}

fn collect_members(
    py: Python,
    classname: &String,
    members: PyRef<PyDict>,
) -> HashMap<String, Py<Value>> {
    // TODO: enumerate for auto numbers
    let mut values_map = HashMap::new();

    for (key, value) in members.iter() {
//        let key_str = key.to_string();
        let is_private_method = key.to_string().starts_with("__") || key.to_string().starts_with("_");

        if !is_private_method {
            let value_cls = Value {
                classname: classname.to_string(),
                name: key.to_string(),
                value: value.into(),
            };
            let obj = Py::new(py, value_cls).expect("Allocation error");
            values_map.insert(key.to_string(), obj);
        }
    }

    values_map
}

// https://docs.python.org/3/library/enum.html#using-a-custom-new
// https://docs.python.org/3/c-api/typeobj.html#c.PyTypeObject.tp_getattr
// https://docs.python.org/3/c-api/typeobj.html?highlight=tp_getattro#c.PyTypeObject.tp_getattro
// https://docs.python.org/3/c-api/object.html#c.PyObject_GetAttr
