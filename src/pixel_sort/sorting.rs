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
use image::Pixel;
use image::Rgba;
use ordered_float::OrderedFloat;

pub fn lightness(pixel: &Rgba<u8>) -> OrderedFloat<f32> {
    if let [r, g, b, _] = pixel.channels() {
        let max = *r.max(g.max(b)) as f32;
        let min = *r.min(g.min(b)) as f32;

        OrderedFloat((min + max) / 2.)
    } else {
        unreachable!()
    }
}

pub fn hue(pixel: &Rgba<u8>) -> OrderedFloat<f32> {
    if let [r, g, b, _] = pixel.channels() {
        let max = *r.max(g.max(b)) as f32;
        let min = *r.min(g.min(b)) as f32;

        if max == min {
            return OrderedFloat(0.);
        }

        let chroma = max - min;

        let r_chroma = (max - *r as f32) / chroma;
        let g_chroma = (max - *g as f32) / chroma;
        let b_chroma = (max - *b as f32) / chroma;

        let h = if *r as f32 == max {
            b_chroma - g_chroma
        } else if *g as f32 == max {
            2. + r_chroma - b_chroma
        } else {
            4. + g_chroma - r_chroma
        };

        OrderedFloat((h / 6.) % 1.)
    } else {
        unreachable!()
    }
}

pub fn saturation(pixel: &Rgba<u8>) -> OrderedFloat<f32> {
    if let [r, g, b, _] = pixel.channels() {
        let max = *r.max(g.max(b)) as f32;
        let min = *r.min(g.min(b)) as f32;
        let l = (min + max) / 2.;

        if min == max {
            return OrderedFloat(0.);
        }

        if l <= 0.5 {
            OrderedFloat((max - min) / (max + min))
        } else {
            OrderedFloat((max - min) / (2. - max - min))
        }
    } else {
        unreachable!()
    }
}

pub fn intensity(pixel: &Rgba<u8>) -> OrderedFloat<f32> {
    if let [r, g, b, _] = pixel.channels() {
        OrderedFloat(*r as f32 + *g as f32 + *b as f32)
    } else {
        unreachable!()
    }
}

pub fn minimum(pixel: &Rgba<u8>) -> OrderedFloat<f32> {
    if let [r, g, b, _] = pixel.channels() {
        OrderedFloat((*r.min(g.min(b))) as f32)
    } else {
        unreachable!()
    }
}
