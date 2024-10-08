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
use memoize::memoize;
use pico::pixel_sort::{self, range, PixelSort, SortFn};
use rand::seq::SliceRandom;
use std::{ffi::OsStr, path::PathBuf};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Cmd {
    #[command(subcommand)]
    glitch: GlitchMode,
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    mask: Option<PathBuf>,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(short, long, value_enum)]
    angle: Option<Angle>,
}

#[derive(Subcommand, Debug)]
enum GlitchMode {
    Cbrt,
    Fib,
    Sum,
    PixelSort {
        #[command(subcommand)]
        range: RangeMode,
        #[arg(short, long, value_enum, default_value_t=Sort::Lightness)]
        sort: Sort,
    },
    Shuffle,
}

#[derive(Subcommand, Debug)]
enum RangeMode {
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
    Wave {
        #[arg(default_value_t = 50, short, long)]
        scale: u32,
    },
    File {
        #[arg(short, long)]
        path: PathBuf,
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

#[derive(ValueEnum, Debug, Copy, Clone)]
enum Angle {
    Ninty,
    OneEighty,
    TwoSeventy,
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
    let mut mask = match args.mask {
        Some(path) => image::open(path)?.to_luma8(),
        None => ImageBuffer::from_fn(image.width(), image.height(), |_, _| [255].into()),
    };

    assert_eq!(
        mask.dimensions(),
        image.dimensions(),
        "Mask must be same size as input image"
    );

    image = match args.angle {
        Some(Angle::Ninty) => image::imageops::rotate90(&image),
        Some(Angle::OneEighty) => image::imageops::rotate180(&image),
        Some(Angle::TwoSeventy) => image::imageops::rotate270(&image),
        None => image,
    };

    mask = match args.angle {
        Some(Angle::Ninty) => image::imageops::rotate90(&mask),
        Some(Angle::OneEighty) => image::imageops::rotate180(&mask),
        Some(Angle::TwoSeventy) => image::imageops::rotate270(&mask),
        None => mask,
    };

    let mut output_image = match args.glitch {
        GlitchMode::Cbrt => {
            image
                .enumerate_pixels_mut()
                .filter(|(x, y, _)| mask.get_pixel(*x, *y).0[0] != 0)
                .for_each(|(_, _, pixel)| {
                    let (bytes, _) = pixel.channels().split_at(std::mem::size_of::<f32>());
                    let channels_f32 = f32::from_ne_bytes(bytes.try_into().unwrap());
                    *pixel = Rgba::from(channels_f32.cbrt().to_ne_bytes());
                });

            image
        }
        GlitchMode::Fib => {
            for (_, _, pixel) in image
                .enumerate_pixels_mut()
                .filter(|(x, y, _)| mask.get_pixel(*x, *y).0[0] != 0)
            {
                pixel.apply(|p| (fib(p) % 256) as u8);
            }

            image
        }
        GlitchMode::Sum => {
            for (_, _, pixel) in image
                .enumerate_pixels_mut()
                .filter(|(x, y, _)| mask.get_pixel(*x, *y).0[0] != 0)
            {
                pixel.apply(sum_of_squares);
            }

            image
        }
        GlitchMode::Shuffle => {
            let mut rng = rand::thread_rng();

            for (_, _, pixel) in image
                .enumerate_pixels_mut()
                .filter(|(x, y, _)| mask.get_pixel(*x, *y).0[0] != 0)
            {
                pixel.channels_mut().shuffle(&mut rng);
            }

            image
        }
        GlitchMode::PixelSort { range, sort } => {
            let range: Box<dyn range::PixelRange> = match range {
                RangeMode::Threshold { lower, upper } => {
                    Box::new(range::Threshold { lower, upper })
                }
                RangeMode::Random { scale } => Box::new(range::Random { scale }),
                RangeMode::Wave { scale } => Box::new(range::Wave { scale }),
                RangeMode::File { path } => {
                    let mut mask = image::open(path)?.to_luma8();

                    mask = match args.angle {
                        Some(Angle::Ninty) => image::imageops::rotate90(&mask),
                        Some(Angle::OneEighty) => image::imageops::rotate180(&mask),
                        Some(Angle::TwoSeventy) => image::imageops::rotate270(&mask),
                        None => mask,
                    };

                    Box::new(range::File { mask })
                }
                RangeMode::None => Box::new(range::None),
            };

            let sort: Box<SortFn> = match sort {
                Sort::Lightness => Box::new(pixel_sort::lightness),
                Sort::Hue => Box::new(pixel_sort::hue),
                Sort::Saturation => Box::new(pixel_sort::saturation),
                Sort::Intensity => Box::new(pixel_sort::intensity),
                Sort::Minimum => Box::new(pixel_sort::minimum),
            };

            PixelSort::new(image, mask, range, sort).sort()
        }
    };

    output_image = match args.angle {
        Some(Angle::Ninty) => image::imageops::rotate270(&output_image),
        Some(Angle::OneEighty) => image::imageops::rotate180(&output_image),
        Some(Angle::TwoSeventy) => image::imageops::rotate90(&output_image),
        None => output_image,
    };

    output_image.save(output_file)?;

    Ok(())
}

fn fib(n: u8) -> u64 {
    #[memoize]
    fn fib_inner(n: u8, prev_fib: u64, fib: u64) -> u64 {
        match n {
            0 => prev_fib,
            n => fib_inner(n - 1, fib, prev_fib + fib),
        }
    }
    fib_inner(n, 0, 1)
}

fn divisors(n: u8) -> Vec<u8> {
    let sqrt = ((n as f32).sqrt()) as u8;
    let divisors = (1..sqrt).filter(|divisor| n % *divisor == 0);
    let divisors_clone = divisors.clone();
    divisors
        .chain(divisors_clone.map(|divisor| n / divisor))
        .collect()
}

fn sum_of_squares(n: u8) -> u8 {
    (divisors(n)
        .iter()
        .map(|divisor| (*divisor as u64).pow(2))
        .sum::<u64>()
        % 256) as u8
}
