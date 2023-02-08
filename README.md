# rust-raytracing

A simple raytracer written in Rust as an excuse to learn Rust.

Mostly adapted from [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

## How to run


### With Gui

```
cargo run -r
```

### Witout Gui

```
cargo run -r -- --no-gui --output <FILE> --scene <SCENE>
```

## Examples

![example render](random.png)

![Perlin noise](perlin.png)

![Textures](earth.jpg)

![Lights](simple_light.png)

![Cornell box](cornell_box.png)

## Resources

Earth map from https://pxhere.com/en/photo/1025037