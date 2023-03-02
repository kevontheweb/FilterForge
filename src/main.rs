use std::{env, f64::consts::PI, io};

extern crate rand;
use rand::Rng;

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
    TopologyBEnhanced {
        // figure 3
        r_f: f64, // selected
        r_g: f64,
        vref_prime: f64,
        r_1: f64, // selected
        r_2: f64,
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
        let offset = vo_zs - (gain * vi_zs);

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
            /*
            (false, true) => {
                let r_f = AmplifierCircuit::get_user_input(
                    "select r_f:\nselect r_f:\n(this may have been suggested by datasheet)",
                );
                let r_g = r_f / (gain - 1.0);
                let vref_prime = offset.abs() / gain;
                let r_1 = AmplifierCircuit::get_user_input("select r_1:");
                let r_2 = (vref_prime * r_1) / (vref - vref_prime);

                AmplifierCircuit::TopologyBEnhanced {
                    r_f,
                    r_g,
                    vref_prime,
                    r_1,
                    r_2,
                }
            }
            */
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
    Butterworth {
        fc: f64,
        order: u8,
        r2: f64,
        c2: f64,
        r1: f64,
        c1: f64,
    },
}

impl AntiAliasingFilter {
    // add code here
    fn q_factors(order: u8) -> Vec<f64> {
        let mut q: Vec<f64> = Vec::new();
        match order {
            2 => {
                q = vec![1.0];
            }
            4 => {
                q = vec![0.541, 1.306];
            }
            6 => {
                q = vec![0.518, 0.707, 1.932];
            }
            8 => {
                q = vec![0.51, 0.601, 0.9, 2.563];
            }
            _ => panic!("order must be 2, 4, 6, or 8"),
        };
        q
    }
    fn component_values(q: Vec<f64>, fc: f64, order: u8) -> Vec<AntiAliasingFilter> {
        // cutoff freq in hz
        // choose c (1nf)
        let mut filters: Vec<AntiAliasingFilter> = Vec::new();
        for q in q.iter() {

            /*
            /* full formulas */
            k = 1.0f64;
            fc = 1.0f64/(2.0f64*PI*(r1*r2*c1*c2).powf(2.0f64));
            q = (r2*r2*c1*c2).powf(0.5f64) / (r1*c1 + r2*c1 + r1*c2*(1.0f64-k));
            /* equivalent component unity gain krc circuit */
            let k=1, r1=m*r, r2 = r, c1 = c, c2 = n*c
            therfore:
            fsf * fc = 1.0f64 / (2.0f64*PI*r*c*(m*n).pow(0.5f64))
            q = (m*n).pow(0.5f64)/(m+1)
            */


            // choose n and calculate m
            // let n = 6.0f64 + (4.0 * q.powf(2.0));
            // let m = ((1.0f64 * (n.powf(2.0f64) - (4.0f64 * n * q.powf(2.0f64))).powf(1.0f64 / 2.0f64)) + n
                // - (2.0f64 * q.powf(2.0f64)))
                // / (2.0f64 * q.powf(2.0f64));
                
           // choose m and calculate n
           /*
           // let m = 1.0f64;
           // let n = q.powf(2.0f64) * (m.powf(2.0f64) + 2.0f64*m + 1.0f64) / m;
            
           let n = 6.0f64 + (4.0 * q.powf(2.0));
           let k = (n / 2.0f64*q.powf(2.0f64)) - 1.0f64;
           let m = k + (k.powf(2.0f64) - 1.0f64).powf(0.5f64);
           println!("m: {}\nn: {}\nk: {}\nq: {}",m,n,k,q);

            // let c1 = 20e-9f64;
            // let c2 = 1e-9f64;
            let mut c2;
            let c1 = get_user_input("select c1 (20nF is typically a good starting choice)");
            loop {
                c2 = get_user_input("select c2 (c1/c2 must be > 4q^2)");
                if &c1 / &c2 < n {
                    println!(
                        "ratio is {}, it should be greater than or equal to {}", (c1 / c2), n);
                    continue;
                } else {
                    break;
                }
            }
            let r2 = 1.0f64 / ((m * n).powf(1.0f64 / 2.0f64) * c2 * 2.0f64 * PI * fc);
            let r1 = m * r2;
            */

            // equal caps and resistors as a ratio aproximation
            let c = get_user_input("select c1 (20nF is typically a good starting choice)");
            let _k =1.0f64;
            let _fsf =1.0f64; 
            let m =( (2.0f64 * q.powf(2.0f64)) - (1.0f64 - 4.0f64*q.powf(2.0f64)) + 1.0f64)/(2.0f64*q.powf(2.0f64)).abs();

            let r = fc*2.0f64*PI*m.powf(0.5f64)*c;
            let r1=m*r;
            let r2 = r;
            let c1 = c;
            let c2 = c;


            let filter = AntiAliasingFilter::Butterworth {
                fc,
                order,
                r2,
                c2,
                r1,
                c1,
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
