# Color Thief (Rust)

A rewrite of the color thief library in Rust. The library builds a color palette in-theme with a provided image. This library is a Rust port of Shipeng Feng's [Color Thief](https://github.com/fengsp/color-thief-py)

## Motivation

Primarily, to improve in writing Rust as project-based learning. Color Thief is a library that `wal` was built from.

## Notes

- Considering a separating of ColorSpaces from MMCQ. `ColorSpace` should be the data model holding information but not changing it. `MMCQ` is the calculating engine binning and categorizing the colors. Separating these concerns should improve testing through modularity.
- Considering a better data model for pixels. It would be best to pass and calculate pixels as a set or vector instead of repeating operations thrice at different parts of the algorithm.
- Will need to unify the naming and terminology used for : `ColorSpace`, `MMCQ`, `Rgba`, `hash` / `id` / `index` / `hashed_color`, etc.
