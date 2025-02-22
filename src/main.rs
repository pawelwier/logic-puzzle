use std::vec;

#[derive(Clone)]
enum Field {
    Blank,
    Filled
}

impl Field {
    fn display(&self) -> String {
        match self {
            Field::Blank => { "-".to_owned() },
            Field::Filled => { "X".to_owned() },
        }
    }
}

enum RowType {
    Empty,
    One,
    Two,
    ThreeAndMore
}

enum Dimension {
    Horizontal,
    Vertial
}

struct RowResults {
    row_size: i32,
    props: Vec<i32>
}

struct PuzzleSchemaDimension {
    dimension: Dimension,
    row_size: i32,
    prop_rows: Vec<Vec<i32>>
}

struct PuzzleSchema {
    rows: PuzzleSchemaDimension,
    columns: PuzzleSchemaDimension
}

impl RowResults {
    fn new(row_size: i32, props: Vec<i32>) -> RowResults {
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

impl PuzzleSchemaDimension {
    fn new(row_size: i32, prop_rows: Vec<Vec<i32>>, dimension: Dimension) -> PuzzleSchemaDimension {
        Self { row_size, prop_rows, dimension }
    }
}

impl PuzzleSchema {
    fn new(rows: PuzzleSchemaDimension, columns: PuzzleSchemaDimension) -> PuzzleSchema {
        Self { rows, columns }
    }
}

fn get_props_type(props: &Vec<i32>) -> RowType {
    let prop_count = props.len();

    match prop_count {
        0 => RowType::Empty,
        1 => RowType::One,
        2 => RowType::Two,
        _ => RowType::ThreeAndMore,
    }
}

fn get_empty_row(row_size: i32) -> Vec<Vec<Field>> {
    let empty_option = vec![Field::Blank; row_size as usize];
    vec![empty_option; 1]
}

fn get_variants_one_prop(props: &Vec<i32>, row_size: i32) -> Vec<Vec<Field>> {
    let single_prop = props[0];
    let empty_spaces = row_size - single_prop;
    let mut variants: Vec<Vec<Field>> = vec![];

    for i in 0..=empty_spaces {
        let mut option: Vec<Vec<Field>> = vec![];
        option.push(vec![Field::Blank; i as usize]);
        option.push(vec![Field::Filled; single_prop as usize]);
        option.push(vec![Field::Blank; (empty_spaces - i) as usize]);

        variants.push(option.into_iter().flatten().collect());
    };

    variants
}

fn get_variants_two_props(props: &Vec<i32>, row_size: i32) -> Vec<Vec<Field>> {
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

fn get_variants_three_and_more_props(_props: &Vec<i32>, _row_size: i32) -> Vec<Vec<Field>> {
    let variants: Vec<Vec<Field>> = vec![];

    // TODO: add logic (with recurrent get_variants_two_props?)

    variants
}

fn get_variants(props: &Vec<i32>, row_size: i32) -> Vec<Vec<Field>> {
    // TODO: if props + spaces sum > row_size: throw Error
    let props_type = get_props_type(props);

    match props_type {
        RowType::Empty => { get_empty_row(row_size) },
        RowType::One => { get_variants_one_prop(props, row_size) },
        RowType::Two => { get_variants_two_props(props, row_size) },
        RowType::ThreeAndMore => { get_variants_three_and_more_props(props, row_size) }
    }
}

fn _test_sample_data() -> () {
        let row_size: i32 = 10;
    // let props_empty: Vec<i32> = vec![];
    // let props_single_1: Vec<i32> = vec![2];
    // let props_single_2: Vec<i32> = vec![8];
    let props_two_1: Vec<i32> = vec![2, 4];
    let props_two_2: Vec<i32> = vec![1, 6];


    // let mut results_empty = RowResults::new(row_size, props_empty);
    // let mut results_single_1 = RowResults::new(row_size, props_single_1);
    // let mut results_single_2 = RowResults::new(row_size, props_single_2);
    // let mut results_single_2 = RowResults::new(row_size, props_single_2);
    let mut results_two_1 = RowResults::new(row_size, props_two_1);
    let mut results_two_2 = RowResults::new(row_size, props_two_2);

    // results_empty.print_results();
    // println!("");
    // results_single_1.print_results();
    // println!("");
    // results_single_2.print_results();

    results_two_1.print_results();
    println!("");
    results_two_2.print_results();
}

fn main() {
    let (row_size, column_size) = (6, 4);

    let row_props: Vec<Vec<i32>> = vec![
        vec![2, 1],
        vec![3, 1],
        vec![1, 4],
        vec![5],
    ];

    let column_props: Vec<Vec<i32>> = vec![
        vec![1, 2],
        vec![2, 1],
        vec![3],
        vec![3],
        vec![2],
        vec![3],
    ];

    let result_rows: &mut Vec<RowResults> = &mut vec![];
    let solution_options: &mut Vec<Vec<Vec<Field>>> = &mut vec![];
    let solution_option: &mut Vec<Vec<Field>> = &mut vec![];
    let i: i32 = -1; // TODO: fix, start with i: usize = 0 

    for row in row_props.iter() {
        let results = RowResults::new(row_size, row.to_owned());
        result_rows.push(results);
    }

    fn add_variants(
        result_rows: &mut Vec<RowResults>,
        mut i: i32,
        column_size: usize,
        options: &mut Vec<Vec<Vec<Field>>>,
        option: &mut Vec<Vec<Field>>,
    ) -> () {
        i += 1;

        if i >= column_size as i32 { return; }
        
        let rows: Vec<Vec<Field>> = result_rows[i as usize].get_row_variants();

        for row in rows {
            if i == (column_size - 1) as i32 {
                let last_el: Vec<Vec<Field>> = vec![row];
                let mut option_copy: Vec<Vec<Field>> = option.clone();
                option_copy.extend_from_slice(&last_el);
                options.push(option_copy);
            } else {
                option.push(row.clone());
                add_variants(result_rows, i, column_size, options, option);
                option.pop(); 
            }
        }
    }

    add_variants(
        result_rows,
        i,
        column_size,
        solution_options,
        solution_option,
    );

    for op in solution_options.iter() {
        for row in op {
            for f in row {
                print!("{}", f.display());
            }
            println!("");
        }
        println!("");
    }

    println!("{} options", solution_options.len());
}
