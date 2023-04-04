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

const E24_R: [f64; 24] = [
    1.0, 1.1, 1.2, 1.3, 1.5, 1.6, 1.8, 2.0, 2.2, 2.4, 2.7, 3.0, 3.3, 3.6, 3.9, 4.3, 4.7, 5.1, 5.6,
    6.2, 6.8, 7.5, 8.2, 9.1,
];

const E96_R: [f64; 96] = [
    1.00, 1.02, 1.05, 1.07, 1.10, 1.13, 1.15, 1.18, 1.21, 1.24, 1.27, 1.30, 1.33, 1.37, 1.40, 1.43,
    1.47, 1.50, 1.54, 1.58, 1.62, 1.65, 1.69, 1.74, 1.78, 1.82, 1.87, 1.91, 1.96, 2.00, 2.05, 2.10,
    2.16, 2.21, 2.26, 2.32, 2.37, 2.43, 2.49, 2.55, 2.61, 2.67, 2.74, 2.80, 2.87, 2.94, 3.01, 3.09,
    3.16, 3.24, 3.32, 3.40, 3.48, 3.57, 3.65, 3.74, 3.83, 3.92, 4.02, 4.12, 4.22, 4.32, 4.42, 4.53,
    4.64, 4.75, 4.87, 4.99, 5.11, 5.23, 5.36, 5.49, 5.62, 5.76, 5.90, 6.04, 6.19, 6.34, 6.49, 6.65,
    6.81, 6.98, 7.15, 7.32, 7.50, 7.68, 7.87, 8.06, 8.25, 8.45, 8.66, 8.87, 9.09, 9.31, 9.53, 9.76,
];

const E24_C: [f64; 24] = [
    1.0, 1.1, 1.2, 1.3, 1.5, 1.6, 1.8, 2.0, 2.2, 2.4, 2.7, 3.0, 3.3, 3.6, 3.9, 4.3, 4.7, 5.1, 5.6,
    6.2, 6.8, 7.5, 8.2, 9.1,
];

const E96_C: [f64; 96] = [
    1.00, 1.02, 1.05, 1.07, 1.10, 1.13, 1.15, 1.18, 1.21, 1.24, 1.27, 1.30, 1.33, 1.37, 1.40, 1.43,
    1.47, 1.50, 1.54, 1.58, 1.62, 1.65, 1.69, 1.74, 1.78, 1.82, 1.87, 1.91, 1.96, 2.00, 2.05, 2.10,
    2.16, 2.21, 2.26, 2.32, 2.37, 2.43, 2.49, 2.55, 2.61, 2.67, 2.74, 2.80, 2.87, 2.94, 3.01, 3.09,
    3.16, 3.24, 3.32, 3.40, 3.48, 3.57, 3.65, 3.74, 3.83, 3.92, 4.02, 4.12, 4.22, 4.32, 4.42, 4.53,
    4.64, 4.75, 4.87, 4.99, 5.11, 5.23, 5.36, 5.49, 5.62, 5.76, 5.90, 6.04, 6.19, 6.34, 6.49, 6.65,
    6.81, 6.98, 7.15, 7.32, 7.50, 7.68, 7.87, 8.06, 8.25, 8.45, 8.66, 8.87, 9.09, 9.31, 9.53, 9.76,
];

pub fn get_closest_value(value: f64, component_type: char, tolerance: u8) -> f64 {
    let range: &[f64];
    // std ranges should be different for caps and resistors
    match component_type {
        'r' => {
            range = match tolerance {
                1 => &E96_R[..],
                //2.0 => &E48_R[..],
                5 => &E24_R[..],
                _ => panic!("Unsupported tolerance value"),
            };
        }
        'c' => {
            range = match tolerance {
                1 => &E96_C[..],
                //2.0 => &E48_C[..],
                5 => &E24_C[..],
                _ => panic!("Unsupported tolerance value"),
            };
        }
        _ => panic!(
            "Unsupported component type (please use 'r' for resistors and 'c' for capacitors)"
        ),
    }

    let mut order_of_magnitude = value.log10().floor() as i32;
    let mut scaled_value = value / 10_f64.powi(order_of_magnitude);

    // account for values out of bounds
    if scaled_value < range[0] {
        scaled_value *= 10_f64;
        order_of_magnitude += 1;
    } else if scaled_value > range[range.len() - 1] {
        scaled_value /= 10_f64;
        order_of_magnitude -= 1;
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
