pub fn get_prop_sum(props: &Vec<Vec<usize>>) -> usize {
    props.iter().flatten().sum()
}

fn get_prop_min_len(props: Vec<usize>) -> usize {
    let prop_sum: usize = props.iter().sum();
    let space_count: usize = props.len() - 1;
    prop_sum + space_count
}

fn is_props_len_invalid(props: &Vec<usize>, size: usize) -> bool {
    get_prop_min_len(props.to_vec()) > size
}

pub fn is_any_props_len_invalid(props: &Vec<Vec<usize>>, size: usize) -> bool {
    props.iter().any(|el| { is_props_len_invalid(el, size) })
}