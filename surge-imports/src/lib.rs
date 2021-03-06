#[macro_export] macro_rules! x { 
    ($x:ident) => { 
        mod $x; 
        pub use $x::*; 
    }
}

#[macro_export] macro_rules! ix { 
    () => { 
        use crate::{ 
            imports::* , 
        };
        use crate::*;
    } 
}

#[macro_export] macro_rules! tsyncratio {
    ($self:ident, $x:ident) => ({
        let temposync = $self.params[AdsrParam::$x].temposync;
        match temposync {
            true => $self.time_unit.temposyncratio(),
            false => 1.0,
        }
    })
}


#[cfg(target_arch = "x86_64")] pub use core::arch::x86_64::*;
pub use ::approx_eq::*;
pub use atomic_float::{AtomicF64,AtomicF32};
pub use auto_impl::*;
pub use bitflags::*;
pub use byteorder::{ByteOrder,BigEndian,LittleEndian};
pub use core::arch::x86_64::*;
pub use core::convert::*;
pub use core::ffi::*;
pub use core::ops::*;
pub use coreaudio::audio_unit::*;
pub use coreaudio::Error;
pub use coreaudio::audio_unit::render_callback::*;
pub use derivative::*;
pub use derive_more::*;
pub use dft::*;
pub use dft;
pub use std::cmp::Ordering;
pub use downcast_rs::*;
pub use enhanced_enum::*;
pub use enum_dispatch::*;
pub use float_ord::*;
pub use getset::*;
pub use half::*;
pub use indoc::*;
pub use lazy_static::*;
pub use libc;
pub use lockfree::map::Map as LFMap;
pub use mopa::*;
pub use ndarray::*;
pub use ndarray;
pub use num::*;
pub use num::complex::*;
pub use num_traits::*;
pub use num_traits::pow::*;
pub use num_traits;
pub use paste::*;
pub use proc_macro2::*;
pub use prometheus::core::*;
pub use rand::*;
pub use rand;
pub use regex::*;
pub use serde::*;
pub use std::any::*;
pub use std::cell::*;
pub use std::cmp::*;
pub use std::collections::*;
pub use std::convert::*;
pub use std::f32::consts::PI as PI_32;
pub use std::f64::consts::PI as PI;
pub use std::ffi::*;
pub use std::ffi;
pub use std::fmt::*;
pub use std::fmt;
pub use std::fs::*;
pub use std::io::*;
pub use std::io::Write;
pub use std::iter::*;
pub use std::marker::*;
pub use std::ops::*;
pub use std::path::*;
pub use std::pin::*;
pub use std::rc::*;
pub use std::sync::*;
pub use std::sync::atomic::*;
pub use std::sync::atomic;
pub use std::thread;
pub use std::time::*;
pub use std::time;
pub use uuid::*;
pub use vst::api::*;
pub use vst::buffer::*;
pub use vst::event::*;
pub use vst::host::*;
pub use vst::plugin::*;
