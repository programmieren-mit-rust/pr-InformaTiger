# Image Search Library for PNGs (Rust)

This Image Search Library is a powerful and customizable image search library specifically developed for the Rust programming language. Its primary objective is to facilitate efficient image searches based on a provided input image. With this library, you can easily retrieve the most similar images from a pre-indexed pool or one of your own.

## Key Features:
- Perform image searches based on PNG images using Rust.
- Retrieve the most similar images from a pool of pre-indexed PNG images.
- Customize and expand the image pool according to your specific requirements and preferences.
- Benefit from efficient algorithms for accurate and fast image retrieval.

## Usage:
1. Add the Image Search Library as a dependency in your Rust project.
2. (Optional) Provide your own index-pool.
3. Provide the input PNG image for the search.
4. Configure the library according to your preferences, including the image pool and similarity criteria.
5. Execute the search using the library's functions or methods.
6. Access the results and utilize them in your Rust application.

Whether you need image search functionality for content-based image retrieval, recommendation systems, or any other PNG image-related tasks, this Image Search Library in Rust offers a customizable and efficient solution to meet your needs.

For detailed instructions on installation, usage examples, and customization options, please refer to the documentation provided with the library.
ImageSearchLib is a versatile library specifically developed for performing image searches based on a given image.
By providing a search term in the form of an image, the library efficiently retrieves the most similar images from a pool of indexed images.
The great advantage of this library lies in its expandability, allowing users to tailor the image pool to their specific needs.

The library provides the following central types:

    Images: This type represents images and can be used to store and manipulate PNG image data.
    Search Index: This type represents the search index, which is a collection of pre-indexed images.
    Features: This type represents the features extracted from images, along with their associated similarity measures.

Users of the library can use the pre-implemented features provided by the library or implement their own and use them with the search index.
Usage

To use the library, follow these steps:

    Import the library into your project.
    Create an instance of the SearchIndex class.
    Add images to the search index using the add_image method, providing the image data and associated features.
    To search for similar images, use the search method, providing the search image and specifying the similarity criteria.
    The search method will return a list of the most similar images based on the specified criteria.

Here's an example of how to use the library:

 cargo imsearch


### Create a search index
    search_index = SearchIndex()

### Add images to the search index
    image_data = Images.read_image('path/to/image.png')
    features = Features.extract(image_data)
    search_index.add_image(image_data, features)

### Search for similar images
    search_image = Images.read_image('path/to/search_image.png')
    similar_images = search_index.search(search_image, criteria='euclidean_distance', num_results=5)

### Display the similar images
    # TODO:

### Create a search index
    search_index = SearchIndex()

### Add images to the search index
    input()
    repeat_input()

### Search for similar images using the custom features
    picture_path = input_search_image()

Supported Similarity Criteria


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
This program is licensed by MIT.
