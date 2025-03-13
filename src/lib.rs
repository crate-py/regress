use pyo3::{create_exception, prelude::*, types::PySlice};
use regress::{Match, Range, Regex};

create_exception!(regress, RegressError, pyo3::exceptions::PyException);

#[repr(transparent)]
#[pyclass(name = "Regex", module = "regress", frozen)]
struct RegexPy {
    inner: Regex,
}

#[pymethods]
impl RegexPy {
    #[new]
    #[pyo3(signature = (value, flags=None))]
    fn init(value: &str, flags: Option<&str>) -> PyResult<Self> {
        match flags {
            Some(f) => match Regex::with_flags(value, f) {
                Ok(inner) => Ok(RegexPy { inner }),
                Err(e) => Err(RegressError::new_err(e.to_string())),
            },
            None => match Regex::new(value) {
                Ok(inner) => Ok(RegexPy { inner }),
                Err(e) => Err(RegressError::new_err(e.to_string())),
            },
        }
    }

    fn find(&self, value: &str) -> Option<MatchPy> {
        self.inner.find(value).map(|inner| MatchPy { inner })
    }

    fn find_iter(&self, value: &str) -> Vec<MatchPy> {
        self.inner
            .find_iter(value)
            .map(|m| MatchPy { inner: m })
            .collect()
    }
}

#[repr(transparent)]
#[pyclass(name = "Match", module = "regress", frozen)]
struct MatchPy {
    inner: Match,
}

#[pymethods]
impl MatchPy {
    fn group(&self, idx: usize, py: Python) -> PyResult<Option<PyObject>> {
        match self.inner.group(idx) {
            Some(m) => Ok(Some(to_slice(py, m)?)),
            None => Ok(None),
        }
    }

    fn named_group(&self, name: &str, py: Python) -> PyResult<Option<PyObject>> {
        match self.inner.named_group(name) {
            Some(m) => Ok(Some(to_slice(py, m)?)),
            None => Ok(None),
        }
    }

    fn range(&self, py: Python) -> PyResult<PyObject> {
        to_slice(py, self.inner.range())
    }
}

#[pymodule]
#[pyo3(name = "regress")]
fn regress_py(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MatchPy>()?;
    m.add_class::<RegexPy>()?;
    m.add("RegressError", py.get_type::<RegressError>())?;
    Ok(())
}

fn to_slice(py: Python, range: Range) -> PyResult<PyObject> {
    Ok(PySlice::new(py, range.start.try_into()?, range.end.try_into()?, 1).into())
}
