
use crate::parser::proto::simple_proto::STxIn;
use crate::{derive_block_to_py, derive_ftx_to_py, derive_outpoint_to_py, derive_stx_to_py};
use bitcoin::{OutPoint, TxIn};
use bitcoin_explorer::parser::script::ScriptInfo;
pub use bitcoin_explorer::*;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

pub trait ToPy {
    /// Converts self into a Python object.
    fn to_py(&self, py: Python) -> PyResult<PyObject>;
}

derive_block_to_py!(SBlock);
derive_block_to_py!(FBlock);
derive_block_to_py!(SConnectedBlock);
derive_block_to_py!(FConnectedBlock);
derive_outpoint_to_py!(OutPoint);