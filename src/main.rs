use calamine::*;
use std::io;
fn main() {
    let xlsx_path = format!("{}/workbook.xlsx", env!("CARGO_MANIFEST_DIR"));
    let mut workbook: Xlsx<_> = open_workbook(xlsx_path).unwrap();
    let sheet = workbook.worksheet_range("Sheet1").unwrap();
    let xlsx_iter = sheet.deserialize::<Vec<String>>().unwrap();

    let csv_path = format!("{}/workbook.csv", env!("CARGO_MANIFEST_DIR"));
    let mut reader = csv::Reader::from_path(csv_path).unwrap();
    let csv_iter = reader.deserialize::<Vec<String>>();

    for (xlsx_row, csv_row) in xlsx_iter.zip(csv_iter) {
        let xlsx_row = xlsx_row.unwrap();
        let csv_row = csv_row.unwrap();
        println!("xlsx_row: {xlsx_row:?}");
        println!("csv_row: {csv_row:?}");
        for (xlsx_val, csv_val) in xlsx_row.iter().zip(csv_row.iter()) {
            assert_eq!(xlsx_val, csv_val);
        }
    }
}
