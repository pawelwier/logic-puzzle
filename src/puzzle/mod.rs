use helpers::{
    get_prop_sum, is_any_props_len_invalid
};

pub mod helpers;

enum Dimension {
    Horizontal,
    Vertial
}

pub struct PuzzleSchema {
    rows: PuzzleSchemaDimension,
    columns: PuzzleSchemaDimension
}

struct RowResults {
    row_size: usize,
    props: Vec<usize>
}

struct PuzzleSchemaDimension {
    dimension: Dimension,
    size: usize,
    props: Vec<Vec<usize>>
}

impl PuzzleSchema {
    pub fn new(column_props: &Vec<Vec<usize>>, row_props: &Vec<Vec<usize>>) -> PuzzleSchema {
        Self { 
            rows: PuzzleSchemaDimension {
                size: column_props.len(),
                props: row_props.clone(),
                dimension: Dimension::Horizontal
            },
            columns: PuzzleSchemaDimension {
                size: row_props.len(),
                props: column_props.clone(),
                dimension: Dimension::Vertial
            },
        }
    }

    pub fn is_props_valid(&self) -> bool {
        let prop_sums_match = get_prop_sum(&self.columns.props) == get_prop_sum(&self.rows.props);
        let is_all_row_props_len_valid = !is_any_props_len_invalid(&self.rows.props, self.rows.size);
        let is_all_col_props_len_valid = !is_any_props_len_invalid(&self.columns.props, self.columns.size);

        prop_sums_match && is_all_row_props_len_valid && is_all_col_props_len_valid
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.rows.size, self.columns.size)
    }
}
