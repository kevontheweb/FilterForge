use std::cmp::Ordering;
use std::{env, f64::consts::PI, io};

// from 'designing gain and offset in thirty seconds' - Application Report SLOA097 (Texas Instruments)
fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    match args.len() {
        3 => {
        let fc = args[1].parse::<f64>().unwrap();
        let order = args[2].parse::<u8>().unwrap();
        let q = AntiAliasingFilter::q_factors(order);
        let filter = AntiAliasingFilter::component_values(q, fc, order);
        println!("\nfilter values:\n{:?}", filter);
        },
        6 =>{
            let mut values = [0.0; 5];
            for (i, arg) in &mut args[1..6].iter().enumerate() {
                values[i] = match arg.parse::<f64>() {
                    Ok(value) => value,
                    Err(error) => {
                        panic!("\n\ncould not parse input as floats\n{error}\n\nPlease type 5 values (as floats) separated by a space in the following order\nv_ref vo_fs vo_zs vi_fs vi_zs.\n\n");
                    }
                };
            }

        let vref = values[0];
        let vo_fs = values[1];
        let vo_zs = values[2];
        let vi_fs = values[3];
        let vi_zs = values[4];

        let circuit = AmplifierCircuit::calc(vref, vo_fs, vo_zs, vi_fs, vi_zs);
        println!("\ncomponent values:\n{:?}", circuit);

        },
        _ => panic!("\n\nPlease type 5 values (as floats) separated by a space in the following order\nv_ref vo_fs vo_zs vi_fs vi_zs.\n\n to build an offset and scaling circuit\n\nOR\n\n\n\nPlease type 2 values separated by a space in the following order\ncutoff_frequency (hz, as float) order (2, 4, 6, or 8)"),
    }
    if args.len() == 3 {}
}

#[derive(Debug)]
enum AmplifierCircuit {
    TopologyA {
        // positive gain and positive offset
        // figure 1
        // section 3
        r_1: f64,
        r_2: f64,
        r_f: f64,
        r_g: f64,
    },
    TopologyB {
        // positive gain and negative offset
        // figure 2
        // section 4
        r_f: f64,
        r_g: f64,
        r_g2: f64,
        r_g1: f64,
        vref_prime: f64,
        r_1: f64,
    },
    TopologyC {
        // section 5
        // negative gain and positive offset
        r_f: f64, // needs to be selected
        r_g: f64,
        r_2: f64, // same order of magnitude as rf
        r_1: f64, // same order of magnitude as rf
    },
    TopologyD {
        // section 6
        // negative gain and negative offset
        r_f: f64, // needs to be selected
        r_g1: f64,
        r_g2: f64, // same order of magnitude as rf
    },
}

impl AmplifierCircuit {
    fn calc(vref: f64, vo_fs: f64, vo_zs: f64, vi_fs: f64, vi_zs: f64) -> AmplifierCircuit {
        let gain = (vo_fs - vo_zs) / (vi_fs - vi_zs);
        println!("gain: {}", gain);
        let offset = vo_zs - (gain * vi_zs);
        println!("offset: {}", offset);

        match (gain.is_sign_negative(), offset.is_sign_negative()) {
            (false, false) => {
                let r_1 = get_user_input("select r_1:");
                let r_2 = vref * r_1 * gain / offset;
                let r_f =
                    get_user_input("select r_f:\n(this may have been suggested by datasheet)");
                let r_g = r_2 * r_f / (gain * (r_1 + r_2) - r_2);
                AmplifierCircuit::TopologyA { r_1, r_2, r_f, r_g }
            }
            (false, true) => {
                let r_f = get_user_input(
                    "select r_f:\nselect r_f:\n(this may have been suggested by datasheet)",
                );
                let r_g = r_f / (gain - 1.0);
                let r_g2 = r_g / 10.0;
                let r_g1 = r_g - r_g2;
                let vref_prime = offset.abs() * r_g1 / (r_g1 - r_f);
                let r_1 = r_g2 * (vref - vref_prime) / vref_prime;
                AmplifierCircuit::TopologyB {
                    r_f,
                    r_g,
                    r_g2,
                    r_g1,
                    vref_prime,
                    r_1,
                }
            }
            (true, false) => {
                let r_f = get_user_input(
                    "select r_f:\nselect r_f:\n(this may have been suggested by datasheet)",
                );
                let r_g = r_f / gain.abs();
                let r_2 = get_user_input("select r_2:\n(same order of magnitude as r_f)");
                let r_1 = (offset * r_2 * r_g) / ((vref * (r_f + r_g)) - (offset * r_g));
                AmplifierCircuit::TopologyC { r_f, r_g, r_2, r_1 }
            }
            (true, true) => {
                let r_f = get_user_input("select r_f:\nselect r_f:\n");
                let r_g1 = r_f / gain.abs();
                let r_g2 = vref * (r_f / offset.abs());
                AmplifierCircuit::TopologyD { r_f, r_g1, r_g2 }
            }
        }
    }
}

#[derive(Debug)]
enum AntiAliasingFilter {
    // this is an enum so that chebyschev and others can be added. q and fc scaling factors will need to be moved out to properties of the topology, right now it is hardcoded in
    Butterworth {
        fc: f64,
        order: u8,
        r2: f64,
        c2: f64,
        r1: f64,
        c1: f64,
        q: f64,
    },
}

impl AntiAliasingFilter {
    // add code here
    fn q_factors(order: u8) -> Vec<f64> {
        let mut q: Vec<f64> = Vec::new();
        match order {
            2 => q = vec![0.707],
            4 => q = vec![0.541, 1.306],
            6 => q = vec![0.518, 0.707, 1.932],
            8 => q = vec![0.51, 0.601, 0.9, 2.563],
            _ => panic!("order must be 2, 4, 6, or 8"),
        };
        q
    }

    fn component_values(q: Vec<f64>, fc: f64, order: u8) -> Vec<AntiAliasingFilter> {
        // cutoff freq in hz
        // choose c (1nf)
        let mut filters: Vec<AntiAliasingFilter> = Vec::new();
        for q in q.iter() {
            // ==========================================
            // sergio franco
            // ==========================================
            let _c = 1e-9;
            let c = get_user_input("select c1 (20nF is typically a good starting choice)");

            // inputs
            let _m = 1f64; // q is maximised

            // calculations
            let mut n = (4.0f64 * q * q).round();
            let mut k = n / (2.0 * q * q) - 1.0;
            while k <= 1.0 {
                n += 0.1;
                k = n / (2.0 * q * q) - 1.0;
            }

            let m = k + (k * k - 1.0).sqrt();
            let c1 = n * c;
            let c2 = c;
            let r = 1.0 / ((m * n).sqrt() * 2.0 * PI * c * fc);
            let r1 = m * r;
            let r2 = r;

            println!("ideal {:?}", (r1, r2, c1, c2, n, q));
            let r2 = get_closest_value(r2, 'r', 5);
            let r1 = get_closest_value(r1, 'r', 5);
            let c2 = get_closest_value(c2, 'c', 5);
            let c1 = get_closest_value(c1, 'c', 5);
            println!("std {:?}", (r1, r2, c1, c2, n, q));
            let filter = AntiAliasingFilter::Butterworth {
                fc,
                order,
                r2,
                c2,
                r1,
                c1,
                q: *q,
            };
            filters.push(filter);
        }
        filters
    }
}

fn get_user_input(name: &str) -> f64 {
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

fn get_closest_value(value: f64, component_type: char, tolerance: u8) -> f64 {
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

#[test]
fn test_get_closest_value() {
    assert_eq!(get_closest_value(1.0, 'r', 5), 1.0);
    assert_eq!(get_closest_value(0.99, 'r', 5), 1.0);
    assert_eq!(get_closest_value(1.1, 'r', 5), 1.2);
    assert_eq!(get_closest_value(10.0, 'r', 5), 10.0);
    assert_eq!(get_closest_value(9.5, 'r', 5), 9.1);

    assert_eq!(get_closest_value(1234.56, 'r', 5), 1.2e3);
    assert_eq!(get_closest_value(9876543.21, 'r', 5), 9.76e6);
    assert_eq!(get_closest_value(9876.54, 'r', 5), 9.76e3);

    // todo: write tests for capacitors
    // assert_eq!(get_closest_value(0.001e-12, 'c', 5.0), 0.01e-12);
    // assert_eq!(get_closest_value(1.387e-12, 'c', 5.0), );
    // assert_eq!(get_closest_value(9876.54, 'c', 5.0), 9.76e3);
}
