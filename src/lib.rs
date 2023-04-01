
#[macro_use]
mod proto_to_py;

#[doc(inline)]
use bitcoin_explorer::*;
use proto_to_py::*;
use pyo3::prelude::*;
use pyo3::Python;
use std::ops::Deref;
use std::path::Path;

#[pyclass(name = "BitcoinDB")]
struct BitcoinDBPy(BitcoinDB);

impl Deref for BitcoinDBPy {
    type Target = BitcoinDB;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[pymethods]
impl BitcoinDBPy {
    #[new]
    fn new(path: &str, tx_index: bool) -> PyResult<Self> {
        let path = Path::new(path);
        match BitcoinDB::new(path, tx_index) {
            Ok(db) => Ok(BitcoinDBPy(db)),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, height, /)")]
    fn get_block_full(&self, height: usize, py: Python) -> PyResult<PyObject> {
        match self.get_block::<FBlock>(height) {
            Ok(block) => block.to_py(py),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, height, /)")]
    fn get_block_simple(&self, height: usize, py: Python) -> PyResult<PyObject> {
        match self.get_block::<SBlock>(height) {
            Ok(block) => block.to_py(py),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, height, /)")]
    fn get_block_full_connected(&self, height: usize, py: Python) -> PyResult<PyObject> {
        match self.get_connected_block::<FConnectedBlock>(height) {
            Ok(block) => block.to_py(py),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, height, /)")]
    fn get_block_simple_connected(&self, height: usize, py: Python) -> PyResult<PyObject> {
        match self.get_connected_block::<SConnectedBlock>(height) {
            Ok(block) => block.to_py(py),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, height, /)")]
    fn get_block_header(&self, height: usize, py: Python) -> PyResult<PyObject> {
        match self.get_header(height) {
            Ok(block) => block.to_py(py),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, height, /)")]
    fn get_hash_from_height(&self, height: usize) -> PyResult<String> {
        match self.0.get_hash_from_height(height) {
            Ok(b) => Ok(b.to_hex()),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, hash, /)")]
    fn get_height_from_hash(&self, hash: String) -> PyResult<usize> {
        if let Ok(blk_hash) = BlockHash::from_hex(&hash) {
            match self.0.get_height_from_hash(&blk_hash) {
                Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
                Ok(h) => Ok(h),
            }
        } else {
            Err(pyo3::exceptions::PyException::new_err(
                "invalid txid format",
            ))
        }
    }

    #[pyo3(text_signature = "($self, txid, /)")]
    fn get_height_from_txid(&self, txid: String) -> PyResult<usize> {
        if let Ok(txid) = Txid::from_hex(&txid) {
            match self.get_height_of_transaction(&txid) {
                Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
                Ok(h) => Ok(h),
            }
        } else {
            Err(pyo3::exceptions::PyException::new_err(
                "invalid txid format",
            ))
        }
    }

    #[pyo3(text_signature = "($self, txid, /)")]
    fn get_transaction_full(&self, txid: String, py: Python) -> PyResult<PyObject> {
        if let Ok(txid) = Txid::from_hex(&txid) {
            match self.get_transaction::<FTransaction>(&txid) {
                Ok(t) => t.to_py(py),
                Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
            }
        } else {
            Err(pyo3::exceptions::PyException::new_err(
                "invalid txid format",
            ))
        }
    }

    #[pyo3(text_signature = "($self, txid, /)")]