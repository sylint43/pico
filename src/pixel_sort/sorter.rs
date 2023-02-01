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
use super::sorting::lightness;
use image::{Rgba, RgbaImage};

pub fn sort_image(image: &RgbaImage, intervals: Vec<Vec<u32>>) -> Vec<Vec<&Rgba<u8>>> {
    let mut sorted_pixels = vec![];
    for y in 0..image.height() {
        let mut row: Vec<&Rgba<u8>> = vec![];
        let mut x_min = 0;
        let mut xs = intervals[y as usize].clone();
        xs.push(image.width()); // ensure we go through the entire row

        // For loop progressivly takes interval end-point and copies all the pixels
        // from the last end-point to the current, then sorts.
        for x_max in xs {
            let mut interval = vec![];
            for x in x_min..x_max {
                interval.push(image.get_pixel(x, y))
            }
            interval.sort_by_key(|p| lightness(p));
            row.extend(interval);
            x_min = x_max;
        }

        sorted_pixels.push(row)
    }

    sorted_pixels
}
