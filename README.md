# Image resize

This script will resize all images in a folder to be less than 1MB in compressed size (hopefully).


# Usage

```sh
imres /path/to/folder
```

The specified image folder can also include subfolders, which may contain additional images.

## How is it downsized
For the purpose of speed it is very straightforward
- Compute the ratio of (compressed image size / 1MB)
- Downsize the dimensions of the image by sqrt of the ratio

This approach does not guarantee that the image will be 100% less than 1MB but it never failed me (over 100k images were a success).
Please note that the script doesn't optimize the new dimensions of the image, so the resized image might be smaller than necessary.


## Output format
All images will be converted to JPEG. PNGs with no background will be converted to JPEG with white background.


## EXIF
The resized image will be rotated based on the EXIF orientation information, but will not have any EXIF metadata after processing.
