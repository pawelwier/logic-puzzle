use std::vec;
use puzzle::PuzzleSchema;

pub mod puzzle;

// TODO: break into separate files, use defined structs

#[derive(Clone, Debug, PartialEq)]
enum Field {
    Blank,
    Filled
}

impl Field {
    fn display(&self) -> String {
        match self {
            Field::Blank => { " - ".to_owned() },
            Field::Filled => { " X ".to_owned() },
        }
    }

    fn is_filled(&self) -> bool {
        self == &Field::Filled
    }
}

enum RowType {
    Empty,
    One,
    Two,
    ThreeAndMore
}

struct RowResults {
    row_size: usize,
    props: Vec<usize>
}

impl RowResults {
    fn new(row_size: usize, props: Vec<usize>) -> RowResults {
        Self { row_size, props }
    }

    fn get_row_variants(&mut self) -> Vec<Vec<Field>> {
        get_variants(&self.props, self.row_size)
    }

    fn print_results(&mut self) -> () {
        let results = self.get_row_variants();

        results.into_iter().for_each(|i| {
            i.into_iter().for_each(|i| {
                print!("{}", i.display());
            });
            print!("\n");
        });
    }
}
fn get_props_type(props: &Vec<usize>) -> RowType {
    let prop_count = props.len();

    match prop_count {
        0 => RowType::Empty,
        1 => RowType::One,
        2 => RowType::Two,
        _ => RowType::ThreeAndMore,
    }
}

fn get_empty_row(row_size: usize) -> Vec<Vec<Field>> {
    let empty_option = vec![Field::Blank; row_size as usize];
    vec![empty_option; 1]
}

fn get_variants_one_prop(props: &Vec<usize>, row_size: usize) -> Vec<Vec<Field>> {
    let single_prop = props[0];
    let empty_spaces = row_size - single_prop as usize;
    let mut variants: Vec<Vec<Field>> = vec![];

    for i in 0..=empty_spaces {
        let mut option: Vec<Vec<Field>> = vec![];
        option.push(vec![Field::Blank; i]);
        option.push(vec![Field::Filled; single_prop as usize]);
        option.push(vec![Field::Blank; (empty_spaces - i) as usize]);

        variants.push(option.into_iter().flatten().collect());
    };

    variants
}

fn get_variants_two_props(props: &Vec<usize>, row_size: usize) -> Vec<Vec<Field>> {
    let (first, second) = (props[0], props[1]);
    let mut variants: Vec<Vec<Field>> = vec![];
    let first_prop_max_index = row_size - (first + second + 1);
    let second_prop_max_index = row_size - second;

    for first_index in 0..=first_prop_max_index {
        let second_index_min = first_index + first + 1;
        for second_index in second_index_min..=second_prop_max_index {
            let mut option: Vec<Vec<Field>> = vec![];
            option.push(vec![Field::Blank; first_index as usize]);
            option.push(vec![Field::Filled; first as usize]);
            option.push(vec![Field::Blank; (second_index - (first_index + first)) as usize]);
            option.push(vec![Field::Filled; second as usize]);
            option.push(vec![Field::Blank; (row_size - (second_index + second)) as usize]);
    
            variants.push(option.into_iter().flatten().collect());
        }
    }

    variants
}

fn get_variants_three_and_more_props(_props: &Vec<usize>, _row_size: usize) -> Vec<Vec<Field>> {
    let variants: Vec<Vec<Field>> = vec![];

    // TODO: add logic (with recurrent get_variants_two_props?)

    variants
}

fn get_variants(props: &Vec<usize>, row_size: usize) -> Vec<Vec<Field>> {
    // TODO: if props + spaces sum > row_size: throw Error
    let props_type = get_props_type(props);

    match props_type {
        RowType::Empty => { get_empty_row(row_size) },
        RowType::One => { get_variants_one_prop(props, row_size) },
        RowType::Two => { get_variants_two_props(props, row_size) },
        RowType::ThreeAndMore => { get_variants_three_and_more_props(props, row_size) }
    }
}

fn main() {
    // TODO: if sums are not equal, throw Error
    let (row_props, column_props): (Vec<Vec<usize>>, Vec<Vec<usize>>) = (
        vec![
            vec![2, 2],
            vec![3, 1],
            vec![1, 4],
            vec![1, 3],
        ], 
        vec![
            vec![1, 2],
            vec![2],
            vec![3],
            vec![3],
            vec![1, 2],
            vec![3],
        ]
    );

    let puzzle_schema: PuzzleSchema = PuzzleSchema::new(&column_props, &row_props);
    let (row_size, column_size) = puzzle_schema.get_size();

    let result_rows: &mut Vec<RowResults> = &mut vec![];
    let solution_options: &mut Vec<Vec<Vec<Field>>> = &mut vec![];
    let solution_option: &mut Vec<Vec<Field>> = &mut vec![];
    let i: i32 = -1; // TODO: fix, start with i: usize = 0
    
    let result_columns: &mut Vec<RowResults> = &mut vec![];
    let column_options: &mut Vec<Vec<Vec<Field>>> = &mut vec![];
    let column_option: &mut Vec<Vec<Field>> = &mut vec![];
    let i_col: i32 = -1; // TODO: fix, start with i: usize = 0

    for row in row_props.clone().iter() {
        let results = RowResults::new(row_size, row.to_owned());
        result_rows.push(results);
    }

    for column in column_props.iter() {
        let results = RowResults::new(column_size, column.to_owned());
        result_columns.push(results);
    }

    fn add_variants(
        data_rows: &mut Vec<RowResults>,
        mut i: i32,
        size: usize,
        options: &mut Vec<Vec<Vec<Field>>>,
        option: &mut Vec<Vec<Field>>,
    ) -> () {
        i += 1;

        if i >= size as i32 { return; }
        
        let rows: Vec<Vec<Field>> = data_rows[i as usize].get_row_variants();

        for row in rows {
            if i == (size - 1) as i32 {
                let last_el: Vec<Vec<Field>> = vec![row];
                let mut option_copy: Vec<Vec<Field>> = option.clone();
                option_copy.extend_from_slice(&last_el);
                options.push(option_copy);
            } else {
                option.push(row.clone());
                add_variants(data_rows, i, size, options, option);
                option.pop(); 
            }
        }
    }

    /*
        Line by line, map puzzle fields into vec of bools (Field::Filled = true)
        args:
        [
            [Field::Filled, Field::Filled, Field::Blank, Field::Blank],
            [Field::Filled, Field::Blank, Field::Filled, Field::Filled],
        ]
        return:
        [true, true, false, false, true, false, true, true]
    */
    fn map_puzzle_row_fields(fields: Vec<Vec<Field>>) -> Vec<bool> {
        fields.into_iter().flatten().map(|field| { field.is_filled() }).collect()
    }

    /*
        args:
        [
            [1, 5]
            [2, 6]
            [3, 7]
            [4, 8]
        ]
        return:
        [
            [1, 2, 3, 4],
            [5, 6, 7, 8]
        ]
    */
    fn map_puzzle_columns_to_rows(fields: Vec<Vec<Field>>) -> Vec<Vec<Field>> {
        let col_count = fields[0].len();
        let mut mapped_rows: Vec<Vec<Field>> = vec![];
        for i in 0..col_count {
            let mut mapped_row: Vec<Field> = vec![];
            fields.iter().for_each(|row| { mapped_row.push(row[i].clone()); });
            mapped_rows.push(mapped_row);
        }
        mapped_rows
    }

    fn map_puzzle_column_fields(fields: Vec<Vec<Field>>) -> Vec<bool> {
        map_puzzle_row_fields(map_puzzle_columns_to_rows(fields))
    }

    /* compare two rows of bools, return matching one or None */
    fn get_matching_solution(rows: Vec<bool>, columns: Vec<bool>) -> Option<Vec<bool>> {
        // TODO: allow for multiple correct solutions
        let matching = rows.iter().zip(&columns).filter(|&(row_val, col_val)| row_val == col_val).count();
        if matching == rows.len() {
            Some(rows)
        } else {
            None
        }
    }

    fn map_bools_to_fields(values: Vec<bool>, row_count: usize, column_count: usize) -> Vec<Vec<Field>> {
        // TODO: if row_count * column_count != values, throw Error
        let mut puzzle_schema: Vec<Vec<bool>> = vec![];
            for i in 0..row_count {
                let mapped_row = &values[i * column_count..column_count + i * column_count];
                puzzle_schema.push(mapped_row.to_vec());
            }

        puzzle_schema.iter().map(|row| {
            row.iter().map(|el| { if *el { Field::Filled } else { Field::Blank } }).collect()
        }).collect()
    }

    // TODO: break into 2 functions
    fn display_results(fields: Vec<Vec<Field>>) -> () {
        println!("");
        let _ = fields.iter().for_each(|row| {
            let _ = row.iter().for_each(|el| {
                print!("{}", el.display());
            });
            println!("");
        });
        println!("");
    }

    add_variants(
        result_rows,
        i,
        column_size as usize,
        solution_options,
        solution_option,
    );

    add_variants(
        result_columns,
        i_col,
        row_size as usize,
        column_options,
        column_option,
    );

    for solution in solution_options.iter() {
        for columns in column_options.iter() {

            let matching_solution = get_matching_solution(
                map_puzzle_row_fields(solution.to_vec()),
                map_puzzle_column_fields(columns.to_vec()),
            );

            match matching_solution {
                Some(value) => {
                    let fields = map_bools_to_fields(value, column_size as usize, row_size as usize);
                    display_results(fields);
                },
                None => {}
            }
        }
    }
}
