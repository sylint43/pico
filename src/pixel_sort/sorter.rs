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
use ordered_float::OrderedFloat;

pub fn sort_image(image: &RgbaImage, intervals: Vec<Vec<u32>>) -> Vec<Vec<&Rgba<u8>>> {
    let mut sorted_pixels = vec![];
    for y in 0..image.height() {
        let mut row: Vec<&Rgba<u8>> = vec![];
        let mut x_min = 0;
        let mut xs = if let Some(row) = intervals.get(y as usize) {
            row.clone()
        } else {
            vec![]
        };
        xs.push(image.width());
        for x_max in xs {
            let mut interval = vec![];
            for x in x_min..x_max {
                interval.push(image.get_pixel(x, y))
            }
            row.extend(sort_interval(interval, lightness));
            x_min = x_max;
        }

        sorted_pixels.push(row)
    }

    sorted_pixels
}

fn sort_interval(
    interval: Vec<&Rgba<u8>>,
    sort_function: fn(&Rgba<u8>) -> OrderedFloat<f32>,
) -> Vec<&Rgba<u8>> {
    let mut ret = interval.clone();
    ret.sort_by_key(|p| sort_function(p));
    ret
}
