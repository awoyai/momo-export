use rust_xlsxwriter::{Workbook, XlsxError};
use std::fs::File;

pub fn save_data(header:Vec<&str>, data: Vec<Vec<String>>, file_name: String) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.write_row(0, 0, header)?;
    sheet.write_row_matrix(1, 0, data)?;
    let file = File::create(file_name)?;
    workbook.save_to_writer(file)
}