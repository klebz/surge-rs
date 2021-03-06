#![feature(in_band_lifetimes)]

#[macro_use] mod imports; use imports::*;
#[cfg(test)] mod tests;

x![aux];
x![block];
x![flanger];
x![constants];
x![filter];
x![init];
x![init_comb];
x![interp_delay];
x![param];
x![process];
x![weights];
