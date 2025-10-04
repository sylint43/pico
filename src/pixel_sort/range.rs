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
use image::{GrayImage, RgbaImage};
use ordered_float::OrderedFloat;
use rand::Rng;

pub trait PixelRange {
    fn create_pixel_ranges(&self, image: &RgbaImage) -> Vec<Vec<u32>>;
}

pub struct Threshold {
    pub lower: f32,
    pub upper: f32,
}

impl PixelRange for Threshold {
    fn create_pixel_ranges(&self, image: &RgbaImage) -> Vec<Vec<u32>> {
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

impl PixelRange for Random {
    fn create_pixel_ranges(&self, image: &RgbaImage) -> Vec<Vec<u32>> {
        let mut rng = rand::rng();
        let mut intervals: Vec<Vec<u32>> = vec![vec![]; image.height() as usize];

        for y in 0..image.height() {
            let mut x = (self.scale as f32 * rng.random::<f32>()) as u32;
            while x < image.width() {
                intervals[y as usize].push(x);
                x += (self.scale as f32 * rng.random::<f32>()) as u32;
            }
        }

        intervals
    }
}

pub struct Wave {
    pub scale: u32,
}

impl PixelRange for Wave {
    fn create_pixel_ranges(&self, image: &RgbaImage) -> Vec<Vec<u32>> {
        let mut rng = rand::rng();
        let mut intervals: Vec<Vec<u32>> = vec![vec![]; image.height() as usize];

        for y in 0..image.height() {
            let mut x = self.scale + rng.random_range(0..10);
            while x < image.width() {
                intervals[y as usize].push(x);
                x += self.scale + rng.random_range(0..10);
            }
        }

        intervals
    }
}

pub struct File {
    pub mask: GrayImage,
}

impl PixelRange for File {
    fn create_pixel_ranges(&self, image: &RgbaImage) -> Vec<Vec<u32>> {
        assert_eq!(
            image.dimensions(),
            self.mask.dimensions(),
            "Mask must be same size as input image"
        );

        let mut intervals = vec![vec![]; image.height() as usize];

        for (x, y, p) in self.mask.enumerate_pixels() {
            let pixel_value = p.0[0];
            let black = 0;

            if pixel_value == black {
                intervals[y as usize].push(x);
            }
        }

        intervals
    }
}

pub struct None;

impl PixelRange for None {
    fn create_pixel_ranges(&self, image: &RgbaImage) -> Vec<Vec<u32>> {
        vec![vec![]; image.height() as usize]
    }
}
