#[cfg(test)]

use cpython::{PyResult, Python, PyTuple, PyErr, exc, ToPyObject, PythonObject};

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
// Steps:
//Evaluation:
// 1: get Weight matrices
// 2: do bfs, compute total probability
// 3: do DP get correct outcomes
// 4: return outcomes


//Steps:
// Training
// 1. For each pixel
// a) if it is on a boundary, skip
// b) otherwise, use formula to increment W
// c) return 
py_module_initalizer!(segCRF)
