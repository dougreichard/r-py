use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PyTuple;
use pyo3::{wrap_pyfunction, wrap_pymodule};

#[pyclass]
struct Doug {
    inner: i32,
}

#[pymethods]
impl Doug {
    #[new]
    fn new(value: i32) -> Self {
        Doug { inner: value }
    }
}

#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

#[pymodule]
fn reichard(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Doug>()?;
    m.add_function(wrap_pyfunction!(double, m)?)?;

    Ok(())
}

fn main() -> PyResult<()> {
    let source = std::fs::read_to_string("assets/script.py")?;
    Python::with_gil(|py| {
        //let reichard = py.import("reichard")?;
        let sys = PyModule::import(py, "sys")?;
        let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
        let reichard_ref = wrap_pymodule!(reichard)(py);
        sys_modules.set_item("reichard", reichard_ref)?;

        let code: &PyModule = PyModule::from_code(
            py,
            &source,
            "script.py",
            "script"
        )?;
        let on_init: Py<PyAny> = code.getattr("on_init")?.into();
        let on_tick: Py<PyAny> = code.getattr("on_tick")?.into();

        // call object without any arguments
        on_init.call0(py)?;
        on_tick.call0(py)?;

        Ok(())
    })
}
