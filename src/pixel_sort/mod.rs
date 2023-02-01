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
use self::sorter::sort_image;
use image::{Rgba, RgbaImage};

pub fn pixel_sort(image: RgbaImage) -> RgbaImage {
    let intervals = interval::threshold(&image, 0.2, 0.85);
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
