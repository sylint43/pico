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
pub use self::interval::Interval;
use self::sorter::sort_image;
use image::{imageops::rotate270, imageops::rotate90, Rgba, RgbaImage};

pub fn pixel_sort(
    image: RgbaImage,
    lower_threshold: f32,
    upper_threshold: f32,
    interval: Interval,
    scale: u32,
) -> RgbaImage {
    match interval {
        Interval::Threshold => {
            let interval_fn = |lower_threshold: f32, upper_threshold: f32| {
                move |image: &RgbaImage| {
                    interval::threshold(image, lower_threshold, upper_threshold)
                }
            };
            let vertical_sort = helper(
                rotate90(&image),
                interval_fn(lower_threshold, upper_threshold),
            );
            helper(
                rotate270(&vertical_sort),
                interval_fn(lower_threshold, upper_threshold),
            )
        }
        Interval::Random => {
            let interval_fn = |scale: u32| move |image: &RgbaImage| interval::random(image, scale);
            helper(image, interval_fn(scale))
        }
    }
}

fn helper<F>(image: RgbaImage, interval_fn: F) -> RgbaImage
where
    F: Fn(&RgbaImage) -> Vec<Vec<u32>>,
{
    let intervals = interval_fn(&image);
    let pixels = sort_image(&image, intervals);
    place_pixels(pixels, &image)
}

fn place_pixels(pixels: Vec<Vec<&Rgba<u8>>>, original: &RgbaImage) -> RgbaImage {
    let mut output = RgbaImage::new(original.width(), original.height());

    for (x, y, pixel) in output.enumerate_pixels_mut() {
        *pixel = *pixels[y as usize][x as usize];
    }

    output
}

mod interval;
mod sorter;
mod sorting;
