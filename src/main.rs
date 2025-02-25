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

        variants.push(
            option
                .into_iter()
                .flatten()
                .collect()
        );
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

// TODO: fix and refactor
// 1. additional spaces can be divided between slots if > 1 
// 2. break into separate functions
fn get_variants_three_and_more_props(props: &Vec<usize>, row_size: usize) -> Vec<Vec<Field>> {
    let mut variants: Vec<Vec<Field>> = vec![];
    let prop_sum: usize = props.iter().sum();
    let spaces_min_sum = props.len() - 1; // at least one field between each group of props
    let first_prop_max_index = row_size - prop_sum - spaces_min_sum;

    for first_index in 0..=first_prop_max_index {
        let mut spaces = props.len();
        if first_index == first_prop_max_index { spaces = 1 };
        for space in 0..spaces {
            let mut remaining_spaces = first_prop_max_index - first_index;
            let mut option: Vec<Vec<Field>> = vec![];
            option.push(vec![Field::Blank; first_index]);
            for (i, prop) in props.iter().enumerate() {
                option.push(vec![Field::Filled; *prop]);
                if i < props.len() -1 {
                    let mut add_value = 0;
                    if i == space {
                        add_value += remaining_spaces;  
                        remaining_spaces = 0;
                    };

                    option.push(vec![Field::Blank; 1 + add_value]);
                }
            };
            option.push(vec![Field::Blank; remaining_spaces]);
            variants.push(option.into_iter().flatten().collect());
        };
    }

    variants
}

// fn try_sample() -> () {
//     let row_size: usize = 15;
//     let props_three: Vec<usize> = vec![1, 3, 2, 3];
//     let mut results_three = RowResults::new(row_size, props_three);
//     results_three.print_results();
// }

fn get_variants(props: &Vec<usize>, row_size: usize) -> Vec<Vec<Field>> {
    let props_type = get_props_type(props);

    match props_type {
        RowType::Empty => { get_empty_row(row_size) },
        RowType::One => { get_variants_one_prop(props, row_size) },
        RowType::Two => { get_variants_two_props(props, row_size) },
        RowType::ThreeAndMore => { get_variants_three_and_more_props(props, row_size) }
    }
}

fn main() {
    // try_sample();

    let (row_props, column_props): (Vec<Vec<usize>>, Vec<Vec<usize>>) = (
        vec![
            vec![5],
            vec![4, 3],
            vec![3, 5],
            vec![1, 3, 2]
        ], 
        vec![
            vec![1],
            vec![2],
            vec![4],
            vec![2],
            vec![2, 1],
            vec![1, 2],
            vec![4],
            vec![2],
            vec![3],
            vec![2],
        ]
    );
    let puzzle_schema: PuzzleSchema = PuzzleSchema::new(&column_props, &row_props);
    if !puzzle_schema.is_props_valid() {
        panic!("Invalid props.")
    }
    let (row_size, column_size) = puzzle_schema.get_size();

    let result_rows: &mut Vec<RowResults> = &mut vec![];
    let solution_options: &mut Vec<Vec<Vec<Field>>> = &mut vec![];
    let solution_option: &mut Vec<Vec<Field>> = &mut vec![];
    let i: usize = 0;
    
    let result_columns: &mut Vec<RowResults> = &mut vec![];
    let column_options: &mut Vec<Vec<Vec<Field>>> = &mut vec![];
    let column_option: &mut Vec<Vec<Field>> = &mut vec![];
    let i_col: usize = 0;

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
        mut i: usize,
        size: usize,
        options: &mut Vec<Vec<Vec<Field>>>,
        option: &mut Vec<Vec<Field>>,
        update_index: bool,
    ) -> () {
        if update_index { i += 1; }

        if i >= size { return; }
        
        let rows: Vec<Vec<Field>> = data_rows[i as usize].get_row_variants();

        for row in rows {
            if i == size - 1 {
                let last_el: Vec<Vec<Field>> = vec![row];
                let mut option_copy: Vec<Vec<Field>> = option.clone();
                option_copy.extend_from_slice(&last_el);
                options.push(option_copy);
            } else {
                option.push(row.clone());
                add_variants(data_rows, i, size, options, option, true);
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
        let matching = rows
            .iter()
            .zip(&columns)
            .filter(|&(row_val, col_val)| row_val == col_val)
            .count();

        if matching == rows.len() {
            Some(rows)
        } else {
            None
        }
    }

    fn map_bools_to_fields(values: Vec<bool>, row_count: usize, column_count: usize) -> Vec<Vec<Field>> {
        if row_count * column_count != values.len() {
            panic!("Error mapping values to fields. Sums do not match")
        }

        let mut puzzle_schema: Vec<Vec<bool>> = vec![];
            for i in 0..row_count {
                let mapped_row = &values[i * column_count..column_count + i * column_count];
                puzzle_schema.push(mapped_row.to_vec());
            }

        puzzle_schema.iter().map(|row| {
            row.iter().map(|el| { if *el { Field::Filled } else { Field::Blank } }).collect()
        }).collect()
    }

    fn display_results(fields: Vec<Vec<Field>>) -> () {
        println!("");
        let _ = fields
            .iter()
            .for_each(|row| {
                let _ = row
                    .iter()
                    .for_each(|el| {
                        print!("{}", el.display());
                    }
                );
                println!("");
            }
        );
        println!("");
    }

    add_variants(
        result_rows,
        i,
        column_size as usize,
        solution_options,
        solution_option,
        false
    );

    add_variants(
        result_columns,
        i_col,
        row_size as usize,
        column_options,
        column_option,
        false
    );

    println!("\n");
    println!("column variants: {}", column_options.len());
    println!("row variants: {}", solution_options.len());
    println!("total options: {}", column_options.len() * solution_options.len());


    for solution in solution_options.iter() {
        for columns in column_options.iter() {

            let matching_solution = get_matching_solution(
                map_puzzle_row_fields(solution.to_vec()),
                map_puzzle_column_fields(columns.to_vec()),
            );

            match matching_solution {
                Some(value) => {
                    let fields = map_bools_to_fields(
                        value,
                        column_size as usize,
                        row_size as usize
                    );
                    display_results(fields);
                },
                None => {}
            }
        }
    }
}
