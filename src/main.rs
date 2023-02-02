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
use clap::{Parser, Subcommand, ValueEnum};
use image::{ImageBuffer, Pixel, Rgba};
use pico::pixel_sort::{self, interval, PixelSort, SortFn};
use std::{ffi::OsStr, path::PathBuf};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Cmd {
    #[command(subcommand)]
    glitch: GlitchMode,
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum GlitchMode {
    Cbrt,
    PixelSort {
        #[command(subcommand)]
        interval: IntervalMode,
        #[arg(short, long)]
        mask: Option<PathBuf>,
        #[arg(short, long, value_enum)]
        sort: Sort,
    },
}

#[derive(Subcommand, Debug)]
enum IntervalMode {
    Threshold {
        #[arg(default_value_t = 0.2, short, long)]
        lower: f32,
        #[arg(default_value_t = 0.85, short, long)]
        upper: f32,
    },
    Random {
        #[arg(default_value_t = 50, short, long)]
        scale: u32,
    },
    None,
}

#[derive(ValueEnum, Debug, Copy, Clone)]
enum Sort {
    Lightness,
    Hue,
    Saturation,
    Intensity,
    Minimum,
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

    let output_image = match args.glitch {
        GlitchMode::Cbrt => {
            image.pixels_mut().for_each(|pixel| {
                let (bytes, _) = pixel.channels().split_at(std::mem::size_of::<f32>());
                let channels_f32 = f32::from_ne_bytes(bytes.try_into().unwrap());
                *pixel = Rgba::from(channels_f32.cbrt().to_ne_bytes())
            });

            image
        }
        GlitchMode::PixelSort {
            interval,
            mask,
            sort,
        } => {
            let mask = match mask {
                Some(path) => image::open(path)?.to_luma8(),
                None => ImageBuffer::from_fn(image.width(), image.height(), |_, _| [255].into()),
            };

            assert_eq!(
                mask.dimensions(),
                image.dimensions(),
                "Mask must be same size as input image"
            );

            let interval: Box<dyn interval::Interval> = match interval {
                IntervalMode::Threshold { lower, upper } => {
                    Box::new(interval::Threshold { lower, upper })
                }
                IntervalMode::Random { scale } => Box::new(interval::Random { scale }),
                IntervalMode::None => Box::new(interval::None),
            };

            let sort: Box<SortFn> = match sort {
                Sort::Lightness => Box::new(pixel_sort::lightness),
                Sort::Hue => Box::new(pixel_sort::hue),
                Sort::Saturation => Box::new(pixel_sort::saturation),
                Sort::Intensity => Box::new(pixel_sort::intensity),
                Sort::Minimum => Box::new(pixel_sort::minimum),
            };

            PixelSort::new(image, mask, interval, sort).sort()
        }
    };

    output_image.save(output_file)?;

    Ok(())
}
