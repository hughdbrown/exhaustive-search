use knapsack_utils::Item;

type SearchResult =  (Vec<usize>, usize, u64);

fn exhaustive_search_helper(
    items: &[Item],
    i: usize,
    limit_weight: usize,
    weight: usize,
    value: u64,
    path: &[usize],
) -> Result<SearchResult, ()>
{
    if weight > limit_weight {
        return Err(()); 
    }
    if i >= items.len() {
        return Ok((path.to_vec(), weight, value));
    }

    let item = &items[i];
    let mut rpath: Vec<usize> = path.to_vec();
    rpath.push(i);
    let left = exhaustive_search_helper(items, i + 1, limit_weight, weight, value, path);
    let right = exhaustive_search_helper(items, i + 1, limit_weight, weight + item.weight, value + item.value, &rpath);
    match (left, right) {
        (Ok(lvalue), Ok(rvalue)) => {
            return Ok(if lvalue.2 > rvalue.2 { lvalue } else { rvalue });
        },
        (Ok(lvalue), Err(())) => return Ok(lvalue),
        (Err(()), Ok(rvalue)) => return Ok(rvalue),
        (Err(()), Err(())) => return Err(()),
    }
}

pub fn exhaustive_search(items: &[Item], limit_weight: usize) -> Result<SearchResult, ()> {
    let path = vec![];
    return exhaustive_search_helper(items, 0, limit_weight, 0, 0, &path);
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
        match exhaustive_search(&items, 4) {
            Ok(value) => assert_eq!(value.2, 3500),
            Err(_) => assert!(false),
        }
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
        let expected_vals: Vec<u64> = vec![22400, 20900, 20400, 20400, 19400, 19100, 18400, 18200];
        for (i, expected_val) in expected_vals.iter().map(|x: &u64| *x).enumerate() {
            match exhaustive_search(&items, 34 - i) {
                Ok(value) => assert_eq!(value.2, expected_val),
                Err(_) => assert!(false),
            }
        }
    }
}
