mod tsp;
use tsp::geo::*;
use tsp::io::*;
use tsp::gen::*;
use tsp::pop::*;
use tsp::alg::*;
use tsp::tabu::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn tsp_pop(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(two_opt, m)?)?;
    m.add_function(wrap_pyfunction!(tabu_search, m)?)?;
    m.add_function(wrap_pyfunction!(read_file, m)?)?;
    m.add_function(wrap_pyfunction!(read_tour, m)?)?;
    m.add_function(wrap_pyfunction!(create_euclid, m)?)?;
    m.add_function(wrap_pyfunction!(create_atsp, m)?)?;
    m.add_function(wrap_pyfunction!(create_tsp, m)?)?;
    m.add_function(wrap_pyfunction!(population_alg_no_threads_no_isles, m)?)?;
    m.add_function(wrap_pyfunction!(population_alg_no_threads_isles, m)?)?;
    m.add_function(wrap_pyfunction!(population_alg_threads_no_isles, m)?)?;    m.add_function(wrap_pyfunction!(population_alg_threads_no_isles, m)?)?;
    m.add_function(wrap_pyfunction!(population_alg_threads_isles, m)?)?;
    m.add_class::<Matrix>()?;
    Ok(())
}