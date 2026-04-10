# image-to-pixel-color
Useful for 3D pixelart. It takes in an PNG image and counts the pixel amount as well as calculates the number of rectangles one will need to make the 3D pixel art. This is outputted via a csv file

## How to use
Download the project and the run:
```
cargo run --release <PNGF_filepath> 
```

### Flags / Command-line arguments
#### -r | --resolution <resolution>
Downscales the inputed image into an image of size eg. 16x16. This is very useful if you have some sort of pixelart that you know is eg. 16x16 but the image is eg. 512x512.  
The downscaled image will apear in the directory ```downscaled_images```, showcasing the image the program counted the pixels.

#### -m | --max
Reduces the amount of colors in the image to the number specified. If the number provided is larger than the images color amount this flag will simply be ignored. This will also produce a showcase image located in the directory ```recolored_images```.

This will output a ```color_data.csv``` that contains the colors, their count and the 3D-requirement (explained below)

## 3D-requirement
### Formula
```3D-req = pixel_amount * 8 + numer_of_edges```

### Origin
Useful when using the technique specified in this video: https://youtu.be/U66X3PisXU4  
It gives you the amount of cut-outs sorted by color that you will need to make the entire pixel art.
