pub const TEST_DATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

pub fn sum_invalid_ids_in_range(range: (u64, u64)) -> u64 {
    let mut sum = 0;

    let the_range = range.0..=range.1;
    let order_0 = range.0.ilog10();
    let order_1 = range.1.ilog10();

    let diff = order_1 - order_0;

    let mut upper_range = (split_number(range.0), split_number(range.1));

    let uorder_0 = upper_range.0.0.ilog10();
    let uorder_1 = upper_range.1.0.ilog10();

    let udiff = uorder_1 - uorder_0;

    // if diff != udiff {
    //     let
    // }

    for x in upper_range.0.0..=upper_range.1.0 {
        let candidate = candidate(x);

        if the_range.contains(&candidate) {
            sum += candidate;
        }
    }

    println!("Sum for range {range:?} is {sum}");

    sum
}

pub fn split_number(id: u64) -> (u64, u64) {
    let power_of_ten = id.ilog10() + 1;

    let half_power = if power_of_ten.is_multiple_of(2) {
        10u64.pow(power_of_ten / 2)
    } else {
        10u64.pow(power_of_ten / 2 + 1)
    };

    let left_half = id / half_power;
    let right_half = id % half_power;

    (left_half, right_half)
}

pub fn candidate(half: u64) -> u64 {
    let power_of_ten = half.ilog10() + 1;
    let half_power = 10u64.pow(power_of_ten);

    half + half * half_power
}

pub fn is_invalid_id(id: u64) -> bool {
    let power_of_ten = id.ilog10() + 1;

    // odd numbers can never be invalid
    if !power_of_ten.is_multiple_of(2) {
        return false;
    }

    let half_power = 10u64.pow(power_of_ten / 2);

    let half_digits = id / half_power;

    let reassembled = half_digits + half_digits * half_power;

    id == reassembled
}

pub fn parse_id_ranges(s: &str) -> Vec<(u64, u64)> {
    s.split(',').map(parse_id_range).collect()
}

pub fn parse_id_range(s: &str) -> (u64, u64) {
    let mut items = s.split('-');

    let a = items.next().unwrap();
    let b = items.next().unwrap();

    (a.parse().unwrap(), b.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("11-22" => (11, 22))]
    #[test_case("2121212118-2121212124" => (2121212118, 2121212124))]
    fn test_parse_id_range(s: &str) -> (u64, u64) {
        parse_id_range(s)
    }

    #[test_case("11-22,95-115,998-1012" => vec![(11, 22), (95, 115), (998, 1012)])]
    fn test_parse_id_ranges(s: &str) -> Vec<(u64, u64)> {
        parse_id_ranges(s)
    }

    #[test_case((11, 22) => 33)]
    #[test_case((1698522, 1698528) => 0)]
    #[test_case((38593856, 38593862) => 38593859)]
    fn test_sum_invalid_ids_in_range(range: (u64, u64)) -> u64 {
        sum_invalid_ids_in_range(range)
    }

    #[test_case(11 => true)]
    #[test_case(111 => false)]
    #[test_case(12321 => false)]
    #[test_case(13 => false)]
    #[test_case(22 => true)]
    #[test_case(38593856 => false)]
    #[test_case(38593859 => true)]
    fn test_is_invalid_id(id: u64) -> bool {
        is_invalid_id(id)
    }

    #[test_case(1234 => (12, 34))]
    #[test_case(998 => (9, 98))]
    #[test_case(123123 => (123, 123))]
    fn test_split_number(n: u64) -> (u64, u64) {
        split_number(n)
    }

    #[test_case(12 => 1212)]
    #[test_case(123 => 123123)]
    fn test_candidate(n: u64) -> u64 {
        candidate(n)
    }

    #[test]
    fn example() {
        let ranges = parse_id_ranges(TEST_DATA);

        let sum = ranges
            .into_iter()
            .map(sum_invalid_ids_in_range)
            .sum::<u64>();

        assert_eq!(sum, 1227775554);
    }
}
