#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.eq(_second_list) {
        return Comparison::Equal;
    }

    if _first_list.len() == 0 {
        return Comparison::Sublist;
    }

    if _second_list.len() == 0 {
        return Comparison::Superlist;
    }

    let is_superlist = _first_list.windows(_second_list.len()).any(|x| x == _second_list);
    let is_sublist = _second_list.windows(_first_list.len()).any(|x| x == _first_list);

    if is_superlist && _first_list.len() > _second_list.len() {
        return Comparison::Superlist;
    }

    if is_sublist && _second_list.len() > _first_list.len() {
        return Comparison::Sublist;
    }
    Comparison::Unequal
}