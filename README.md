# [WIP] Color Thief with Rust

![GitHub Actions Build Status](https://github.com/nedsantiago/color-thief-rs/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MPL_2.0-red.svg)](https://www.mozilla.org/en-US/MPL/2.0/)

**This project is a work in progress.**

A Rust rewrite of `color-thief-py`. Color Thief generates a color palette based on a provided image. This repository is a Rust port of Shipeng Feng's [color-thief-py](https://github.com/fengsp/color-thief-py)


### References

Feng, S. (2017, February 9). *color-thief-py*. Github. [https://github.com/fengsp/color-thief-py](https://github.com/fengsp/color-thief-py)

Dhakar, L. (2024, October 5). *quantize*. Github. [https://github.com/lokesh/quantize](https://github.com/lokesh/quantize)

## Library Structure

### Data Models

```mermaid
classDiagram
    class Rgba {
        +array[T;4] 0
    }
    class RgbaImage {
        +Vec[Rgba]
    }
    class MinMaxBox {
        +u8 rmin
        +u8 rmax
        +u8 gmin
        +u8 gmax
        +u8 bmin
        +u8 bmax
    }
    class FrequencyMap {
        +HashMap[u32,u32] 0
    }
    class DimHistograms {
        +Vec[Histogram, 3] 0
    }
    class Histogram {
        +Vec[u32, 4] 0
    }
    class BoxQueue {
        +Vec[MinMaxBox] 0
    }
    class ColorPalette {
        +Vec[Rgba] 0
    }
    BoxQueue --> MinMaxBox: uses
    RgbaImage --> Rgba: uses
    DimHistograms --> Histogram: uses
```
- `calc_minmax_freq_histo` will also bin the pixels based through pixel shifting.
- Between `load_img` and `calc_minmax_freq_histo`, algorithm should check for pixel validity.
- `histogram` may need `inverse_histogram` and `volume_count_histogram` counterparts
- Need more details for `calc_nearest_colors`, `calc_average_colors`, `twophase_split`, and `iterative_split`
- `sort_boxes` should sort the BoxQueues before usage
- `split_box` should be implemented inside `iterative_split` and `twophase_split`

### Process Flowchart

```mermaid
flowchart TD;
    START([START]) --> img_path[/img_path: String/]
    img_path --> load_img[load_img]
    load_img --> rgba_img[/img: RgbaImage/]
    rgba_img --> calc_minmax_box[calc_minmax_box]
    rgba_img --> calc_frequency_map[calc_frequency_map]
    rgba_img --> calc_histogram[calc_histogram]
    calc_minmax_box --> init_minmax_box[/init_minmax_box: MinMaxBox/]
    calc_frequency_map --> frequency_map[/frequency_map: FrequencyMap/]
    calc_histogram --> histogram[/histogram: Histogram/]
    init_minmax_box --> iterative_split[iterative_split]
    histogram --> iterative_split
    iterative_split --> boxes_itersplit[/boxes_itersplit: BoxQueue/]
    boxes_itersplit --> two_phase_split[two_phase_split]
    two_phase_split --> boxes_two_phase[/boxes_two_phase: BoxQueue/]
    boxes_two_phase --> calc_average_colors[calc_average_colors]
    calc_average_colors --> average_colors[/average_colors: ColorPalette/]
    average_colors --> calc_nearest_colors[/calc_nearest_colors/]
    calc_nearest_colors --> nearest_colors[nearest_colors: ColorPalette]
    nearest_colors --> END([END])
```
