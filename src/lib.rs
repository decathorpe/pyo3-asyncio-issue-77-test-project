use std::time::Duration;

use pyo3::prelude::*;

#[pyfunction]
fn sleep(py: Python, secs: f64) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        tokio::time::sleep(Duration::from_secs_f64(secs)).await;
        Ok(())
    })
}

#[pymodule]
fn invalid_state_error(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sleep, m)?)?;
    Ok(())
}
