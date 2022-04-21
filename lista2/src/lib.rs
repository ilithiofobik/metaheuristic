mod tsp;
use tsp::tabu::*;
use tsp::geo::*;
use tsp::io::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn tsp_tabu(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tabu_search, m)?)?;
    m.add_function(wrap_pyfunction!(read_file, m)?)?;
    m.add_function(wrap_pyfunction!(read_tour, m)?)?;
    m.add_class::<Matrix>()?;
    Ok(())
}