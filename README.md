# Readme
This is a simple project to test the image crate for Rust.Currently only takes jpg safely, so have a jpg selected for use.
## Arguments
1. source image path 
2. output image path (optional)
   3. if one is not provided it will use the same directory as the source image with "NEW_" prepended to the file name.
### example
`cargo run -- "C:\\Users\\Me\\Downloads\\input_image.jpg" "Z:\\output_image.jpg"`