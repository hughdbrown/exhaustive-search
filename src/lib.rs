use std::cmp::max;

pub struct Item {
    pub value: u64,
    pub weight: usize,
}

pub enum SearchResult {
    Success(u64),
    Failure,
}

fn exhaustive_search_helper(items: &[Item], i: usize, limit_weight: usize, weight: usize, value: u64) -> SearchResult {
    if weight > limit_weight {
        return SearchResult::Failure;
    }
    if i >= items.len() {
        return SearchResult::Success(value);
    }
    let item = &items[i];
    let left: SearchResult = exhaustive_search_helper(items, i + 1, limit_weight, weight, value);
    let right: SearchResult = exhaustive_search_helper(items, i + 1, limit_weight, weight + item.weight, value + item.value);
    match (left, right) {
        (SearchResult::Success(lvalue), SearchResult::Success(rvalue)) => return SearchResult::Success(max(lvalue, rvalue)),
        (SearchResult::Success(lvalue), SearchResult::Failure) => return SearchResult::Success(lvalue),
        (SearchResult::Failure, SearchResult::Success(rvalue)) => return SearchResult::Success(rvalue),
        (SearchResult::Failure, SearchResult::Failure) => return SearchResult::Failure,
    }
}

pub fn exhaustive_search(items: &[Item], limit_weight: usize) -> u64 {
    match exhaustive_search_helper(items, 0, limit_weight, 0, 0) {
        SearchResult::Success(value) => return value,
        SearchResult::Failure => return 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exhaustive_search_1() {
        let items = vec![
            Item{value:1500, weight:1},
            Item{value:2000, weight:3},
            Item{value:3000, weight:4},
        ];
        let value = exhaustive_search(&items, 4);
        assert_eq!(value, 3500);
    }

    #[test]
    fn exhaustive_search_2() {
        let items = vec![
            Item{value:1500, weight:1},
            Item{value:2000, weight:3},
            Item{value:3000, weight:4},
            Item{value:3300, weight:5},
            Item{value:4000, weight:6},
            Item{value:4200, weight:7},
            Item{value:4400, weight:8},
        ];
        let value = exhaustive_search(&items, 34);
        assert_eq!(value, 22400);
        let value = exhaustive_search(&items, 33);
        assert_eq!(value, 20900);
        let value = exhaustive_search(&items, 32);
        assert_eq!(value, 20400);
        let value = exhaustive_search(&items, 31);
        assert_eq!(value, 20400);
        let value = exhaustive_search(&items, 30);
        assert_eq!(value, 19400);
        let value = exhaustive_search(&items, 29);
        assert_eq!(value, 19100);
        let value = exhaustive_search(&items, 28);
        assert_eq!(value, 18400);
        let value = exhaustive_search(&items, 27);
        assert_eq!(value, 18200);
    }
}
