pub mod aa_filter;
pub mod amp;
pub mod filters;
pub mod utils;
use crate::aa_filter::AntiAliasingFilter;
use crate::amp::AmplifierCircuit;
use crate::filters::BandpassFilterWide;
use clap::{Args, Parser, Subcommand};

/// program for calculating scaling and offset circuits and low pass filters
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// select type of circuit to calculate
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// for designing a scaling and offset circuit using the Texas Instruments 'designing gain and offset in thirty seconds' - Application Report SLOA097
    Amp(AmpArgs),
    /// for designing low pass filters (butterworth) using sergio franco's Design With Operational Amplifiers And Analog Integrated Circuits
    Filter(FilterArgs),
    /// for desiging low, high and bandpass filters (butterworth 2nd order) quickly using Filter design in thirty seconds - sloa93 app note - texas instruments
    QuickFilter(QuickFiltersArgs),
}

#[derive(Debug, Args)]
struct AmpArgs {
    /// reference voltage
    vref: f64,
    /// output full scale voltage
    vo_fs: f64,
    /// output zero scale voltage
    vo_zs: f64,
    /// input full scale voltage
    vi_fs: f64,
    /// input zero scale voltage
    vi_zs: f64,
}

#[derive(Debug, Args)]
struct FilterArgs {
    /// cutoff frequency,
    fc: f64,
    /// order of the filter (2, 4, 6, or 8)
    order: u8,
}

#[derive(Debug, Args)]
struct QuickFiltersArgs {
    /// pass band frequencies
    f1: f64,
    f2: f64,
    // filter type (unused)
    // filter_type: String,
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::Amp(amp_args) => {
            let vref = amp_args.vref;
            let vo_fs = amp_args.vo_fs;
            let vo_zs = amp_args.vo_zs;
            let vi_fs = amp_args.vi_fs;
            let vi_zs = amp_args.vi_zs;

            let circuit = AmplifierCircuit::calc(vref, vo_fs, vo_zs, vi_fs, vi_zs);
            println!("\ncomponent values:\n{:#?}", circuit);
        }
        Commands::Filter(filter_args) => {
            let fc = filter_args.fc;
            let order: u8;
            match filter_args.order {
                2 | 4 | 6 | 8 => {
                    order = filter_args.order;
                }
                _ => panic!("invalid order"),
            }
            let q = AntiAliasingFilter::q_factors(order);
            let filter = AntiAliasingFilter::component_values(q, fc, order);
            println!("\nfilter values:\n{:#?}", filter);
        }
        Commands::QuickFilter(quick_filter_args) => {
            let f1 = quick_filter_args.f1;
            let f2 = quick_filter_args.f2;
            let filter = BandpassFilterWide::component_values(f1, f2);
            println!("\ncomponent values:\n{:#?}", filter);
        }
    }
}

#[test]
fn test_get_closest_value() {
    use crate::utils::get_closest_value;
    // assert_eq!(get_closest_value(1.0, 'r', 5), 1.0);
    // assert_eq!(get_closest_value(0.99, 'r', 5), 1.0);
    // assert_eq!(get_closest_value(1.1, 'r', 10), 1.2);
    // assert_eq!(get_closest_value(10.0, 'r', 5), 10.0);
    assert_eq!(get_closest_value(9.5, 'r', 5), 9.1);

    assert_eq!(get_closest_value(1234.56, 'r', 5), 1.2e3);
    assert_eq!(get_closest_value(9876543.21, 'r', 1), 9.76e6);
    assert_eq!(get_closest_value(9876.54, 'r', 5), 1e3);

    // todo: write tests for capacitors
    assert_eq!(get_closest_value(0.001e-12, 'c', 5), 0.01e-12);
    assert_eq!(get_closest_value(1.387e-12, 'c', 5), 1.3e-12);
    assert_eq!(get_closest_value(9.87654e-9, 'c', 5), 1.0e-9);
}
