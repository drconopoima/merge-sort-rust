use std::vec::IntoIter;

type PeekIter<T> = (Option<T>, IntoIter<T>);

pub fn merge_sort<T: PartialOrd>(mut vector: Vec<T>) -> Vec<T> {
    let input_length = vector.len();
    if input_length <= 1 {
        return vector;
    }
    let half_element = input_length / 2;
    let second_half = vector.split_off(half_element);
    let sorted_2nd_half = merge_sort(second_half);
    let sorted_1st_half = merge_sort(vector);

    let mut list_1st_iter = sorted_1st_half.into_iter();
    let mut list_2nd_iter = sorted_2nd_half.into_iter();
    let list_1st_peek = list_1st_iter.next();
    let list_2nd_peek = list_2nd_iter.next();
    let accumulator: Vec<T> = Vec::with_capacity(input_length);
    merge(
        (list_1st_peek, list_1st_iter),
        (list_2nd_peek, list_2nd_iter),
        accumulator,
    )
}

fn merge<T: PartialOrd>(
    list_1st_tuple: PeekIter<T>,
    mut list_2nd_tuple: PeekIter<T>,
    mut accumulator: Vec<T>,
) -> Vec<T> {
    match (&list_1st_tuple.0, &list_2nd_tuple.0) {
        (Some(_), Some(_)) => {
            let mut list_1st_peek = list_1st_tuple.0;
            let mut list_1st_iter = list_1st_tuple.1;
            let mut list_2nd_peek = list_2nd_tuple.0;
            let mut list_2nd_iter = list_2nd_tuple.1;
            // bring halves together, lowest to the front
            loop {
                match list_1st_peek {
                    Some(ref list_1st_val) => match list_2nd_peek {
                        Some(ref list_2nd_val) => {
                            if list_2nd_val < list_1st_val {
                                accumulator.push(list_2nd_peek.take().unwrap());
                                list_2nd_peek = list_2nd_iter.next();
                            } else {
                                accumulator.push(list_1st_peek.take().unwrap());
                                list_1st_peek = list_1st_iter.next();
                            }
                        }
                        None => {
                            accumulator.push(list_1st_peek.take().unwrap());
                            accumulator.extend(list_1st_iter);
                            return accumulator;
                        }
                    },
                    None => {
                        if let Some(list_2nd_val) = list_2nd_peek {
                            accumulator.push(list_2nd_val);
                        }
                        accumulator.extend(list_2nd_iter);
                        return accumulator;
                    }
                }
            }
        }
        (None, Some(_)) => {
            accumulator.push(list_2nd_tuple.0.take().unwrap());
            accumulator.extend(list_2nd_tuple.1);
            return accumulator;
        }
        (Some(_), None) => {
            accumulator.push(list_2nd_tuple.0.take().unwrap());
            accumulator.extend(list_1st_tuple.1);
            return accumulator;
        }
        _ => accumulator,
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
