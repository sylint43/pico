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
use clap::Parser;
use image::{Pixel, Rgba};
use std::{ffi::OsStr, path::PathBuf};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Cmd {
    image: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
}
fn main() -> Result<(), image::ImageError> {
    let args = Cmd::parse();
    let output_file = args.output.unwrap_or(
        args.image
            .file_name()
            .unwrap_or(OsStr::new("output.jpg"))
            .into(),
    );
    let mut image = image::open(args.image)?.to_rgba8();
    image.pixels_mut().for_each(|pixel| {
        let (bytes, _) = pixel.channels().split_at(std::mem::size_of::<f32>());
        let channels_f32 = f32::from_ne_bytes(bytes.try_into().unwrap());
        *pixel = Rgba::from(channels_f32.ln().to_ne_bytes())
    });
    image.save(output_file)?;

    Ok(())
}
