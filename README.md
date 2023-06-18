# Image Search Library for PNGs (Rust)

This Image Search Library is a useful and customizable image search library developed in Rust. Its primary objective is to facilitate efficient image searches based on a provided input image. With this library, you can easily retrieve the most similar images from a pre-indexed pool or one of your own.

Key Features:
- Perform image searches based on PNG images.
- Retrieve the most similar images from a pool of pre-indexed PNG images.
- Print histograms of pictures

## Usage:

```rust
fn main() {

    // Asking the user to add elements to the picture library.
    // Later you can compare pictures to the library which was provided.
    get_pictures_from_user();

    // Asking the user for a path to a picture.
    // This picture will later be compared with the library.
    // It is not written to the library itself!!
    let picture_path = input_search_image();

    // Some additional features, which can be used.
    println!("Information on the picture you provided:");

    // Determine the average brightness of a picture.
    let average_brightness = get_average_brightness_of_picture(&picture_path);
    println!("Averagebrightness: {}", average_brightness);

    //read a picture in the U8 picture-format
    let pic_u8: PictureU8 = read_picture(&picture_path);
    println!("PictureU8: {}", pic_u8);

    // Calculate color histograms for a picture.
    let histograms = get_histogram(&pic_u8);
    // Histograms can also be printed to the console.
    print_all_diagrams(histograms);

    // Main usage of the library.
    // Compare a picture to the pictureLibrary (also user-generated).
    // It returns the 5 most similar pictures.
    let similar_five_pictures = get_top_five_similar_pictures(picture_path.clone().as_str()).unwrap();
    //The most similar pictures can be printed to the console.
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
