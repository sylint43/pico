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
use self::range::PixelRange;
use image::{GrayImage, ImageBuffer, Rgba, RgbaImage};
use itertools::Itertools;
use ordered_float::OrderedFloat;
pub use sorting::*;
use std::iter;

pub type SortFn = dyn Fn(&Rgba<u8>) -> OrderedFloat<f32>;

pub struct PixelSort {
    image: RgbaImage,
    mask: GrayImage,
    range: Box<dyn PixelRange>,
    sort_fn: Box<SortFn>,
}

impl PixelSort {
    pub fn new(
        image: RgbaImage,
        mask: GrayImage,
        range: Box<dyn PixelRange>,
        sort_fn: Box<SortFn>,
    ) -> Self {
        Self {
            image,
            mask,
            range,
            sort_fn,
        }
    }

    pub fn sort(&self) -> RgbaImage {
        let ranges = self.range.create_pixel_ranges(&self.image);
        let pixels = self.sort_pixels(ranges, &self.sort_fn);
        self.place_pixels(pixels)
    }

    fn sort_pixels<F>(&self, ranges: Vec<Vec<u32>>, sort_fn: F) -> Vec<Vec<&Rgba<u8>>>
    where
        F: Fn(&Rgba<u8>) -> OrderedFloat<f32>,
    {
        ranges
            .into_iter()
            .enumerate()
            .map(|(y, xs)| {
                iter::once(0u32)
                    .chain(xs)
                    .chain(iter::once(self.image.width()))
                    .tuple_windows::<(u32, u32)>()
                    .flat_map(|(start, end)| {
                        (start..end)
                            .filter(|x| self.mask.get_pixel(*x, y as u32).0[0] != 0)
                            .map(|x| self.image.get_pixel(x, y as u32))
                            .sorted_by_key(|p| sort_fn(p))
                    })
                    .collect::<Vec<&Rgba<u8>>>()
            })
            .collect::<Vec<Vec<&Rgba<u8>>>>()
    }

    fn place_pixels(&self, pixels: Vec<Vec<&Rgba<u8>>>) -> RgbaImage {
        let mut image = ImageBuffer::new(self.image.width(), self.image.height());

        for y in 0..self.image.height() {
            let mut pixels = pixels[y as usize].iter();
            for x in 0..self.image.width() {
                if self.mask.get_pixel(x, y).0[0] == 0 {
                    image.put_pixel(x, y, *self.image.get_pixel(x, y));
                } else {
                    image.put_pixel(x, y, **pixels.next().unwrap());
                }
            }
        }

        image
    }
}

pub mod range;
mod sorting;
