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
use image::{Rgba, RgbaImage};
use ordered_float::OrderedFloat;

pub fn sort_image<F>(
    image: &RgbaImage,
    intervals: Vec<Vec<u32>>,
    sorting_fn: F,
) -> Vec<Vec<&Rgba<u8>>>
where
    F: Fn(&Rgba<u8>) -> OrderedFloat<f32>,
{
    let mut sorted_pixels = vec![];
    for y in 0..image.height() {
        let mut row: Vec<&Rgba<u8>> = vec![];
        let mut x_min = 0;
        let xs = &intervals[y as usize];
        // For loop progressivly takes interval end-point and copies all the pixels
        // from the last end-point to the current, then sorts.
        for &x_max in xs.iter().chain(vec![image.width()].iter()) {
            let mut interval = (x_min..x_max)
                .map(|x| image.get_pixel(x, y))
                .collect::<Vec<&Rgba<u8>>>();
            interval.sort_by_key(|p| sorting_fn(p));
            row.extend(interval);
            x_min = x_max;
        }

        sorted_pixels.push(row)
    }

    sorted_pixels
}
