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

    pub fn get_size(&self) -> (usize, usize) {
        (self.rows.size, self.columns.size)
    }
}
