# Image Search Library for PNGs

This is a simple image search library designed to perform image searches based on a given image. The library takes a search term in the form of an image and outputs the most similar images from a pool of pre-indexed images. The criteria for similarity can be flexibly defined.
Features

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

python

from image_search import SearchIndex, Features, Images

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
for image in similar_images:
    Images.display_image(image)

Custom Features

If you want to use custom features, you can implement your own feature extraction methods and use them with the search index. Simply create a new class that inherits from the Features class and override the extract method to extract the desired features from the image data. Then, you can pass an instance of your custom features class to the add_image method of the search index.

python

from image_search import SearchIndex, Features, Images

class CustomFeatures(Features):
    def extract(self, image_data):
        # Custom feature extraction logic
        # Return the extracted features

# Create a search index
search_index = SearchIndex()

# Add images with custom features to the search index
image_data = Images.read_image('path/to/image.png')
custom_features = CustomFeatures.extract(image_data)
search_index.add_image(image_data, custom_features)

# Search for similar images using the custom features
search_image = Images.read_image('path/to/search_image.png')
similar_images = search_index.search(search_image, criteria='custom_features_similarity', num_results=5)

Supported Similarity Criteria

The library supports the following built-in similarity criteria:

    Euclidean Distance: Measures the similarity between two images based on the Euclidean distance between their feature vectors.
    Cosine Similarity: Measures the similarity between two images based on the cosine of the angle between their feature vectors.
    Custom Features Similarity: Allows you to define your own similarity measure when using custom features.

# License

You as library user can select between MIT and Apache 2.0.
