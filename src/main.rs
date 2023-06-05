#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod tests;

use imsearch::{
    read_picture_u8, PictureF32, PictureU8
};

fn main() {
    let pic_u8: PictureU8 = read_picture_u8("Bilder Programmentwurf-20230603/bird.png");
    println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    let pic_f32: PictureF32 = pic_u8.to_picture_f32();
    println!("PictureF32: {pic_f32}");
}
