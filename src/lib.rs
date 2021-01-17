pub fn merge_sort<T: PartialOrd>(mut vector: Vec<T>) -> Vec<T> {
    let input_length = vector.len();
    if input_length <= 1 {
        return vector;
    }
    let half_element = input_length / 2;
    let mut result: Vec<T> = Vec::with_capacity(input_length);
    let second_half = vector.split_off(half_element);
    let sorted_2nd_half = merge_sort(second_half);
    let sorted_1st_half = merge_sort(vector);

    // bring halves together, lowest to the front
    let mut sorted_1st_half_iter = sorted_1st_half.into_iter();
    let mut sorted_2nd_half_iter = sorted_2nd_half.into_iter();
    let mut sorted_1st_half_peek = sorted_1st_half_iter.next();
    let mut sorted_2nd_half_peek = sorted_2nd_half_iter.next();
    loop {
        match sorted_1st_half_peek {
            Some(ref sorted_1st_half_val) => match sorted_2nd_half_peek {
                Some(ref sorted_2nd_half_val) => {
                    if sorted_2nd_half_val < sorted_1st_half_val {
                        result.push(sorted_2nd_half_peek.take().unwrap());
                        sorted_2nd_half_peek = sorted_2nd_half_iter.next();
                    } else {
                        result.push(sorted_1st_half_peek.take().unwrap());
                        sorted_1st_half_peek = sorted_1st_half_iter.next();
                    }
                }
                None => {
                    result.push(sorted_1st_half_peek.take().unwrap());
                    result.extend(sorted_1st_half_iter);
                    return result;
                }
            },
            None => {
                if let Some(sorted_2nd_half_val) = sorted_2nd_half_peek {
                    result.push(sorted_2nd_half_val);
                }
                result.extend(sorted_2nd_half_iter);
                return result;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sort() {
        use super::*;
        let vector = vec![1, 3, 55, 7, 5, 100, 6, 41, 0, 2, 4];
        let vector = merge_sort(vector);
        assert_eq!(vector, vec![0, 1, 2, 3, 4, 5, 6, 7, 41, 55, 100]);
    }
    #[test]
    fn random_sort() {
        use super::*;
        use rand::{distributions::Uniform, Rng};
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 20);
        let random_vector: Vec<usize> = (0..100).map(|_| rng.sample(&range)).collect();
        let merge_sorted = random_vector.clone();
        let mut std_sorted = random_vector.clone();
        std_sorted.sort();
        // std_sorted.push(1000);
        let merge_sorted = merge_sort(merge_sorted);
        assert_eq!(merge_sorted, std_sorted)
    }
}
