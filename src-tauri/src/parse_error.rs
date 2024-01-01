// Copyright 2023 Felix Kahle. All rights reserved.

use tauri::InvokeError;

use crate::job_row::{
    AnyValueToNaiveDateTimeParseError, AnyValueToNumericParseError, StringToDispoModeError, StringToTemperatureRangeError,
};

/// This error includes all errors that can occur while parsing files
///
/// # Variants
/// * `CalamineError` - An error that occurred while parsing the Excel files
/// * `PolarsError` - An error that occurred while converting the Excel files to DataFrames
/// * `NoHeadersFound` - No headers were found in the Excel files
/// * `InvalidSheetCount` - The number of sheets in the Excel file is not equal to the number of sheets expected
/// * `AnyValueToNumericParse` - An error that occurred while parsing a value to a numeric type
/// * `AnyValueToNaiveDateTimeParse` - An error that occurred while parsing a value to a NaiveDateTime type
/// * `StringToDispoMode` - An error that occurred while parsing a string to a DispoMode
/// * `StringToTemperatureRange` - An error that occurred while parsing a string to a TemperatureRange
#[derive(Debug)]
pub enum ParseFilesError {
    CalamineError(calamine::Error),
    PolarsError(polars::error::PolarsError),
    NoHeadersFound,
    MismatchedRowCount((i32, i32)),
    InvalidSheetCount((i32, i32)),
    AnyValueToNumericParse(AnyValueToNumericParseError),
    AnyValueToNaiveDateTimeParse(AnyValueToNaiveDateTimeParseError),
    StringToDispoMode(StringToDispoModeError),
    StringToTemperatureRange(StringToTemperatureRangeError),
}

impl From<calamine::Error> for ParseFilesError {
    fn from(error: calamine::Error) -> Self {
        ParseFilesError::CalamineError(error)
    }
}

impl From<calamine::XlsError> for ParseFilesError {
    fn from(error: calamine::XlsError) -> Self {
        ParseFilesError::CalamineError(calamine::Error::Xls(error))
    }
}

impl From<polars::error::PolarsError> for ParseFilesError {
    fn from(error: polars::error::PolarsError) -> Self {
        ParseFilesError::PolarsError(error)
    }
}

impl From<AnyValueToNumericParseError> for ParseFilesError {
    fn from(error: AnyValueToNumericParseError) -> Self {
        ParseFilesError::AnyValueToNumericParse(error)
    }
}

impl From<AnyValueToNaiveDateTimeParseError> for ParseFilesError {
    fn from(error: AnyValueToNaiveDateTimeParseError) -> Self {
        ParseFilesError::AnyValueToNaiveDateTimeParse(error)
    }
}

impl From<StringToDispoModeError> for ParseFilesError {
    fn from(error: StringToDispoModeError) -> Self {
        ParseFilesError::StringToDispoMode(error)
    }
}

impl From<StringToTemperatureRangeError> for ParseFilesError {
    fn from(error: StringToTemperatureRangeError) -> Self {
        ParseFilesError::StringToTemperatureRange(error)
    }
}

impl std::fmt::Display for ParseFilesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseFilesError::CalamineError(error) => write!(f, "CalamineError: {}", error),
            ParseFilesError::PolarsError(error) => write!(f, "PolarsError: {}", error),
            ParseFilesError::NoHeadersFound => write!(f, "NoHeadersFound"),
            ParseFilesError::MismatchedRowCount((first, second)) => {
                write!(f, "Mismatched row count. Found {} and {}", first, second)
            }
            ParseFilesError::InvalidSheetCount((expected, actual)) => write!(f, "Expected {} sheets, found {}", expected, actual),
            ParseFilesError::AnyValueToNumericParse(error) => write!(f, "AnyValueToNumericParseError: {}", error),
            ParseFilesError::AnyValueToNaiveDateTimeParse(error) => {
                write!(f, "AnyValueToNaiveDateTimeParseError: {}", error)
            }
            ParseFilesError::StringToDispoMode(error) => write!(f, "StringToDispoModeError: {}", error),
            ParseFilesError::StringToTemperatureRange(error) => {
                write!(f, "StringToTemperatureRangeError: {}", error)
            }
        }
    }
}

impl Into<InvokeError> for ParseFilesError {
    fn into(self) -> InvokeError {
        InvokeError::from(self.to_string())
    }
}

impl std::error::Error for ParseFilesError {}
