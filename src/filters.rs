// Filter design in thirty seconds - sloa93 app note - texas instruments
// todo:
// - decide whether to do this from scratch or use the aa_filter.rs stuff
// - the app note only does this for 2nd order filters, so maybe doing it myself will be better
//
use crate::utils::get_closest_value;
use crate::utils::get_user_input;
use std::f64::consts::PI;

//single supply filters
#[derive(Debug)]
pub struct LowpassFilter {
    c1: f64,
    c2: f64,
    r1: f64,
    r2: f64,
    c_out: f64,
    c_in: f64,
    // 100k resistors after c_in from signal to supply and from signal to ground
    // figure 8
}
impl LowpassFilter {
    pub fn component_values(f: f64) -> LowpassFilter {
        let c1 = get_user_input("select c1");
        let c2 = c1 * 2f64;
        let r1 = 1f64 / (f * c1 * 2f64 * 2f64.sqrt());
        let r2 = r1;
        let c_in = 100f64 * c1;
        let c_out = c_in;
        println!("ideal {:?}", (c1, c2, r1, r2, c_in, c_out));

        let c1 = get_closest_value(c1, 'c', 5);
        let c2 = get_closest_value(c2, 'c', 5);
        let r1 = get_closest_value(r1, 'r', 5);
        let r2 = get_closest_value(r2, 'r', 5);
        let c_in = get_closest_value(c_in, 'c', 5);
        let c_out = get_closest_value(c_out, 'c', 5);
        println!("std {:?}", (c1, c2, r1, r2, c_in, c_out));

        let filter = LowpassFilter {
            c1,
            c2,
            r1,
            r2,
            c_out,
            c_in,
        };
        filter
    }
}

#[derive(Debug)]
pub struct HighpassFilter {
    c1: f64,
    c2: f64,
    r1: f64,
    r2: f64,
    c_out: f64,
    c_in: f64,
    // figure 10
}
impl HighpassFilter {
    pub fn component_values(f: f64) -> HighpassFilter {
        let c1 = get_user_input("select c1");
        let c2 = c1;
        let r1 = 1f64 / (f * c1 * 2f64.sqrt() * PI);
        let r2 = r1 / 2f64;
        let c_in = 100f64 * c1;
        let c_out = c_in;
        println!("ideal {:?}", (c1, c2, r1, r2, c_in, c_out));

        let c1 = get_closest_value(c1, 'c', 5);
        let c2 = get_closest_value(c2, 'c', 5);
        let r1 = get_closest_value(r1, 'r', 5);
        let r2 = get_closest_value(r2, 'r', 5);
        let c_in = get_closest_value(c_in, 'c', 5);
        let c_out = get_closest_value(c_out, 'c', 5);
        println!("std {:?}", (c1, c2, r1, r2, c_in, c_out));

        let filter = HighpassFilter {
            c1,
            c2,
            r1,
            r2,
            c_out,
            c_in,
        };
        filter
    }
}

#[derive(Debug)]
pub struct BandpassFilterWide {
    // highpass for low pass band freq
    hp: HighpassFilter,
    // lowpass for high pass bandfreq
    lp: LowpassFilter,
}
impl BandpassFilterWide {
    pub fn component_values(f1: f64, f2: f64) -> BandpassFilterWide {
        let filter = BandpassFilterWide {
            hp: HighpassFilter::component_values(f1),
            lp: LowpassFilter::component_values(f2),
        };
        filter
    }
}

//dual supply filters
