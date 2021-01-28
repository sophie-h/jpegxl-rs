/*
This file is part of jpegxl-rs.

jpegxl-rs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

jpegxl-rs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with jpegxl-rs.  If not, see <https://www.gnu.org/licenses/>.
*/
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Overview
//! A safe JPEGXL wrapper over `jpeg-xl` library. Check out the original [library](https://gitlab.com/wg1/jpeg-xl)
//! and the [bindings](https://github.com/inflation/jpegxl-sys).

//! # Usage

//! ## Decoding
//! ```
//! # use jpegxl_rs::*;
//! # || -> Result<(), Box<dyn std::error::Error>> {
//! let mut decoder: JXLDecoder<u8> = decoder_builder().build()?;
//!
//! // Use another pixel data type
//! let mut decoder: JXLDecoder<f32> = decoder_builder().build()?;
//!
//! // Customize pixel format
//! let mut decoder: JXLDecoder<u8> = decoder_builder()
//!                                     .num_channels(3)
//!                                     .endian(Endianness::Big)
//!                                     .align(8)
//!                                     .build()?;
//!
//! // Set custom parallel runner and memory manager
//! use jpegxl_rs::{parallel::ThreadsRunner, memory::MallocManager};
//! let manager = Box::new(MallocManager::default());
//! let runner = Box::new(ThreadsRunner::default());
//! let mut decoder: JXLDecoder<u8> = decoder_builder()
//!                                     .memory_manager(manager)
//!                                     .parallel_runner(runner)
//!                                     .build()?;
//! # Ok(()) };
//! ```

//! ## Encoding
//! ```
//! # use jpegxl_rs::encoder_builder;
//! # || -> Result<(), Box<dyn std::error::Error>> {
//! use image::io::Reader as ImageReader;
//! let sample = ImageReader::open("test/sample.png")?.decode()?.to_rgba16();
//! let mut encoder = encoder_builder().build()?;
//! let buffer = encoder.encode(&sample, sample.width() as _, sample.height() as _)?;
//! # Ok(()) };
//! ```
//!
//! Set encoder options
//! ```
//! # || -> Result<(), Box<dyn std::error::Error>> {
//! # use jpegxl_rs::*;
//! let mut encoder = encoder_builder()
//!                     .lossless(true)
//!                     .speed(EncoderSpeed::Falcon)
//!                     .build()?;
//! // You can change the settings after initialization
//! encoder.set_lossless(false);
//! encoder.set_quality(3.0);
//! # Ok(()) };
//! ```

mod common;
mod decode;
mod encode;
mod errors;
pub mod memory;
pub mod parallel;

#[cfg(not(feature = "without-image"))]
pub mod image;

pub use common::*;
pub use decode::*;
pub use encode::*;
pub use errors::*;
