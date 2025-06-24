//! Ingest Arrow RecordBatch and write Parquet (memory demo).

use arrow::{array::Int32Array, record_batch::RecordBatch, datatypes::{Schema, Field, DataType}};
use parquet::arrow::arrow_writer::ArrowWriter;
use anyhow::Result;

pub fn batch_to_parquet_mem() -> Result<Vec<u8>> {
    let schema = Schema::new(vec![Field::new("v", DataType::Int32, false)]);
    let arr = Int32Array::from(vec![1, 2, 3]);
    let batch = RecordBatch::try_new(std::sync::Arc::new(schema.clone()), vec![std::sync::Arc::new(arr)])?;

    let mut buf = Vec::new();
    let mut writer = ArrowWriter::try_new(&mut buf, schema, None)?;
    writer.write(&batch)?;
    writer.close()?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn writes() { assert!(!batch_to_parquet_mem().unwrap().is_empty()); }
}
