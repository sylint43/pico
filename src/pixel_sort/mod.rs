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
use image::{GrayImage, ImageBuffer, Rgba, RgbaImage};
use ordered_float::OrderedFloat;
pub use sorting::*;

pub type SortFn = dyn Fn(&Rgba<u8>) -> OrderedFloat<f32>;

pub struct PixelSort {
    image: RgbaImage,
    mask: GrayImage,
    interval: Box<dyn Interval>,
    sort_fn: Box<SortFn>,
}

impl PixelSort {
    pub fn new(
        image: RgbaImage,
        mask: GrayImage,
        interval: Box<dyn Interval>,
        sort_fn: Box<SortFn>,
    ) -> Self {
        Self {
            image,
            mask,
            interval,
            sort_fn,
        }
    }

    pub fn sort(&self) -> RgbaImage {
        let intervals = self.interval.create_intervals(&self.image);
        let pixels = self.sort_pixels(intervals, &self.sort_fn);
        self.place_pixels(pixels)
    }

    fn sort_pixels<F>(&self, intervals: Vec<Vec<u32>>, sort_fn: F) -> Vec<Vec<&Rgba<u8>>>
    where
        F: Fn(&Rgba<u8>) -> OrderedFloat<f32>,
    {
        (0..self.image.height())
            .map(|y| {
                let width_slice = &[self.image.width()];
                let xs = [0u32]
                    .iter()
                    .chain(&intervals[y as usize])
                    .chain(width_slice)
                    .collect::<Vec<&u32>>();
                let row = xs
                    .windows(2)
                    .flat_map(<&[&u32; 2]>::try_from)
                    .flat_map(|[&start, &end]| {
                        let mut interval = (start..end)
                            .filter(|x| self.mask.get_pixel(*x, y).0[0] == 255)
                            .map(|x| self.image.get_pixel(x, y))
                            .collect::<Vec<&Rgba<u8>>>();
                        interval.sort_by_key(|p| sort_fn(p));
                        interval
                    })
                    .collect::<Vec<&Rgba<u8>>>();
                row
            })
            .collect::<Vec<Vec<&Rgba<u8>>>>()
    }

    fn place_pixels(&self, pixels: Vec<Vec<&Rgba<u8>>>) -> RgbaImage {
        let mut image = ImageBuffer::new(self.image.width(), self.image.height());

        for y in 0..self.image.height() {
            let mut count = 0;
            for x in 0..self.image.width() {
                if self.mask.get_pixel(x, y).0[0] == 0 {
                    image.put_pixel(x, y, *self.image.get_pixel(x, y));
                } else {
                    image.put_pixel(x, y, *pixels[y as usize][count]);
                    count += 1;
                }
            }
        }

        image
    }
}

pub mod interval;
mod sorting;
