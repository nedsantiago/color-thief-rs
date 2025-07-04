# Color Thief (Rust)

A rewrite of the color thief library in Rust. The library builds a color palette in-theme with a provided image. This library is a Rust port of Shipeng Feng's [Color Thief](https://github.com/fengsp/color-thief-py)

## Motivation

Primarily, to improve in writing Rust as project-based learning. Color Thief is a library that `wal` was built from.

## Notes

- Considering a separating of ColorSpaces from MMCQ. `ColorSpace` should be the data model holding information but not changing it. `MMCQ` is the calculating engine binning and categorizing the colors. Separating these concerns should improve testing through modularity.
- Considering a better data model for pixels. It would be best to pass and calculate pixels as a set or vector instead of repeating operations thrice at different parts of the algorithm.
- Will need to unify the naming and terminology used for : `ColorSpace` / `VolumeBox` / `ColorBox` / `Color3D`, `MMCQ`, `Rgba`, `hash` / `id` / `index` / `hashed_color`, etc.
- In `color-thief-py`, `VBox.count()` can be optimized I by looping through the dictionary instead of the entire color space.

## Possible tests

```py
# Use this as basis for a Rust test
histo = {26380: 11, 1057: 1, 0: 1, 25166: 10, 20041: 206, 22122: 2813, 21958: 28, 10530: 48, 14693: 24, 32767:1}
vbox = colorthief.VBox(0,31,0,31,0,31, histo)
assert vbox.count == 3143
```

## Planned Library structure

The library follows a 7-stage data pipeline.

1. **Image to Pixel**: Given a directory to an image, the Color Thief will read its data and create an iterable pixels in rgba format.
2. **Pixel Validity Filter**: Invalid pixels will be filtered out of the data.
3. **Pixels to Color Summary**: Bin the pixels and record the minimum and maximum values of each rgb pixel
4. **Median Split Color Summary by frequency**: sort hash colors by their count and median split based on the accumulated count (true median: split by count)
5. **Median Split Color Summary by volume-count**: sort by `volume * count` and median split
6. **Calculate average color of each Color Summary**: Gather the average color for each Color Summary
7. **Create a color palette**: Create a list of colors based on the average colors selection here.
