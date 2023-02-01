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
use self::interval::Interval;
use self::{sorter::sort_image, sorting::lightness};
use image::{imageops::rotate270, imageops::rotate90, ImageBuffer, RgbaImage};

pub fn pixel_sort(image: RgbaImage, interval: &impl Interval) -> RgbaImage {
    let vertical_sort = helper(rotate90(&image), interval);
    helper(rotate270(&vertical_sort), interval)
}

fn helper(image: RgbaImage, interval: &impl Interval) -> RgbaImage {
    let intervals = interval.create_intervals(&image);
    let pixels = sort_image(&image, intervals, lightness);
    ImageBuffer::from_fn(image.width(), image.height(), |x, y| {
        *pixels[y as usize][x as usize]
    })
}

pub mod interval;
mod sorter;
mod sorting;
