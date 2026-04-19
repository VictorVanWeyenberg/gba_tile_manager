use crate::error::Error;
use png::{BitDepth, ColorType};
use std::io::{BufRead, Seek};

pub fn read_to_rgb_255<R: BufRead + Seek>(reader: &mut png::Reader<R>) -> Result<Vec<u8>, Error> {
    let info = reader.info();
    let color_type = info.color_type;
    let bit_depth = info.bit_depth;
    match color_type {
        ColorType::Rgb => read_rgb(reader),
        ColorType::Indexed => read_indexed(reader, bit_depth),
        ColorType::Rgba => read_rgba(reader),
        _ => todo!("{:?}", color_type)
    }
}

fn read_rgb<R: BufRead + Seek>(reader: &mut png::Reader<R>) -> Result<Vec<u8>, Error> {
    Ok(read_8_bit(reader)?)
}

fn read_rgba<R: BufRead + Seek>(reader: &mut png::Reader<R>) -> Result<Vec<u8>, Error> {
    Ok(read_8_bit(reader)?.into_iter().enumerate()
        .filter_map(|(i, c)| if (i + 1) % 4 == 0 { None } else { Some(c) })
        .collect())
}

fn read_indexed<R: BufRead + Seek>(reader: &mut png::Reader<R>, bit_depth: BitDepth) -> Result<Vec<u8>, Error> {
    let palette = reader.info().palette.clone().unwrap();
    Ok(read_bit_depth(reader, bit_depth)?.into_iter()
        .map(|idx| idx as usize)
        .map(|idx| &palette[idx*3..idx*3+3])
        .flatten()
        .cloned()
        .collect())
}

fn read_8_bit<R: BufRead + Seek>(reader: &mut png::Reader<R>) -> Result<Vec<u8>, Error> {
    let mut buf = vec![0; reader.output_buffer_size().unwrap()];
    reader.next_frame(&mut buf)?;
    Ok(buf)
}

fn read_4_bit<R: BufRead + Seek>(reader: &mut png::Reader<R>) -> Result<Vec<u8>, Error> {
    let mut buf = vec![0; reader.output_buffer_size().unwrap()];
    reader.next_frame(&mut buf)?;
    Ok(buf.into_iter()
        .map(|i| [i & 0x0f, i >> 4])
        .flatten()
        .collect())
}

fn read_bit_depth<R: BufRead + Seek>(reader: &mut png::Reader<R>, bit_depth: BitDepth) -> Result<Vec<u8>, Error> {
    match bit_depth {
        BitDepth::Four => read_4_bit(reader),
        BitDepth::Eight => read_8_bit(reader),
        _ => todo!("{:?}", bit_depth)
    }
}



