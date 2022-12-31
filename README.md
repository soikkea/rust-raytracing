# rust-raytracing

A simple raytracer written in Rust as an excuse to learn Rust.

Mostly adapted from [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

## How to run

The rendering parameters are defined in main.rs. You can give output file name as an argument, for example:

```
>cargo run -r -- image.png
```

## Examples

![example render](random.png)

![Perlin noise](perlin.png)

![Textures](earth.jpg)

![Lights](simple_light.png)

![Cornell box](cornell_box.png)

## Resources

Earth map from https://pxhere.com/en/photo/1025037