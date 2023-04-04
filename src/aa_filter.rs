use crate::utils::get_closest_value;
use crate::utils::get_user_input;
use std::f64::consts::PI;
#[derive(Debug)]
pub enum AntiAliasingFilter {
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
    pub fn q_factors(order: u8) -> Vec<f64> {
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

    pub fn component_values(q: Vec<f64>, fc: f64, order: u8) -> Vec<AntiAliasingFilter> {
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
