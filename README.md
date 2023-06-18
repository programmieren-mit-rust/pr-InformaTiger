# Image Search Library for PNGs (Rust)

This Image Search Library is a useful and customizable image search library developed in Rust. Its primary objective is to facilitate efficient image searches based on a provided input image. With this library, you can easily retrieve the most similar images from a pre-indexed pool or one of your own.

Key Features:
- Perform image searches based on PNG images using Rust.
- Retrieve the most similar images from a pool of pre-indexed PNG images.
- Customize and expand the image pool according to your specific requirements and preferences.
- Benefit from efficient algorithms for accurate and fast image retrieval.

## Usage:

```rust
fn main() {
    
    get_pictures_from_user();

    //Input User: SearchImage
    let picture_path = input_search_image();

    let pic_u8: PictureU8 = read_picture(&picture_path);
    println!("PictureU8: {}", pic_u8);

    let histograms = get_histogram(&pic_u8);
    print_all_diagrams(histograms);
        
    // Provide a path to a picture to get the similarity scores
    let similar_five_pictures = get_top_five_similar_pictures("path/to/picture.png").unwrap();
    print_calculated_similar_pictures(similar_five_pictures);
}
```

For detailed instructions on installation, usage examples, and customization options, please refer to the documentation provided within the library.

## Using different data types for `data`
The functionalities of this crate are not (yet) generic.
You can, however, use your own `PictureX##` structs that use types for `data` other than `u8` or `f32` by implementing the `Picture` trait.

```rust
pub struct PictureU32 {
    pub lines: u32,   //height
    pub columns: u32, //width
    pub color_channel_count: usize,
    pub data: Vec<u32>, //different data type: u32
}
```

This means you have to map all values of your type to `[0, 255]` for `PictureU8` and `[0.0, 1.0]` for `PictureF32`.

```rust
impl Picture for PictureU32 {
    fn to_picture_u8(&self) -> PictureU8 {
        let mut new_data = Vec::<u8>::new();

        //convert each value from [0, u32::MAX] to [0, 255]
        for i in 0..self.data.len() {
            //
            let new_value_in_f32 = ((self.data[i] as f32) / (u32::MAX as f32)) * 255.0;
            new_data.push(new_value_in_f32 as u8);
        }

        PictureU8 {
            lines: self.lines,
            columns: self.columns,
            color_channel_count: self.color_channel_count,
            data: new_data,
        }
    }

    fn to_picture_f32(&self) -> PictureF32 {
        let mut new_data = Vec::<f32>::new();

        //convert each value from [0, u32::MAX] to [0.0, 1.0]
        for i in 0..self.data.len() {
            new_data.push((self.data[i] as f32) / (u32::MAX as f32));
        }

        PictureF32 {
            lines: self.lines,
            columns: self.columns,
            color_channel_count: self.color_channel_count,
            data: new_data,
        }
    }
}
```

You can now use your struct just like the built-in `PictureF32` and `PictureU8` types.
Keep in mind that they will be treated as those types:

```rust
fn main() {
    // You'll have to figure out how to read `PictureU32`s
    let pic_u32 = PictureU32 {
        lines: 1,
        columns: 3,
        color_channel_count: 3,
        data: vec![
            123_456,
            128,
            0,
            210_000_000,
            0,
            0,
            456_234,
            90_000,
            2_123_000_333,
        ],
    };

    print_all_diagrams(get_histogram(&pic_u32));
}
```

# License
See [License file](LICENSE-MIT).
