#![no_std]
// associated re-typing not supported in rust yet
#![allow(clippy::type_complexity)]
#![allow(non_camel_case_types)]

//! This crate provides a ST7796S driver to connect to TFT displays.

pub mod instruction;

use crate::instruction::Command;
