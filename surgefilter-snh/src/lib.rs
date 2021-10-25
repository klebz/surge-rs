#![feature(in_band_lifetimes)]

macro_rules! x  { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { () => ( use crate::{ imports::* , };) }

#[macro_use] pub mod imports;
#[cfg(test)] pub mod tests;

x![snh];