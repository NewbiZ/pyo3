#![cfg(all(feature = "rust_decimal", not(any(Py_LIMITED_API))))]

use crate::{
    types::*, FromPyObject, IntoPy, PyAny, PyObject, PyResult,
    Python, ToPyObject,
};

use rust_decimal::Decimal;

#[cfg_attr(docsrs, doc(cfg(feature = "rust_decimal")))]
impl ToPyObject for Decimal {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let decimal_cls = py.import("decimal").unwrap().getattr("Decimal").unwrap();
        // CPython does not expose Decimal in its API, so we have to rely on the Python class.
        // There are basically three ways to build a Decimal: from a string, from a tuple of
        // digits, and from a float.
        // The latter is extremely lossy and would defeat the purpose, thus we prefer the string
        // alternative here.
        let args = (self.to_string() ,);
        let result = decimal_cls.call(args, None).unwrap();
        result.to_object(py)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "rust_decimal")))]
impl IntoPy<PyObject> for Decimal {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "rust_decimal")))]
impl<'source> FromPyObject<'source> for Decimal {
    fn extract(ob: &'source PyAny) -> PyResult<Decimal> {
        // As convenience, anything that can be represented as a string will be submitted
        // to rust_decimal
        let result_str = ob.call_method("__str__", (), None)?;
        Ok(Decimal::from_str_exact(result_str.to_string().as_str()).unwrap())
    }
}
