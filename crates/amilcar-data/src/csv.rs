use std::io;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsvSummary {
    pub headers: Vec<String>,
    pub columns: usize,
    pub rows: usize,
}

#[derive(Debug, Error)]
pub enum CsvProcessingError {
    #[error("CSV parsing or serialization failed: {0}")]
    Csv(#[from] ::csv::Error),
    #[error("CSV I/O failed: {0}")]
    Io(#[from] io::Error),
    #[error("CSV output was not valid UTF-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("CSV column not found: {0}")]
    MissingColumn(String),
    #[error("at least one CSV column must be selected")]
    EmptySelection,
}

/// Parse CSV using the first row as headers and require consistent record widths.
pub fn inspect(input: &str) -> Result<CsvSummary, CsvProcessingError> {
    let mut reader = ::csv::ReaderBuilder::new().from_reader(input.as_bytes());
    let headers = reader.headers()?.clone();
    let mut rows = 0_usize;
    for record in reader.records() {
        record?;
        rows += 1;
    }
    let headers = headers.iter().map(str::to_owned).collect::<Vec<_>>();
    Ok(CsvSummary {
        columns: headers.len(),
        headers,
        rows,
    })
}

pub fn validate(input: &str) -> Result<CsvSummary, CsvProcessingError> {
    inspect(input)
}

/// Rewrite CSV data with consistent quoting and record separators.
pub fn format(input: &str) -> Result<String, CsvProcessingError> {
    let mut reader = ::csv::ReaderBuilder::new().from_reader(input.as_bytes());
    let headers = reader.headers()?.clone();
    let mut writer = ::csv::WriterBuilder::new().from_writer(Vec::new());
    writer.write_record(&headers)?;
    for record in reader.records() {
        writer.write_record(&record?)?;
    }
    let bytes = writer
        .into_inner()
        .map_err(|error| CsvProcessingError::Io(error.into_error()))?;
    Ok(String::from_utf8(bytes)?)
}

/// Select exact header names, preserving row order and requested column order.
pub fn select_columns(input: &str, selected: &[String]) -> Result<String, CsvProcessingError> {
    if selected.is_empty() {
        return Err(CsvProcessingError::EmptySelection);
    }
    let mut reader = ::csv::ReaderBuilder::new().from_reader(input.as_bytes());
    let headers = reader.headers()?.clone();
    let mut indexes = Vec::with_capacity(selected.len());
    for column in selected {
        let index = headers
            .iter()
            .position(|header| header == column)
            .ok_or_else(|| CsvProcessingError::MissingColumn(column.clone()))?;
        indexes.push(index);
    }
    let mut writer = ::csv::WriterBuilder::new().from_writer(Vec::new());
    writer.write_record(indexes.iter().map(|index| &headers[*index]))?;
    for record in reader.records() {
        let record = record?;
        writer.write_record(indexes.iter().map(|index| record.get(*index).unwrap_or("")))?;
    }
    let bytes = writer
        .into_inner()
        .map_err(|error| CsvProcessingError::Io(error.into_error()))?;
    Ok(String::from_utf8(bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inspects_and_validates_csv() {
        let summary = inspect("name,age\nAna,20\nLuis,30\n").unwrap();
        assert_eq!(summary.headers, vec!["name", "age"]);
        assert_eq!(summary.columns, 2);
        assert_eq!(summary.rows, 2);
        assert_eq!(validate("a,b\n1,2\n").unwrap().rows, 1);
    }

    #[test]
    fn rejects_inconsistent_records() {
        assert!(validate("a,b\n1,2,3\n").is_err());
    }

    #[test]
    fn selects_requested_columns_in_requested_order() {
        let result = select_columns(
            "name,age,city\nAna,20,Lima\n",
            &["city".into(), "name".into()],
        )
        .unwrap();
        assert!(result.contains("city,name"));
        assert!(result.contains("Lima,Ana"));
    }

    #[test]
    fn rejects_invalid_column_selection() {
        assert!(matches!(
            select_columns("name\nAna\n", &["age".into()]),
            Err(CsvProcessingError::MissingColumn(_))
        ));
        assert!(matches!(
            select_columns("name\nAna\n", &[]),
            Err(CsvProcessingError::EmptySelection)
        ));
    }

    #[test]
    fn formats_csv() {
        let result = format("name,city\nAna,Lima\n").unwrap();
        assert!(result.contains("name,city"));
        assert_eq!(inspect(&result).unwrap().rows, 1);
    }
}
