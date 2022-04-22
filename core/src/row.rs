
use tiberius::{ColumnData, Column};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TokenRow {
    data: Vec<ColumnData<'static>>,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub(crate) columns: Arc<Vec<Column>>,
    pub(crate) data: TokenRow,
    pub(crate) result_index: usize,
}