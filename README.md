# FilterForge

A scaling and offset circuit, and active lowpass filter calculator.

![lpf response](./circuits/ngspice/response.svg)

This program generates component values for a scaling and offset circuit using 'designing gain and offset in thirty seconds' - Application Report SLOA097 (Texas Instruments)
It also currently can design butterworth low pass filters with orders 2, 4, 6, and 8. based on the unity gain KRC circuit in 'Design with Operational Amplifiers' - S. Franco

It suggests the closest standard component values.

I mostly just wanted to play with rust, it was quite fun to make this and I tried to handle all errors possible.
If you use this and find errors please let me know by opening an issue.

I also think I used enums incorrectly and should move the different circuit topologies out to separate structs then have logic for determining which struct is constructed elsewhere
That is mostly an issue about writing idiomatic (or 'rusty') rust code, which hopefully will improve as I learn more rust. 🦀

## todo

- [x] refactor and move aa filter and scaling circuits out to separate libs
- [ ] add support for chebychev filters
- [ ] add support for highpass and bandpass filters
- [x] standard component suggestions
  - [x] find closest standard component value
- [ ] spice netlist output
  - [ ] make generic spice netlist for the amplifier topologies
  - [ ] make generic spice netlist for the filter topologies
- [ ] better cli interface
  - [x] commands
    - [x] `filter` (takes in cutoff frequency and order)
      - [ ] `--butterworth`, `--chebychev`
      - [ ] `--std` for standard component out (currently **does** this by default)
    - [x] `amp` (takes in voltages, reference, output full scale, output zero scale, input full scale, input zero scale)
      - [ ] `--std` for standard component out (currently **does not** do this by default)
    - [ ] `--spice` (output netlist to stdout, with simulation directives, maybe suggest ngspice command to plot freq response)
    - [ ] `--help`
  - [ ] output
    - [x] fix messed up order of values in output
    - [ ] implement format traits for output so that it can pretty print as json
    - [ ] improve output to show both ideal and standard
