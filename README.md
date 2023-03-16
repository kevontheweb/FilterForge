# scaling and offset circuit calculator

this program generates component values for a scaling and offset circuit using 'designing gain and offset in thirty seconds' - Application Report SLOA097 (Texas Instruments)

I just wanted to play with rust, it was quite fun to make this and I tried to handle all errors possible (i still want to check for NaNs in output). If you use this and find errors please let me know by opening an issue.

I also think I used enums incorrectly and should move the different circuit topologies out to separate structs then have logic for determining which struct is constructed elsewhere. but it functions as it is.

## todo

- [ ] refactor and move aa filter and scaling circuits out to separate libs
- [ ] better cli interface with a proper help message
- [ ] standard component suggestions
- [ ] spice netlist output

