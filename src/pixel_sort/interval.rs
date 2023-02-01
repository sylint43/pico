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
use rand::Rng;

/*
Intervals are (x, y) pixel coordinates into the input image that marks the end of a interval
of pixels. They will always have image.height() rows but each row may not have image.width() columns.

A coordinate is inserted into the vec based on the interval function used to create the intervals

Threshold looks for pixels within a certain lightness threshold and marks pixels that don't pass
Random randomly jumps to a pixel in a row and adds it to the interval list. Can be scaled.
 */

pub trait Interval {
    fn create_intervals(&self, image: &RgbaImage) -> Vec<Vec<u32>>;
}

pub struct Threshold {
    pub lower: f32,
    pub upper: f32,
}

impl Interval for Threshold {
    fn create_intervals(&self, image: &RgbaImage) -> Vec<Vec<u32>> {
        let mut intervals: Vec<Vec<u32>> = vec![vec![]; image.height() as usize];
        for (x, y, p) in image.enumerate_pixels() {
            let level = lightness(p);
            if level < OrderedFloat(self.lower * 255.) || level > OrderedFloat(self.upper * 255.) {
                intervals[y as usize].push(x);
            }
        }

        intervals
    }
}

pub struct Random {
    pub scale: u32,
}

impl Interval for Random {
    fn create_intervals(&self, image: &RgbaImage) -> Vec<Vec<u32>> {
        let mut rng = rand::thread_rng();
        let mut intervals: Vec<Vec<u32>> = vec![vec![]; image.height() as usize];

        for y in 0..image.height() {
            let mut x = 0;
            loop {
                x += (self.scale as f32 * rng.gen::<f32>()) as u32;
                if x > image.width() {
                    break;
                }

                intervals[y as usize].push(x);
            }
        }

        intervals
    }
}
