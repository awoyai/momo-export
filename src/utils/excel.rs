use rust_xlsxwriter::{Workbook, XlsxError};

pub fn save_data(header:Vec<String>, data: Vec<Vec<String>>, file_name: String) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.write_row(0, 0, header)?;
    sheet.write_row_matrix(1, 0, data)?;
    workbook.save(file_name)
}