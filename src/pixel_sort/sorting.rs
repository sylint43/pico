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
