# image-to-pixel-color
Useful for 3D pixelart, takes in an PNG image and counts the pixel amount as well as calculated the number of rectangles one will  
need to make the 3D pixel art. This is outputted via a csv file

## How to use
Download the project and the run:
```
cargo run --release <PNGF_filepath> 
```

This will output a ```color_data.csv``` that contains the colors, their count and the 3D-requirement (explained below)

## Bugs
- not tested for anything besides PNG files


## Coming features
- multiple file formats
- color blend parameter
- export as .xlsx file

## 3D-requirement
### Formula
```3D-req = pixel_amount * 8 + numer_of_edges```

### Origin
Useful when using the tecnique specified in this video: https://youtu.be/U66X3PisXU4  
It gives you the amount of cut-outs sorted by color that you will need to make the entire pixel art.
