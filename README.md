# scaling and offset circuit, and active lowpass filter calculator

This program generates component values for a scaling and offset circuit using 'designing gain and offset in thirty seconds' - Application Report SLOA097 (Texas Instruments)
It also currently can design butterworth low pass filters with orders 2, 4, 6, and 8. based on the unity gain KRC circuit in 'Design with Operational Amplifiers' - S. Franco

There is a function for suggesting closest standard components but it is currently not implemented properly

I mostly just wanted to play with rust, it was quite fun to make this and I tried to handle all errors possible. 
If you use this and find errors please let me know by opening an issue.

I also think I used enums incorrectly and should move the different circuit topologies out to separate structs then have logic for determining which struct is constructed elsewhere
That is mostly an issue about writing idiomatic (or 'rusty') rust code, which hopefully will improve as I learn more rust. 🦀

## todo

- [ ] refactor and move aa filter and scaling circuits out to separate libs

- [ ] add support for chebychev filters
- [x] standard component suggestions
  - [ ] improve output to show both ideal and standard
  - [ ] find closest standard component value
- [ ] spice netlist output
  - [ ] make generic spice netlist for the amplifier topologies
  - [ ] make generic spice netlist for the filter topologies
- [ ] better cli interface 
  - [ ] remove interactivity and require flags.
    > this may not be possible since the user inputs depend on which topology is chosen. 
    > I also don't want to make the user figure out which topology they should use.
  - [ ] flags
    - [ ] `--lpf` with options for `butterworth` and `chebyshev` 
      - [ ] `--order` required if `--lpf` is passed
    - [ ] `--amp` (default behaviour)
    - [ ] `--spice` (output netlist to stdout, with simulation directives, maybe suggest ngspice command to plot freq response)
    - [ ] `--std` for standard component out
    - [ ] `--help`


