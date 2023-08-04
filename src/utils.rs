use std::io;

pub fn get_user_input(name: &str) -> f64 {
    println!("{name}");
    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: f64 = value.trim().parse().expect("Please type a number!");
    value
}
// https://en.wikipedia.org/wiki/E_series_of_preferred_numbers#Examples

const E192: [f64; 192] = [
    1.00, 1.01, 1.02, 1.04, 1.05, 1.06, 1.07, 1.09, 1.10, 1.11, 1.13, 1.14, 1.15, 1.17, 1.18, 1.20,
    1.21, 1.23, 1.24, 1.26, 1.27, 1.29, 1.30, 1.32, 1.33, 1.35, 1.37, 1.38, 1.40, 1.42, 1.43, 1.45,
    1.47, 1.49, 1.50, 1.52, 1.54, 1.56, 1.58, 1.60, 1.62, 1.64, 1.65, 1.67, 1.69, 1.72, 1.74, 1.76,
    1.78, 1.80, 1.82, 1.84, 1.87, 1.89, 1.91, 1.93, 1.96, 1.98, 2.00, 2.03, 2.05, 2.08, 2.10, 2.13,
    2.15, 2.18, 2.21, 2.23, 2.26, 2.29, 2.32, 2.34, 2.37, 2.40, 2.43, 2.46, 2.49, 2.52, 2.55, 2.58,
    2.61, 2.64, 2.67, 2.71, 2.74, 2.77, 2.80, 2.84, 2.87, 2.91, 2.94, 2.98, 3.01, 3.05, 3.09, 3.12,
    3.16, 3.20, 3.24, 3.28, 3.32, 3.36, 3.40, 3.44, 3.48, 3.52, 3.57, 3.61, 3.65, 3.70, 3.74, 3.79,
    3.83, 3.88, 3.92, 3.97, 4.02, 4.07, 4.12, 4.17, 4.22, 4.27, 4.32, 4.37, 4.42, 4.48, 4.53, 4.59,
    4.64, 4.70, 4.75, 4.81, 4.87, 4.93, 4.99, 5.05, 5.11, 5.17, 5.23, 5.30, 5.36, 5.42, 5.49, 5.56,
    5.62, 5.69, 5.76, 5.83, 5.90, 5.97, 6.04, 6.12, 6.19, 6.26, 6.34, 6.42, 6.49, 6.57, 6.65, 6.73,
    6.81, 6.90, 6.98, 7.06, 7.15, 7.23, 7.32, 7.41, 7.50, 7.59, 7.68, 7.77, 7.87, 7.96, 8.06, 8.16,
    8.25, 8.35, 8.45, 8.56, 8.66, 8.76, 8.87, 8.98, 9.09, 9.20, 9.31, 9.42, 9.53, 9.65, 9.76, 9.88,
];
const E96: [f64; 96] = [
    1.00, 1.02, 1.05, 1.07, 1.10, 1.13, 1.15, 1.18, 1.21, 1.24, 1.27, 1.30, 1.33, 1.37, 1.40, 1.43,
    1.47, 1.50, 1.54, 1.58, 1.62, 1.65, 1.69, 1.74, 1.78, 1.82, 1.87, 1.91, 1.96, 2.00, 2.05, 2.10,
    2.16, 2.21, 2.26, 2.32, 2.37, 2.43, 2.49, 2.55, 2.61, 2.67, 2.74, 2.80, 2.87, 2.94, 3.01, 3.09,
    3.16, 3.24, 3.32, 3.40, 3.48, 3.57, 3.65, 3.74, 3.83, 3.92, 4.02, 4.12, 4.22, 4.32, 4.42, 4.53,
    4.64, 4.75, 4.87, 4.99, 5.11, 5.23, 5.36, 5.49, 5.62, 5.76, 5.90, 6.04, 6.19, 6.34, 6.49, 6.65,
    6.81, 6.98, 7.15, 7.32, 7.50, 7.68, 7.87, 8.06, 8.25, 8.45, 8.66, 8.87, 9.09, 9.31, 9.53, 9.76,
];
const E48: [f64; 48] = [
    1.00, 1.05, 1.10, 1.15, 1.21, 1.27, 1.33, 1.40, 1.47, 1.54, 1.62, 1.69, 1.78, 1.87, 1.96, 2.05,
    2.15, 2.26, 2.37, 2.49, 2.61, 2.74, 2.87, 3.01, 3.16, 3.32, 3.48, 3.65, 3.83, 4.02, 4.22, 4.42,
    4.64, 4.87, 5.11, 5.36, 5.62, 5.90, 6.19, 6.49, 6.81, 7.15, 7.50, 7.87, 8.25, 8.66, 9.09, 9.53,
];
const E24: [f64; 24] = [
    1.0, 1.1, 1.2, 1.3, 1.5, 1.6, 1.8, 2.0, 2.2, 2.4, 2.7, 3.0, 3.3, 3.6, 3.9, 4.3, 4.7, 5.1, 5.6,
    6.2, 6.8, 7.5, 8.2, 9.1,
];
const E12: [f64; 12] = [1.0, 1.2, 1.5, 1.8, 2.2, 2.7, 3.3, 3.9, 4.7, 5.6, 6.8, 8.2];
const E6: [f64; 6] = [1.0, 1.5, 2.2, 3.3, 4.7, 6.8];

pub fn get_closest_value(value: f64, component_type: char, tolerance: u8) -> f64 {
    let range: &[f64];
    // std ranges should be different for caps and resistors
    match component_type {
        'r' => {
            range = match tolerance {
                1 => &E96[..],
                2 => &E48[..],
                5 => &E24[..],
                10 => &E12[..],
                20 => &E6[..],
                _ => panic!("Unsupported tolerance value"),
            };
        }
        'c' => {
            range = match tolerance {
                1 => &E96[..],
                2 => &E48[..],
                5 => &E24[..],
                10 => &E12[..],
                20 => &E6[..],
                _ => panic!("Unsupported tolerance value"),
            };
        }
        _ => panic!(
            "Unsupported component type (please use 'r' for resistors and 'c' for capacitors)"
        ),
    }

    // to do : make sure order of magnitude ranges lie withing available components for each type (resistor, capacitor)

    // get order of magnitude from original value
    let mut order_of_magnitude = value.log10().floor() as i32;
    let mut scaled_value = value / 10_f64.powi(order_of_magnitude);

    // account for values out of bounds
    let distance: f64 = (range[range.len() - 1] - range[0] * 10_f64) / 2_f64;
    if scaled_value < range[0] {
        if scaled_value - distance > range[0] - distance {
            scaled_value *= 10_f64;
            order_of_magnitude -= 1;
        }
    } else if scaled_value > range[range.len() - 1] {
        if scaled_value + distance > range[range.len() - 1] + distance {
            scaled_value /= 10_f64;
            order_of_magnitude += 1;
        }
    }

    // choose closest value
    let mut closest = range[0];
    let mut min_diff = (scaled_value - closest).abs();

    for &value in range {
        let diff = (scaled_value - value).abs();
        if diff < min_diff {
            closest = value;
            min_diff = diff;
        }
    }
    closest * 10_f64.powi(order_of_magnitude)
}
