pub struct Alignment {}
pub struct ColumnProperty {}

pub struct Table {
    row_num: usize,
    column_num: usize,
    column_properties: Vec<ColumnProperty>,
    rows: Vec<Vec<String>>
}

impl Table {
    pub fn new() -> Self {
        Self {
            row_num: 0,
            column_num: 0,
            column_properties: vec![],
            rows: vec![]
        }
    }

    pub fn add_column<T: Into<String>>(&mut self, nane: T, align: Alignment) {

    }
}
