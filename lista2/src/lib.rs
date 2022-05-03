mod tsp;
use tsp::tabu::*;
use tsp::geo::*;
use tsp::io::*;
use tsp::gen::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn tsp_tabu(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tabu_search, m)?)?;
    m.add_function(wrap_pyfunction!(tabu_search_no_threads, m)?)?;
    m.add_function(wrap_pyfunction!(read_file, m)?)?;
    m.add_function(wrap_pyfunction!(read_tour, m)?)?;
    m.add_function(wrap_pyfunction!(create_euclid, m)?)?;
    m.add_function(wrap_pyfunction!(create_atsp, m)?)?;
    m.add_function(wrap_pyfunction!(create_tsp, m)?)?;
    m.add_class::<Matrix>()?;
    Ok(())
}