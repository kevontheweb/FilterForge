pub mod aa_filter;
pub mod amp;
pub mod utils;
use crate::aa_filter::AntiAliasingFilter;
use crate::amp::AmplifierCircuit;
use std::env;
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

#[test]
fn test_get_closest_value() {
    use crate::utils::get_closest_value;
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
