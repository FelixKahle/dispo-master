// Copyright 2023 Felix Kahle. All rights reserved.

use crate::{file_parsing::ColumnMapping, parse_error::ParseFilesError};
use chrono::NaiveDateTime;
use num_traits::{Num, NumCast};
use polars::frame::DataFrame;
use std::fmt::{self};

/// The DispoMode enum represents the different modes of a dispo operation
/// * Delivery: The dispo operation is a delivery
/// * Pickup: The dispo operation is a pickup
#[derive(serde::Serialize, Debug, Copy, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DispoMode {
    Delivery,
    Pickup,
}

#[derive(Debug)]
pub struct StringToDispoModeError(String);

impl fmt::Display for StringToDispoModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing '{}' to a DispoMode. Expected 'Delivery' or 'Pickup'", self.0)
    }
}

impl std::error::Error for StringToDispoModeError {}

impl DispoMode {
    /// Create a DispoMode from a string.
    /// The string must be one of the following:
    /// * Delivery
    /// * Pickup
    ///
    /// # Arguments
    /// * `value` - The string to create the DispoMode from
    ///
    /// # Returns
    /// * Result containing the DispoMode or an error
    pub fn from_str(value: &str) -> Result<Self, StringToDispoModeError> {
        match value {
            "Delivery" => Ok(DispoMode::Delivery),
            "Pickup" => Ok(DispoMode::Pickup),
            _ => Err(StringToDispoModeError(format!("{}", value))),
        }
    }
}

impl fmt::Display for DispoMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DispoMode::Delivery => write!(f, "Delivery"),
            DispoMode::Pickup => write!(f, "Pickup"),
        }
    }
}

/// The TemperatureRange enum represents the different temperature ranges of a dispo operation
#[derive(serde::Serialize, Debug, Copy, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TemperatureRange {
    DryIce,
    DryShipper,
    Refrigerated,
    ControlledAmbient,
    Frozen,
    Ambient,
    NonSOP,
    Invalid,
}

#[derive(Debug, Clone)]
pub struct StringToTemperatureRangeError(String);

impl fmt::Display for StringToTemperatureRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "String can not be parsed to TemperatureRange: {}", self.0)
    }
}

impl TemperatureRange {
    /// Create a TemperatureRange from a string.
    /// The string must be one of the following:
    /// * Frozen Dry Ice -80C to -20C
    /// * Deep Frozen Dry Ice -70C [+/-10C]
    /// * Cryogenics -190C to -150C
    /// * Refrigerated +2C to +8C
    /// * Controlled Ambient +15C to +25C
    /// * Frozen -25C to -15C
    /// * Ambient
    /// * Frozen -50C  [+/-10C]
    ///
    /// # Arguments
    /// * `value` - The string to create the TemperatureRange from
    ///
    /// # Returns
    /// * Result containing the TemperatureRange or an error
    fn from_str(value: &str) -> Result<Self, StringToTemperatureRangeError> {
        match value {
            "Frozen Dry Ice -80C to -20C" => Ok(TemperatureRange::DryIce),
            "Deep Frozen Dry Ice -70C [+/-10C]" => Ok(TemperatureRange::DryIce),
            "Cryogenics -190C to -150C" => Ok(TemperatureRange::DryShipper),
            "Refrigerated +2C to +8C" => Ok(TemperatureRange::Refrigerated),
            "Controlled Ambient +15C to +25C" => Ok(TemperatureRange::ControlledAmbient),
            "Frozen -25C to -15C" => Ok(TemperatureRange::Frozen),
            "Ambient" => Ok(TemperatureRange::Ambient),
            "Frozen -50C  [+/-10C]" => Ok(TemperatureRange::NonSOP),
            _ => Err(StringToTemperatureRangeError(value.to_owned())),
        }
    }
}

impl fmt::Display for TemperatureRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemperatureRange::DryIce => write!(f, "Dry Ice"),
            TemperatureRange::DryShipper => write!(f, "Dry Shipper"),
            TemperatureRange::Refrigerated => write!(f, "Refrigerated"),
            TemperatureRange::ControlledAmbient => write!(f, "Controlled Ambient"),
            TemperatureRange::Frozen => write!(f, "Frozen"),
            TemperatureRange::Ambient => write!(f, "Ambient"),
            TemperatureRange::NonSOP => write!(f, "Non SOP"),
            TemperatureRange::Invalid => write!(f, "Invalid"),
        }
    }
}

/// The JobRow struct represents a single row of a dispo operation
/// * mode: The mode of the dispo operation
/// * job_number: The job number of the dispo operation
/// * hawb_number: The hawb number of the dispo operation
/// * temperature_range: The temperature range of the dispo operation
/// * quantities: The amount of elements of the dispo operation
/// * address: The address of the dispo operation
/// * postal_code: The postal code of the dispo operation
/// * city: The city of the dispo operation
/// * country: The country of the dispo operation
/// * equipment: The equipment of the dispo operation
/// * tolerance: The tolerance of the dispo operation
/// * early_date: The early date of the dispo operation
/// * late_date: The late date of the dispo operation
/// * calculated_date: The calculated date of the dispo operation.
/// * contact_name: The contact name of the dispo operation
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct JobRow {
    pub mode: DispoMode,
    pub job_number: String,
    pub hawb_number: String,
    pub temperature_ranges: Vec<TemperatureRange>,
    pub quantities: i32,
    pub address: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub equipment: String,
    /// The tolerance of the dispo operation
    /// Should be 0, 15, 30, 60, 120
    pub tolerance: i32,
    /// The calculated date of the dispo operation.
    /// This is being calculated as the middle between the early and late date.
    pub early_date: NaiveDateTime,
    pub late_date: NaiveDateTime,
    pub calculated_date: NaiveDateTime,
    pub contact_name: String,
}

impl fmt::Display for JobRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let temperature_range = self
            .temperature_ranges
            .iter()
            .map(|range| range.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(
            f,
            "JobRow {{
                mode: {},
                job_number: {},
                hawb_number: {},
                temperature_range: {},
                quantities: {},
                address: {},
                postal_code: {},
                city: {},
                country: {},
                equipment: {},
                tolerance: {},
                early_date: {},
                late_date: {},
                calculated_date: {},
                contact_name: {}
            }}",
            self.mode,
            self.job_number,
            self.hawb_number,
            temperature_range,
            self.quantities,
            self.address,
            self.postal_code,
            self.city,
            self.country,
            self.equipment,
            self.tolerance,
            self.early_date,
            self.late_date,
            self.calculated_date,
            self.contact_name
        )
    }
}

#[allow(dead_code)]
impl JobRow {
    /// Create a new JobRow
    ///
    /// # Arguments
    /// * `mode` - The mode of the dispo operation
    /// * `job_number` - The job number of the dispo operation
    /// * `hawb_number` - The hawb number of the dispo operation
    /// * `temperature_range` - The temperature range of the dispo operation
    /// * `amount` - The amount of elements of the dispo operation  
    /// * `address` - The address of the dispo operation
    /// * `postal_code` - The postal code of the dispo operation
    /// * `city` - The city of the dispo operation
    /// * `country` - The country of the dispo operation
    /// * `equipment` - The equipment of the dispo operation
    /// * `tolerance` - The tolerance of the dispo operation
    /// * `early_date` - The early date of the dispo operation
    /// * `late_date` - The late date of the dispo operation
    /// * `date` - The date of the dispo operation
    /// * `contact_name` - The contact name of the dispo operation
    ///
    /// # Returns
    /// * A new JobRow
    pub fn new(
        mode: DispoMode,
        job_number: String,
        hawb_number: String,
        temperature_range: Vec<TemperatureRange>,
        amount: i32,
        address: String,
        postal_code: String,
        city: String,
        country: String,
        equipment: String,
        tolerance: i32,
        early_date: NaiveDateTime,
        late_date: NaiveDateTime,
        calculated_date: NaiveDateTime,
        contact_name: String,
    ) -> Self {
        JobRow {
            mode,
            job_number,
            hawb_number,
            temperature_ranges: temperature_range,
            quantities: amount,
            address,
            postal_code,
            city,
            country,
            equipment,
            tolerance,
            early_date,
            late_date,
            calculated_date,
            contact_name,
        }
    }

    /// Create a vector of JobRow from a polars DataFrame
    ///
    /// # Arguments
    /// * `df` - The DataFrame to create the JobRow from
    /// * `mode` - The mode of the dispo operation
    ///
    /// # Returns
    /// * Result containing a vector of JobRow or an error
    pub fn from_dataframe(df: &polars::prelude::DataFrame, mode: DispoMode) -> Result<Vec<JobRow>, ParseFilesError> {
        let column_mapping = ColumnMapping::new(mode);

        let job_numbers = extract_column_as_string(df, &column_mapping.job_number)?;
        let hawb_numbers = extract_column_as_string(df, &column_mapping.hawb)?;
        let temperature_ranges = extract_column_as_temperature_ranges(df, column_mapping.temperature_range)?;
        let addresses = extract_column_as_string(df, &column_mapping.address)?;
        let quantities: Vec<i32> = extract_column_as_i32(df, &column_mapping.quantity)?;
        let postal_codes = extract_column_as_string(df, &column_mapping.postal_code)?;
        let cities = extract_column_as_string(df, &column_mapping.city)?;
        let countries = extract_column_as_string(df, &column_mapping.country)?;
        let equipment = extract_column_as_string(df, &column_mapping.equipment_codes)?;
        let contact_names = extract_column_as_string(df, &column_mapping.name)?;
        let early_dates: Vec<NaiveDateTime> = df
            .column(&column_mapping.target_early)?
            .iter()
            .map(|cell| any_value_to_naive_date_time(&cell, "%m/%d/%Y %H:%M").unwrap_or_default())
            .collect();
        let late_dates: Vec<NaiveDateTime> = df
            .column(&column_mapping.target_late)?
            .iter()
            .map(|cell| any_value_to_naive_date_time(&cell, "%m/%d/%Y %H:%M").unwrap_or_default())
            .collect();

        let total_elements = df.height();
        let mut result = Vec::<JobRow>::with_capacity(total_elements);

        for index in 0..total_elements {
            let row = JobRow::new(
                mode,
                job_numbers.get(index).unwrap_or(&String::new()).to_string(),
                hawb_numbers.get(index).unwrap_or(&String::new()).to_string(),
                temperature_ranges.get(index).cloned().unwrap_or(vec![TemperatureRange::Invalid]),
                quantities.get(index).unwrap_or(&-1).to_owned(),
                addresses.get(index).unwrap_or(&String::new()).to_string(),
                postal_codes.get(index).unwrap_or(&String::new()).to_string(),
                cities.get(index).unwrap_or(&String::new()).to_string(),
                countries.get(index).unwrap_or(&String::new()).to_string(),
                equipment.get(index).unwrap_or(&String::new()).to_string(),
                calculate_tolerance(
                    early_dates.get(index).cloned().unwrap_or_default(),
                    late_dates.get(index).cloned().unwrap_or_default(),
                ),
                early_dates.get(index).cloned().unwrap_or_default(),
                late_dates.get(index).cloned().unwrap_or_default(),
                middle_between_dates(
                    early_dates.get(index).cloned().unwrap_or_default(),
                    late_dates.get(index).cloned().unwrap_or_default(),
                ),
                contact_names.get(index).unwrap_or(&String::new()).to_string(),
            );

            result.push(row);
        }

        Ok(result)
    }
}

/// Extract a column from a DataFrame as a vector of strings
///
/// # Arguments
/// * `df` - The DataFrame to extract the column from
/// * `column_name` - The name of the column to extract
///
/// # Returns
/// * Result containing a vector of strings or an error
fn extract_column_as_string(df: &DataFrame, column_name: &str) -> Result<Vec<String>, polars::prelude::PolarsError> {
    Ok(df
        .column(column_name)?
        .iter()
        .map(|cell| match cell {
            polars::prelude::AnyValue::Utf8(s) => s.to_owned(),
            polars::prelude::AnyValue::Utf8Owned(s) => s.to_string(),
            _ => cell.to_string(),
        })
        .collect())
}

/// Extract the temperature ranges from a string
/// The string must be a comma separated list of temperature ranges
/// If the string is empty, the Ambient temperature range is returned
///
/// # Arguments
/// * `input` - The string to extract the temperature ranges from
///
/// # Returns
/// * A vector of TemperatureRange
fn extract_column_as_temperature_ranges(
    df: &DataFrame,
    column_name: &str,
) -> Result<Vec<Vec<TemperatureRange>>, polars::prelude::PolarsError> {
    Ok(df
        .column(column_name)?
        .iter()
        .map(|cell| match cell {
            polars::prelude::AnyValue::Utf8(s) => extract_temperature_ranges(s),
            polars::prelude::AnyValue::Utf8Owned(s) => extract_temperature_ranges(&s),
            _ => vec![TemperatureRange::Ambient],
        })
        .collect())
}

/// Extract the temperature ranges from a string
/// The string must be a comma separated list of temperature ranges or a single temperature range
/// If the string is empty, the Ambient temperature range is returned
/// If the string is invalid, the Invalid temperature range is returned
///
/// # Arguments
/// * `input` - The string to extract the temperature ranges from
///
/// # Returns
/// * A vector of TemperatureRange
fn extract_temperature_ranges(input: &str) -> Vec<TemperatureRange> {
    if input.is_empty() {
        return vec![TemperatureRange::Ambient];
    }

    let splitted: Vec<&str> = input.split(",").collect();
    splitted
        .iter()
        .map(|s| TemperatureRange::from_str(s.trim()).unwrap_or_else(|_| TemperatureRange::Invalid))
        .collect()
}

/// Extract a column from a DataFrame as a vector of i32
///
/// # Note
/// This function tries to read the cells as a f32 and then converts them to i32
///
/// # Arguments
/// * `df` - The DataFrame to extract the column from
/// * `column_name` - The name of the column to extract
///
/// # Returns
/// * Result containing a vector of i32 or an error
fn extract_column_as_i32(df: &DataFrame, column_name: &str) -> Result<Vec<i32>, ParseFilesError> {
    let column = df.column(column_name).map_err(ParseFilesError::from)?;

    column
        .iter()
        .map(|cell| {
            any_value_to_numeric::<f32>(&cell)
                .map_err(ParseFilesError::from)
                .and_then(|num| Ok(num as i32))
        })
        .collect()
}

/// Error type for the any_value_to_numeric function
///
/// # Variants
/// * InvalidType: The AnyValue is not a numeric type
/// * StringParseError: The AnyValue is a string but could not be parsed
/// * ParseError: The AnyValue could not be parsed
#[derive(Debug)]
pub enum AnyValueToNumericParseError {
    InvalidType(String),
    StringParseError(String),
    ParseError(String),
}

impl fmt::Display for AnyValueToNumericParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnyValueToNumericParseError::InvalidType(value) => write!(f, "Value can not parsed to numeric: {}", value),
            AnyValueToNumericParseError::StringParseError(value) => write!(f, "Error parsing string to numeric: {}", value),
            AnyValueToNumericParseError::ParseError(value) => write!(f, "Parse error: {}", value),
        }
    }
}

impl std::error::Error for AnyValueToNumericParseError {}

/// Convert a polars AnyValue to a numeric type
///
/// # Type parameters
/// * `F` - The numeric type to convert to
///
/// # Type restrictions
/// * `F` must implement the Num, NumCast and FromStr traits
///
/// # Arguments
/// * `value` - The AnyValue to convert
///
/// # Returns
/// * Result containing the numeric value or an error
fn any_value_to_numeric<'a, F>(value: &polars::prelude::AnyValue) -> Result<F, AnyValueToNumericParseError>
where
    F: Num + NumCast + core::str::FromStr,
{
    match value {
        polars::prelude::AnyValue::Utf8(s) => {
            if let Ok(i) = s.parse::<F>() {
                Ok(i)
            } else {
                Err(AnyValueToNumericParseError::StringParseError(value.to_string()))
            }
        }
        polars::prelude::AnyValue::Utf8Owned(s) => {
            if let Ok(i) = s.as_str().parse::<F>() {
                Ok(i)
            } else {
                Err(AnyValueToNumericParseError::StringParseError(value.to_string()))
            }
        }
        polars::prelude::AnyValue::UInt8(i) => NumCast::from(*i).ok_or(AnyValueToNumericParseError::ParseError(value.to_string())),
        polars::prelude::AnyValue::UInt16(i) => NumCast::from(*i).ok_or(AnyValueToNumericParseError::ParseError(value.to_string())),
        polars::prelude::AnyValue::UInt32(i) => NumCast::from(*i).ok_or(AnyValueToNumericParseError::ParseError(value.to_string())),
        polars::prelude::AnyValue::UInt64(i) => NumCast::from(*i).ok_or(AnyValueToNumericParseError::ParseError(value.to_string())),
        polars::prelude::AnyValue::Int8(i) => NumCast::from(*i).ok_or(AnyValueToNumericParseError::ParseError(value.to_string())),
        polars::prelude::AnyValue::Int16(i) => NumCast::from(*i).ok_or(AnyValueToNumericParseError::ParseError(value.to_string())),
        polars::prelude::AnyValue::Int32(i) => NumCast::from(*i).ok_or(AnyValueToNumericParseError::ParseError(value.to_string())),
        polars::prelude::AnyValue::Int64(i) => NumCast::from(*i).ok_or(AnyValueToNumericParseError::ParseError(value.to_string())),
        polars::prelude::AnyValue::Float32(f) => NumCast::from(*f).ok_or(AnyValueToNumericParseError::ParseError(value.to_string())),
        polars::prelude::AnyValue::Float64(f) => NumCast::from(*f).ok_or(AnyValueToNumericParseError::ParseError(value.to_string())),
        _ => Err(AnyValueToNumericParseError::InvalidType(value.to_string())),
    }
}

/// Error type for the any_value_to_naive_date_time function
///
/// # Variants
/// * InvalidType: The AnyValue is not a string
/// * ParseError: The AnyValue is a string but could not be parsed
#[derive(Debug)]
pub enum AnyValueToNaiveDateTimeParseError {
    InvalidType(String),
    ParseError(String),
}

impl fmt::Display for AnyValueToNaiveDateTimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnyValueToNaiveDateTimeParseError::InvalidType(value) => write!(f, "Value can not parsed to NaiveDateTime: {}", value),
            AnyValueToNaiveDateTimeParseError::ParseError(value) => write!(f, "Error parsing string to NaiveDateTime: {}", value),
        }
    }
}

impl std::error::Error for AnyValueToNaiveDateTimeParseError {}

/// Convert a polars AnyValue to a NaiveDateTime
/// The format string must be a valid format string for the NaiveDateTime::parse_from_str function
///
/// # Arguments
/// * `value` - The AnyValue to convert
/// * `format` - The format string to use for parsing
///
/// # Returns
/// * Result containing the NaiveDateTime or an error
fn any_value_to_naive_date_time(
    value: &polars::prelude::AnyValue,
    format: &str,
) -> Result<NaiveDateTime, AnyValueToNaiveDateTimeParseError> {
    match value {
        polars::prelude::AnyValue::Utf8(date_str) => {
            if let Ok(d) = NaiveDateTime::parse_from_str(&date_str, format) {
                Ok(d)
            } else {
                Err(AnyValueToNaiveDateTimeParseError::ParseError(value.to_string()))
            }
        }
        polars::prelude::AnyValue::Utf8Owned(date_str) => {
            if let Ok(d) = NaiveDateTime::parse_from_str(&date_str, format) {
                Ok(d)
            } else {
                Err(AnyValueToNaiveDateTimeParseError::ParseError(value.to_string()))
            }
        }
        _ => Err(AnyValueToNaiveDateTimeParseError::InvalidType(value.to_string())),
    }
}

/// Calculate the middle between two dates
/// It does not matter which date is the earlier and which is the later date
/// They can both be the same date, in that case the same date is returned.
///
/// # Arguments
/// * `date1` - The first date
/// * `date2` - The second date
///
/// # Returns
/// * The middle between the two dates
fn middle_between_dates(date1: NaiveDateTime, date2: NaiveDateTime) -> NaiveDateTime {
    // Determine the earlier and later dates
    let (earlier_date, later_date) = if date1 < date2 { (date1, date2) } else { (date2, date1) };
    // Calculate the duration between the two dates
    let duration = later_date - earlier_date;
    // Calculate half of the duration
    let half_duration = duration / 2;
    // Add half of the duration to the earlier date to get the middle date
    earlier_date + half_duration
}

/// Calculate the difference between two dates in minutes
/// It does not matter which date is the earlier and which is the later date
/// They can both be the same date, in that case 0 is returned.
/// If the first date is earlier than the second date, the difference is positive.
/// If the first date is later than the second date, the difference is negative.
///
/// # Arguments
/// * `date1` - The first date
/// * `date2` - The second date
///
/// # Returns
/// * The difference between the two dates in minutes
#[allow(dead_code)]
fn difference_in_minutes(date1: NaiveDateTime, date2: NaiveDateTime) -> i64 {
    // Calculate the duration between the two dates
    let duration = date1.signed_duration_since(date2);
    // Get the absolute value of the duration in minutes
    duration.num_minutes()
}

/// Calculate the tolerance for a dispo operation using a edge date and a middle date
/// The edge date is the early or late date of the dispo operation
/// The middle date is the calculated date of the dispo operation
/// The tolerance is calculated as follows:
/// * If the difference between the edge date and the middle date is equal to 0 minutes, the tolerance is 0
/// * If the difference between the edge date and the middle date is less than or equal to 15 minutes, the tolerance is 15
/// * If the difference between the edge date and the middle date is less than or equal to 30 minutes, the tolerance is 30
/// * If the difference between the edge date and the middle date is less than or equal to 60 minutes, the tolerance is 60
/// * If the difference between the edge date and the middle date is greater than 60 minutes, the tolerance is 120
///
/// # Arguments
/// * `edge_date` - The edge date of the dispo operation
/// * `middle_date` - The middle date of the dispo operation
///
/// # Returns
/// * The tolerance of the dispo operation
#[allow(dead_code)]
fn calculate_tolerance_middle_date(edge_date: NaiveDateTime, middle_date: NaiveDateTime) -> i32 {
    let difference = difference_in_minutes(edge_date, middle_date).abs();

    if difference <= 0 {
        0
    } else if difference <= 15 {
        15
    } else if difference <= 30 {
        30
    } else if difference <= 60 {
        60
    } else {
        120
    }
}

/// Calculate the tolerance for a dispo operation using a early date and a late date
/// The early date is the early date of the dispo operation
/// The late date is the late date of the dispo operation
/// The tolerance is calculated as follows:
/// * If the difference between the edge date and the middle date is equal to 0 minutes, the tolerance is 0
/// * If the difference between the edge date and the middle date is less than or equal to 15 minutes, the tolerance is 15
/// * If the difference between the edge date and the middle date is less than or equal to 30 minutes, the tolerance is 30
/// * If the difference between the edge date and the middle date is less than or equal to 60 minutes, the tolerance is 60
/// * If the difference between the edge date and the middle date is greater than 60 minutes, the tolerance is 120
///
/// # Arguments
/// * `early_date` - The early date of the dispo operation
/// * `late_date` - The late date of the dispo operation
///
/// # Returns
/// * The tolerance of the dispo operation
#[allow(dead_code)]
fn calculate_tolerance(early_date: NaiveDateTime, late_date: NaiveDateTime) -> i32 {
    let middle_date = middle_between_dates(early_date, late_date);
    calculate_tolerance_middle_date(early_date, middle_date)
}
