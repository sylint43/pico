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
use image::RgbaImage;
use ordered_float::OrderedFloat;

pub fn threshold(image: &RgbaImage, lower_threshold: f32, upper_threshold: f32) -> Vec<Vec<u32>> {
    let mut intervals: Vec<Vec<u32>> = vec![];
    for (x, y, p) in image.enumerate_pixels() {
        let level = lightness(p);
        if level < OrderedFloat(lower_threshold * 255.)
            || level > OrderedFloat(upper_threshold * 255.)
        {
            if let Some(row) = intervals.get_mut(y as usize) {
                row.push(x);
            } else {
                intervals.push(vec![x]);
            }
        }
    }

    intervals
}
