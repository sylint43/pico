// Copyright (C) 2023 Sylvia Waldron
//
// This file is part of pico.
//
// pico is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// pico is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with pico.  If not, see <http://www.gnu.org/licenses/>.
use clap::{Parser, Subcommand};
use image::{Pixel, Rgba};
use pico::pixel_sort::{pixel_sort, Interval};
use std::{ffi::OsStr, path::PathBuf};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Cmd {
    #[command(subcommand)]
    mode: Mode,
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Mode {
    Cbrt,
    PixelSort {
        #[arg(default_value_t = 0.2, short, long)]
        lower_threshold: f32,
        #[arg(default_value_t = 0.85, short, long)]
        upper_threshold: f32,
        #[arg(default_value_t = Interval::Threshold, value_enum, short, long)]
        interval: Interval,
        #[arg(default_value_t = 50, short, long)]
        scale: u32,
    },
}

fn main() -> Result<(), image::ImageError> {
    let args = Cmd::parse();
    let output_file = args.output.unwrap_or(
        args.input
            .file_name()
            .unwrap_or(OsStr::new("output.jpg"))
            .into(),
    );
    let mut image = image::open(args.input)?.to_rgba8();

    let output_image = match args.mode {
        Mode::Cbrt => {
            image.pixels_mut().for_each(|pixel| {
                let (bytes, _) = pixel.channels().split_at(std::mem::size_of::<f32>());
                let channels_f32 = f32::from_ne_bytes(bytes.try_into().unwrap());
                *pixel = Rgba::from(channels_f32.cbrt().to_ne_bytes())
            });

            image
        }
        Mode::PixelSort {
            upper_threshold,
            lower_threshold,
            interval,
            scale,
        } => pixel_sort(image, lower_threshold, upper_threshold, interval, scale),
    };

    output_image.save(output_file)?;

    Ok(())
}
