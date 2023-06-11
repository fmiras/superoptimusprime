pub fn product<T: Clone>(elements: &Vec<T>, times: usize) -> Vec<Vec<T>> {
    if times == 0 {
        return vec![vec![]];
    }

    let mut result = Vec::new();
    for element in elements {
        let mut sub_product = product::<T>(elements, times - 1);
        for sub_product_element in &mut sub_product {
            sub_product_element.push(element.clone());
        }
        result.extend(sub_product);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_product_2x2() {
        let result = product::<i32>(&vec![1, 2], 2);
        assert_eq!(result.len(), 4);
        assert!(result.contains(&vec![1, 1]));
        assert!(result.contains(&vec![1, 2]));
        assert!(result.contains(&vec![2, 1]));
        assert!(result.contains(&vec![2, 2]));
    }

    #[test]
    fn can_product_with_3x3() {
        let result = product::<&str>(&vec!["LOAD", "SWAP", "INC"], 3);
        assert_eq!(result.len(), 27);
        assert!(result.contains(&vec!["LOAD", "LOAD", "LOAD"]));
        assert!(result.contains(&vec!["LOAD", "LOAD", "SWAP"]));
        assert!(result.contains(&vec!["LOAD", "LOAD", "INC"]));
        assert!(result.contains(&vec!["LOAD", "SWAP", "LOAD"]));
        assert!(result.contains(&vec!["LOAD", "SWAP", "SWAP"]));
        assert!(result.contains(&vec!["LOAD", "SWAP", "INC"]));
        assert!(result.contains(&vec!["LOAD", "INC", "LOAD"]));
        assert!(result.contains(&vec!["LOAD", "INC", "SWAP"]));
        assert!(result.contains(&vec!["LOAD", "INC", "INC"]));
        assert!(result.contains(&vec!["SWAP", "LOAD", "LOAD"]));
        assert!(result.contains(&vec!["SWAP", "LOAD", "SWAP"]));
        assert!(result.contains(&vec!["SWAP", "LOAD", "INC"]));
        assert!(result.contains(&vec!["SWAP", "SWAP", "LOAD"]));
        assert!(result.contains(&vec!["SWAP", "SWAP", "SWAP"]));
        assert!(result.contains(&vec!["SWAP", "SWAP", "INC"]));
        assert!(result.contains(&vec!["SWAP", "INC", "LOAD"]));
        assert!(result.contains(&vec!["SWAP", "INC", "SWAP"]));
        assert!(result.contains(&vec!["SWAP", "INC", "INC"]));
        assert!(result.contains(&vec!["INC", "LOAD", "LOAD"]));
        assert!(result.contains(&vec!["INC", "LOAD", "SWAP"]));
        assert!(result.contains(&vec!["INC", "LOAD", "INC"]));
        assert!(result.contains(&vec!["INC", "SWAP", "LOAD"]));
        assert!(result.contains(&vec!["INC", "SWAP", "SWAP"]));
        assert!(result.contains(&vec!["INC", "SWAP", "INC"]));
        assert!(result.contains(&vec!["INC", "INC", "LOAD"]));
        assert!(result.contains(&vec!["INC", "INC", "SWAP"]));
        assert!(result.contains(&vec!["INC", "INC", "INC"]));
    }

    #[test]
    fn can_product_with_4x2() {
        let result = product::<&str>(&vec!["LOAD", "SWAP", "XOR", "INC"], 2);
        assert_eq!(result.len(), 16);
        assert!(result.contains(&vec!["LOAD", "LOAD"]));
        assert!(result.contains(&vec!["LOAD", "SWAP"]));
        assert!(result.contains(&vec!["LOAD", "XOR"]));
        assert!(result.contains(&vec!["LOAD", "INC"]));
        assert!(result.contains(&vec!["SWAP", "LOAD"]));
        assert!(result.contains(&vec!["SWAP", "SWAP"]));
        assert!(result.contains(&vec!["SWAP", "XOR"]));
        assert!(result.contains(&vec!["SWAP", "INC"]));
        assert!(result.contains(&vec!["XOR", "LOAD"]));
        assert!(result.contains(&vec!["XOR", "SWAP"]));
        assert!(result.contains(&vec!["XOR", "XOR"]));
        assert!(result.contains(&vec!["XOR", "INC"]));
        assert!(result.contains(&vec!["INC", "LOAD"]));
        assert!(result.contains(&vec!["INC", "SWAP"]));
        assert!(result.contains(&vec!["INC", "XOR"]));
        assert!(result.contains(&vec!["INC", "INC"]));
    }
}
