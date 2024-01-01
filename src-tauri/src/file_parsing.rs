// Copyright 2023 Felix Kahle. All rights reserved.

use std::fmt::{self, Display, Formatter};

use crate::{
    job_row::{DispoMode, JobRow},
    parse_error::ParseFilesError,
};
use calamine::{DataType, Reader, Xls};
use polars::{
    datatypes::AnyValue,
    prelude::{DataFrameJoinOps, NamedFrom},
};

// Column names from the .xls files downloaded from TMS
pub const JOB_NUMBER_COLUMN_NAME: &str = "Load #";
pub const HAWB_COLUMN_NAME: &str = "Ref: House Waybill Number";
pub const QUANTITY_COLUMN_NAME: &str = "Actual Quantity";
pub const SHIPPER_COLUMN_NAME: &str = "Shipper";
pub const SHIPPER_NAME_COLUMN_NAME: &str = "Shipper Name";
pub const SHIPPER_ADDRESS_COLUMN_NAME: &str = "Shipper Address";
pub const SHIPPER_CITY_COLUMN_NAME: &str = "Shipper City";
pub const SHIPPER_STATE_COLUMN_NAME: &str = "Shipper State";
pub const SHIPPER_POSTAL_CODE_COLUMN_NAME: &str = "Shipper Postal Code";
pub const SHIPPER_COUNTRY_COLUMN_NAME: &str = "Shipper Country";
pub const TARGET_DELIVERY_EARLY_COLUMN_NAME: &str = "Target Delivery (Early)";
pub const TARGET_DELIVERY_LATE_COLUMN_NAME: &str = "Target Delivery (Late)";
pub const TARGET_SHIP_EARLY_COLUMN_NAME: &str = "Target Ship (Early)";
pub const TARGET_SHIP_LATE_COLUMN_NAME: &str = "Target Ship (Late)";
pub const CONSIGNEE_COLUMN_NAME: &str = "Consignee";
pub const CONSIGNEE_NAME_COLUMN_NAME: &str = "Consignee Name";
pub const CONSIGNEE_ADDRESS_COLUMN_NAME: &str = "Consignee Address";
pub const CONSIGNEE_CITY_COLUMN_NAME: &str = "Consignee City";
pub const CONSIGNEE_STATE_COLUMN_NAME: &str = "Consignee State";
pub const CONSIGNEE_POSTAL_CODE_COLUMN_NAME: &str = "Consignee Postal Code";
pub const CONSIGNEE_COUNTRY_COLUMN_NAME: &str = "Consignee Country";
pub const EQUIPMENT_CODES_COLUMN_NAME: &str = "Equipment Codes";
pub const TEMPERATURE_RANGE_COLUMN_NAME: &str = "Ref: Temperature Range";

/// Based on the mode of the dispo operation, the column names are different.
/// This is a helper struct that maps to the correct column names based on the mode.
/// So for example, if the mode is DispoMode::Delivery, the job_number field maps to the
/// CONSIGNEE_ column names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnMapping {
    pub job_number: &'static str,
    pub hawb: &'static str,
    pub quantity: &'static str,
    pub equipment_codes: &'static str,
    pub temperature_range: &'static str,
    pub target_early: &'static str,
    pub target_late: &'static str,

    // Mode-specific fields
    pub info: &'static str,
    pub name: &'static str,
    pub address: &'static str,
    pub city: &'static str,
    pub state: &'static str,
    pub postal_code: &'static str,
    pub country: &'static str,
}

impl ColumnMapping {
    /// Create a new ColumnMapping based on the mode
    ///
    /// # Arguments
    /// * `mode` - The mode to create the ColumnMapping for
    pub fn new(mode: DispoMode) -> Self {
        Self {
            // Mode-independent columns
            job_number: JOB_NUMBER_COLUMN_NAME,
            hawb: HAWB_COLUMN_NAME,
            quantity: QUANTITY_COLUMN_NAME,
            equipment_codes: EQUIPMENT_CODES_COLUMN_NAME,
            temperature_range: TEMPERATURE_RANGE_COLUMN_NAME,

            target_early: match mode {
                DispoMode::Delivery => TARGET_DELIVERY_EARLY_COLUMN_NAME,
                DispoMode::Pickup => TARGET_SHIP_EARLY_COLUMN_NAME,
            },

            target_late: match mode {
                DispoMode::Delivery => TARGET_DELIVERY_LATE_COLUMN_NAME,
                DispoMode::Pickup => TARGET_SHIP_LATE_COLUMN_NAME,
            },

            info: match mode {
                DispoMode::Delivery => CONSIGNEE_COLUMN_NAME,
                DispoMode::Pickup => SHIPPER_COLUMN_NAME,
            },

            name: match mode {
                DispoMode::Delivery => CONSIGNEE_NAME_COLUMN_NAME,
                DispoMode::Pickup => SHIPPER_NAME_COLUMN_NAME,
            },
            address: match mode {
                DispoMode::Delivery => CONSIGNEE_ADDRESS_COLUMN_NAME,
                DispoMode::Pickup => SHIPPER_ADDRESS_COLUMN_NAME,
            },
            city: match mode {
                DispoMode::Delivery => CONSIGNEE_CITY_COLUMN_NAME,
                DispoMode::Pickup => SHIPPER_CITY_COLUMN_NAME,
            },
            state: match mode {
                DispoMode::Delivery => CONSIGNEE_STATE_COLUMN_NAME,
                DispoMode::Pickup => SHIPPER_STATE_COLUMN_NAME,
            },
            postal_code: match mode {
                DispoMode::Delivery => CONSIGNEE_POSTAL_CODE_COLUMN_NAME,
                DispoMode::Pickup => SHIPPER_POSTAL_CODE_COLUMN_NAME,
            },
            country: match mode {
                DispoMode::Delivery => CONSIGNEE_COUNTRY_COLUMN_NAME,
                DispoMode::Pickup => SHIPPER_COUNTRY_COLUMN_NAME,
            },
        }
    }
}

impl Display for ColumnMapping {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ColumnMapping {{
                Job Number: {},
                HAWB: {},
                Quantity: {},
                Equipment Codes: {},
                Temperature Range: {},
                Target Delivery Early: {},
                Target Delivery Late: {},
                Info: {},
                Name: {},
                Address: {},
                City: {},
                State: {},
                Postal Code: {},
                Country: {}
            }}",
            self.job_number,
            self.hawb,
            self.quantity,
            self.equipment_codes,
            self.temperature_range,
            self.target_early,
            self.target_late,
            self.info,
            self.name,
            self.address,
            self.city,
            self.state,
            self.postal_code,
            self.country
        )
    }
}

/// Remove all null bytes from a string.
///
/// # Arguments
/// * `input` - The string to decode
///
/// # Returns
/// * The decoded string
#[inline]
pub fn decode_text(input: &str) -> String {
    input.encode_utf16().filter(|&c| c != 0).map(|c| c as u8 as char).collect()
}

/// Remove all null bytes from a string.
/// Uses smartstring::alias::String instead of std::string::String
///
/// # Arguments
/// * `input` - The string to decode
///
/// # Returns
/// * The decoded string
#[inline]
pub fn decode_text_smart_string(input: &str) -> smartstring::alias::String {
    smartstring::alias::String::from(decode_text(input))
}

/// Convert a calamine::DataType to a polars::prelude::AnyValue
/// Strings are decoded from UTF-16LE to UTF-8
///
/// # Arguments
/// * `data` - The calamine::DataType to convert
///
/// # Returns
/// * The converted polars::prelude::AnyValue
#[allow(dead_code)]
fn data_type_to_any_value(data: &DataType) -> AnyValue {
    match data {
        DataType::String(s) => AnyValue::Utf8Owned(decode_text_smart_string(&s)),
        DataType::Float(f) => AnyValue::Float64(*f),
        DataType::Int(i) => AnyValue::Int64(*i),
        DataType::Bool(b) => AnyValue::Boolean(*b),
        DataType::Error(_) => AnyValue::Null,
        DataType::Empty => AnyValue::Null,
        DataType::DateTime(d) => AnyValue::Float64(*d),
        DataType::Duration(d) => AnyValue::Float64(*d),
        DataType::DateTimeIso(d) => AnyValue::Utf8Owned(decode_text_smart_string(&d)),
        DataType::DurationIso(d) => AnyValue::Utf8Owned(decode_text_smart_string(&d)),
    }
}

/// Get the header names from a calamine::Range
/// Strings are decoded from UTF-16LE to UTF-8
/// All other types are converted to strings and then decoded from UTF-16LE to UTF-8
/// This is because we need strings to be in the header row.
///
/// # Arguments
/// * `range` - The calamine::Range to get the header names from
///
/// # Returns
/// * Result containing a Vec<String> of header names or an error
fn get_header_names(range: &calamine::Range<calamine::DataType>) -> Result<Vec<String>, ParseFilesError> {
    match range.rows().next() {
        Some(header_row) => {
            let header_names: Vec<String> = header_row
                .iter()
                .map(|cell| match cell {
                    calamine::DataType::String(s) => Ok(decode_text(s)),
                    _ => Ok(decode_text(&cell.to_string())),
                })
                .collect::<Result<Vec<String>, calamine::Error>>()?;
            Ok(header_names)
        }
        // No header row found, return an error then.
        None => Err(ParseFilesError::NoHeadersFound.into()),
    }
}

/// Parse a sheet into a polars::prelude::DataFrame
/// The first row is expected to be the header row.
///
/// # Arguments
/// * `range` - The calamine::Range to parse
///
/// # Returns
/// * Result containing a polars::prelude::DataFrame or an error
#[allow(dead_code)]
pub fn parse_sheet(range: &calamine::Range<calamine::DataType>) -> Result<polars::prelude::DataFrame, ParseFilesError> {
    // Get the header names
    let header_names = get_header_names(&range)?;

    // Iterate through columns and collect data into the Vec<Vec<polars::prelude::AnyValue>>
    let data: Vec<Vec<AnyValue>> = (0..header_names.len())
        .map(|col_idx| {
            range
                .rows()
                .skip(1)
                .map(|row| match row.get(col_idx) {
                    Some(cell) => data_type_to_any_value(&cell),
                    _ => AnyValue::Null,
                })
                .collect()
        })
        .collect();

    // Create series using the correct header names
    let series: Vec<polars::prelude::Series> = header_names
        .iter()
        .zip(data.into_iter())
        .map(|(name, value)| polars::prelude::Series::new(name.as_str(), value))
        .collect();

    let df = polars::prelude::DataFrame::new(series)?;

    Ok(df)
}

/// Parse a .xls file into a polars::prelude::DataFrame
/// The first row is expected to be the header row.
/// The first sheet is parsed.
/// If there are multiple sheets, an error is returned.
/// If the sheet is not found, an error is returned.
/// If the sheet is empty, an error is returned.
/// If the sheet contains only the header row, an empty DataFrame is returned.
///
/// # Arguments
/// * `file_path` - The path to the .xls file
///
/// # Returns
/// * Result containing a polars::prelude::DataFrame or an error
#[allow(dead_code)]
pub fn parse_xls_file_tms(file_path: &str) -> Result<polars::prelude::DataFrame, ParseFilesError> {
    let mut workbook: Xls<_> = calamine::open_workbook(file_path)?;
    let sheet_names = workbook.sheet_names();

    if sheet_names.len() != 1 {
        return Err(ParseFilesError::InvalidSheetCount((1, sheet_names.len() as i32)).into());
    }

    let range = match workbook.worksheet_range(&sheet_names[0]) {
        Some(Ok(range)) => range,
        Some(Err(e)) => return Err(e.into()),
        None => return Err(calamine::Error::Msg("Sheet not found").into()),
    };

    let df = parse_sheet(&range)?;

    Ok(df)
}

pub fn create_job_rows(cl_view_path: &str, shipper_site_path: &str, mode: DispoMode) -> Result<Vec<JobRow>, ParseFilesError> {
    let mut cl_view = parse_xls_file_tms(cl_view_path)?;
    let mut shipper_site = parse_xls_file_tms(shipper_site_path)?;
    let column_mapping = ColumnMapping::new(mode);

    // Drop the old DataFrames and replace it with a new one containg only the wanted columns
    cl_view = select_columns_cl_view(&cl_view, &column_mapping)?;
    shipper_site = select_columns_shipper_site(&shipper_site, &column_mapping)?;

    // Join the DataFrames to create a DataFrame containing all wanted columns.
    let joined = cl_view.inner_join(&shipper_site, ["Load #"], ["Load #"])?;
    // We don't need the old DataFrames anymore
    drop(cl_view);
    drop(shipper_site);

    // Create a Vec<JobRow> from the DataFrame
    let rows = JobRow::from_dataframe(&joined, mode)?;

    Ok(rows)
}

/// Select only the wanted columns from a DataFrame containing the CL View
///
/// # Arguments
/// * `df` - The DataFrame to select the columns from
/// * `mapping` - The ColumnMapping to use
///
/// # Returns
/// * Result containing a DataFrame with only the wanted columns or an error
fn select_columns_cl_view(
    df: &polars::prelude::DataFrame,
    mapping: &ColumnMapping,
) -> Result<polars::prelude::DataFrame, polars::prelude::PolarsError> {
    Ok(df.select([
        mapping.job_number,
        mapping.quantity,
        mapping.equipment_codes,
        mapping.target_early,
        mapping.target_late,
        mapping.info,
        mapping.name,
        mapping.address,
        mapping.city,
        mapping.state,
        mapping.postal_code,
        mapping.country,
    ])?)
}

/// Select only the wanted columns from a DataFrame containing the Shipper Site
///
/// # Arguments
/// * `df` - The DataFrame to select the columns from
/// * `mapping` - The ColumnMapping to use
///
/// # Returns
/// * Result containing a DataFrame with only the wanted columns or an error
fn select_columns_shipper_site(
    df: &polars::prelude::DataFrame,
    mapping: &ColumnMapping,
) -> Result<polars::prelude::DataFrame, polars::prelude::PolarsError> {
    Ok(df.select([mapping.job_number, mapping.hawb, mapping.temperature_range])?)
}
