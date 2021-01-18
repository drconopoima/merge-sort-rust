pub fn merge_sort<T: PartialOrd>(mut vector: Vec<T>) -> Vec<T> {
    let input_length = vector.len();
    if input_length <= 1 {
        return vector;
    }
    let half_element = input_length / 2;
    let second_half = vector.split_off(half_element);
    let sorted_2nd_half = merge_sort(second_half);
    let sorted_1st_half = merge_sort(vector);
    merge(sorted_1st_half, sorted_2nd_half)
}

fn merge<T: PartialOrd>(list_1st: Vec<T>, list_2nd: Vec<T>) -> Vec<T> {
    let length = list_1st.len() + list_2nd.len();
    let mut result: Vec<T> = Vec::with_capacity(length);
    // bring halves together, lowest to the front
    let mut list_1st_iter = list_1st.into_iter();
    let mut list_2nd_iter = list_2nd.into_iter();
    let mut list_1st_peek = list_1st_iter.next();
    let mut list_2nd_peek = list_2nd_iter.next();
    loop {
        match (&mut list_1st_peek, &mut list_2nd_peek) {
            (Some(list_1st_val), Some(list_2nd_val)) => {
                if list_1st_val < list_2nd_val {
                    result.push(list_1st_peek.take().unwrap());
                    list_1st_peek = list_1st_iter.next();
                } else {
                    result.push(list_2nd_peek.take().unwrap());
                    list_2nd_peek = list_2nd_iter.next();
                }
            }
            (Some(_), None) => {
                result.push(list_1st_peek.take().unwrap());
                result.extend(list_1st_iter);
                return result;
            }
            (None, Some(_)) => {
                result.push(list_2nd_peek.take().unwrap());
                result.extend(list_2nd_iter);
                return result;
            }
            _ => return result,
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
    #[test]
    fn does_not_overflow() {
        use super::*;
        use rand::{distributions::Uniform, Rng};
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 20);
        let random_vector: Vec<usize> = (0..10000).map(|_| rng.sample(&range)).collect();
        let merge_sorted = random_vector.clone();
        let mut std_sorted = random_vector.clone();
        std_sorted.sort();
        // std_sorted.push(1000);
        let merge_sorted = merge_sort(merge_sorted);
        assert_eq!(merge_sorted, std_sorted)
    }
}
