# Image Search Library for PNGs (Rust)

This Image Search Library is a powerful and customizable image search library specifically developed for the Rust programming language. Its primary objective is to facilitate efficient image searches based on a provided input image. With this library, you can easily retrieve the most similar images from a pre-indexed pool or one of your own.

### Key Features:
- Perform image searches based on PNG images using Rust.
- Retrieve the most similar images from a pool of pre-indexed PNG images.
-Customize and expand the image pool according to your specific requirements and preferences.
- Benefit from efficient algorithms for accurate and fast image retrieval.

## Usage:
1. Add the Image Search Library as a dependency in your Rust project.
2. (Optional) Provide your own index-pool.
2. Provide the input PNG image for the search.
3. Configure the library according to your preferences, including the image pool and similarity criteria.
4. Execute the search using the library's functions or methods.
5. Access the results and utilize them in your Rust application.

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


# Create a search index
    search_index = SearchIndex()

# Add images to the search index
    image_data = Images.read_image('path/to/image.png')
    features = Features.extract(image_data)
    search_index.add_image(image_data, features)

# Search for similar images
    search_image = Images.read_image('path/to/search_image.png')
    similar_images = search_index.search(search_image, criteria='euclidean_distance', num_results=5)




# Display the similar images
    # TODO:

# Create a search index
    search_index = SearchIndex()

# Add images to the search index
    input()
    repeat_input()


# Search for similar images using the custom features
    picture_path = input_search_image()

Supported Similarity Criteria



# License
This program is licensed by MIT.
