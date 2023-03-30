use std::error::Error;
use url::form_urlencoded;

fn push_number_to_array(number: usize, array: Vec<usize>) -> Vec<usize> {
    array.iter().chain([number].iter()).map(|&x| x).collect()
}

fn is_divisor_larger_than_sqrt_dividend(dividend: usize, divisor: usize) -> bool {
    divisor as f64 >= (dividend as f64).sqrt()
}

pub fn get_prime_number_array(dividend: usize, divisor: usize, result: Vec<usize>) -> Vec<usize> {
    match dividend {
        1 => result,
        _ => match dividend % divisor {
            0 => get_prime_number_array(
                dividend / divisor,
                divisor,
                push_number_to_array(divisor, result),
            ),
            _ => match is_divisor_larger_than_sqrt_dividend(dividend, divisor) {
                true => push_number_to_array(dividend, result),
                false => get_prime_number_array(dividend, divisor + 1, result),
            },
        },
    }
}

pub fn is_param_numeric(params: &str, key_name: &str) -> Result<Option<usize>, Box<dyn Error>> {
    let parsed_params = form_urlencoded::parse(params.as_bytes()).collect::<Vec<_>>();

    let param_value = parsed_params
        .iter()
        .find(|(key, _)| key == key_name)
        .map(|(_, value)| value);

    match param_value {
        Some(value) => match value.parse::<usize>() {
            Ok(number) => Ok(Some(number)),
            Err(_) => Ok(None),
        },
        None => Ok(None),
    }
}
