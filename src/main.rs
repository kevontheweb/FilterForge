use std::{env, f64::consts::PI, io};

// from 'designing gain and offset in thirty seconds' - Application Report SLOA097 (Texas Instruments)
const STD_R_VALS: [f64; 25] = [
    10.0, 15.0, 22.0, 33.0, 47.0, 68.0, 100.0, 150.0, 220.0, 330.0, 470.0, 680.0, 1000.0, 1500.0,
    2200.0, 3300.0, 4700.0, 6800.0, 10000.0, 15000.0, 22000.0, 33000.0, 47000.0, 68000.0, 100000.0,
];
const STD_C_VALS: [f64; 25] = [
    1e-12, 1.5e-12, 2.2e-12, 3.3e-12, 4.7e-12, 6.8e-12, 10e-12, 15e-12, 22e-12, 33e-12, 47e-12,
    68e-12, 100e-12, 150e-12, 220e-12, 330e-12, 470e-12, 680e-12, 1e-9, 1.5e-9, 2.2e-9, 3.3e-9,
    4.7e-9, 6.8e-9, 10e-9,
];

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
            // GIVES NaNs for some stuff
            // ==========================================
            // sergio franco
            // ==========================================
            let c = 1e-9;
            let c = get_user_input("select c1 (20nF is typically a good starting choice)");

            // inputs
            let m = 1f64; // q is maximised

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
            // WORKS WITH HACK
            // ==========================================
            // choose c's equal
            // ==========================================
            /*
            let n = 1.0 + (4.0 * q.powf(2.0));
            // let mut m = -((n.powf(2.0) - 4.0 * n * q.powf(2.0)).sqrt() - n + 2.0 * q.powf(2.0))
            //     / (2.0 * q.powf(2.0));
            let mut m = (-((n * n) - (4.0 * n * q * q)).sqrt() + n - (2.0 * q * q)) / (2.0 * q * q);

            let n_factor = n;
            let mut c1 = get_user_input("select c1 (20nF is typically a good starting choice)");
            let mut c2 = get_user_input("select c2 (c1/c2 must be > 4q^2)");
            loop {
                // m = -((n.powf(2.0) - 4.0 * n * q.powf(2.0)).sqrt() - n + 2.0 * q.powf(2.0))
                //     / (2.0 * q.powf(2.0));
                //     / (2 * q.powi(2));

                if c1 / c2 <= n_factor && c2 / c1 <= n_factor {
                    println!(
                        "ratio is {}, it should be greater than or equal to {}",
                        (c1 / c2),
                        n
                    );
                    c1 = get_user_input("choose c1 again");
                    c2 = get_user_input("select c2 again");
                    continue;
                } else {
                    break;
                }
            }

            m = (-((n * n) - (4.0 * n * q * q)).sqrt() + n - (2.0 * q * q)) / (2.0 * q * q);
            let hack = 10.0 / q;
            let r2 = 1.0 / ((m * n).powf(0.5) * c2 * 2.0 * PI * hack * fc);
            let r1 = m * r2;
            */
            // BROKEN
            // ==========================================
            // A.1.2. filter components as ratios, gain as 1
            // ==========================================

            // choose m calc n
            /*
            let mut choice = String::new();
            println!("choose m or choose n?");
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");
            let mut m = 0f64;
            let mut n = 0f64;
            match choice.trim() {
                "m" => {
                    m = get_user_input("choose m");
                    n = (q * (m + 1f64)).powf(2f64) / m;
                }
                "n" => {
                    n = get_user_input("choose n");
                    m = -((n.powf(2.0) - 4.0 * n * q.powf(2.0)).sqrt() - n + 2.0 * q.powf(2.0))
                        / (2.0 * q.powf(2.0));
                }
                _ => panic!("choose m or choose n"),
            }
            */
            /*
            let mut m = get_user_input("choose m");
            let mut n = 0f64;
            let mut c = 0f64;
            let mut r = 0f64;
            let mut r1 = 0f64;
            let mut r2 = 0f64;
            let mut c1 = 0f64;
            let mut c2 = 0f64;
            loop {
                n = q.powf(2f64) * (m + 1f64).powf(2f64) / m;
                // choose n calc m
                c = get_user_input("choose c");
                r = 1.0f64 / (2f64 * PI * c * (m * n).powf(2.0f64));
                r1 = m * r;
                r2 = r;
                c1 = c;
                c2 = n * c;

                if r2 > 500e3
                    || r2 < 100f64
                    || r1 > 500e3
                    || r1 < 100f64
                    || c1 < 10e-12
                    || c1 > 100e-6
                    || c2 < 10e-12
                    || c2 > 100e-6
                {
                    println!(
                        "values out of range: r1,r2,c1,c2,n,q={:?}",
                        (r1, r2, c1, c2, n, q)
                    );
                    m = get_user_input("choose m such that n>4*q^2");
                    continue;
                } else {
                    break;
                }
            }
            */

            // BROKEN
            // ==========================================
            // A.1.3. resistors as ratios and caps =
            // ==========================================
            /*
            let m = (-2.0 * q * q - (1.0 - 4.0 * q * q).sqrt() + 1.0) / (2.0 * q * q);
            let k = 1f64;
            // choose n calc m
            let c = get_user_input("choose c");
            let r = 1.0f64 / (2f64 * PI * c * fc * m.powf(0.5f64));
            let r1 = m * r;
            let r2 = r;
            let c1 = c;
            let c2 = c;
            */

            // BROKEN
            // ==========================================
            // A.1.4. equal components
            // ==========================================
            /*
            let c = get_user_input("select c1 (20nF is typically a good starting choice)");
            let r = fc / (c * 2f64 * PI);
            let k = (3f64 * q - 1f64) / q;
            println!("the gain is {k}, attenutaion or amplification is necessary");
            let r1 = r;
            let r2 = r;
            let c1 = c;
            let c2 = c;
            */

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
fn convert_to_standard_values(components: &[f64], standard_vals: &[f64]) -> Vec<f64> {
    components
        .iter()
        .map(|&c| {
            let (val, _) = standard_vals
                .iter()
                .enumerate()
                .min_by_key(|&(_, &x)| ((c - x).abs() * 1e12) as i32)
                .unwrap();
            standard_vals[val]
        })
        .collect()
}
fn check_components(components: &[f64], standard_vals: &[f64]) {
    for &c in components {
        let mut closest_val = f64::INFINITY;
        for &std_val in standard_vals {
            let delta = (c - std_val).abs();
            if delta < closest_val {
                closest_val = delta;
            }
        }
        if closest_val > 1e-12 {
            println!("Error: {} component is not a standard value", c);
            return;
        }
    }
}
