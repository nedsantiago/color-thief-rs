## Notes

- Considering a separating of ColorSpaces from MMCQ. `ColorSpace` should be the data model holding information but not changing it. `MMCQ` is the calculating engine binning and categorizing the colors. Separating these concerns should improve testing through modularity.
- Considering a better data model for pixels. It would be best to pass and calculate pixels as a set or vector instead of repeating operations thrice at different parts of the algorithm. Will need to unify the naming and terminology used for : `ColorSpace` / `VolumeBox` / `ColorBox` / `Color3D`, `MMCQ`, `Rgba`, `hash` / `id` / `index` / `hashed_color`, etc.
- In `color-thief-py`, `VBox.count()` can be optimized I by looping through the dictionary instead of the entire color space.
- Current architecture can be improved. Exploring algorithms that can encapsulate the creation of the `ColorSpace` / `VBox` structs. Suspect that the `histo`-generating function and the `ColorSpace` algorithm should be used in a single function. Perhaps `color_calc` can be composed of functions declared somewhere else. Especially important since a frequency calculator seems like a valuable algorithm to have for future projects.
- Create a png without data for testing, there may be a weird case where the while loop may go on until max iteration. In `color-thief-py` line 241, it seems to do nothing when the vbox count is 0 then increments `n_iter` and continues the while loop until max iteration. I think think the program should cite this as a failure mode.
- The Priority Queue appears to sort each time the data changes but I wonder if the sorting is useful for the MMCQ algorithm. For most of it, it seems to only use the maximum value. May be better to only get max value then later run the full sort algorithm when getting the color palette.
- In contrast to the Priority Queue, it seems that getting the median will indeed need a sorting of some kind.
- According to Mozilla web docs using `~~` in Javascript is outdated practice [mdn web docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_NOT), better to use `Math.trunc()`. Thus, this [line](https://github.com/lokesh/quantize/blob/master/src/quantize.js#L488) from color thief could be improved.
- I suspect that this [line](https://github.com/fengsp/color-thief-py/blob/master/colorthief.py#L199) may have an issue in cases where the median was found at the max value of the color range and will need to walk backwards *but* finds that the value immediately below it is `0` or `None`. In that case, it might make a right side that is empty.
- Replace `ColorSpace` with a `RGBBox` with only the minimums and maximums. A separate `FrequencyMap`  and `Histogram`

## Possible tests

```py
# Use this as basis for a Rust test
histo = {26380: 11, 1057: 1, 0: 1, 25166: 10, 20041: 206, 22122: 2813, 21958: 28, 10530: 48, 14693: 24, 32767:1}
vbox = colorthief.VBox(0,31,0,31,0,31, histo)
assert vbox.count == 3143
```

## Planned Library structure

The library follows a 7-stage data pipeline.

### Flowchart
```mermaid
flowchart TD;
    A[Start] --> B[Image];
    B --> C{Is valid?};
    C --> |No| D[Exit with error];
    C --> |Yes, convert| E[RGBA Pixels];
    E --> |Filter pixles| F[RGB Pixels];
    F --> |Bin + min & max| G[ColorSpace];
    F --> |Frequency Analysis| H[Histogram];
    G --> |Iterative Splitting| I[Vector of Colorspaces];
    H --> I;
    I --> |Two-phase Splitting| J[Vector of Colorspaces];
    J --> |Average Color| K[Color Map];
    K --> |Nearest Color| L[Color Palette];
    L --> M[End];
```

1. **Image to Pixel**: Given a directory to an image, the Color Thief will read its data and create an iterable pixels in rgba format.
2. **Pixel Validity Filter**: Invalid pixels will be filtered out of the data.
3. **Pixels to Color Summary**: Bin the pixels and record the minimum and maximum values of each rgb pixel
4. **Median Split Color Summary by frequency**: sort hash colors by their count and median split based on the accumulated count (true median: split by count)
5. **Median Split Color Summary by volume-count**: sort by `volume * count` and median split
6. **Calculate average color of each Color Summary**: Gather the average color for each Color Summary
7. **Create a color palette**: Create a list of colors based on the average colors selection here

**Modified Median Cut Quantization (MMCQ) Algorithm Explanation**

1. **Create a 3D Color Space** - Each dimension represents one color channel (red, green, blue), i.e. represent colors as a 3D coordinate system.
2. **Color Histogram** - Bin the bits by the first 5 significant bits, then count the number of pixels in each binned color. In other words, reduce the number of colors by collecting them using their binary place numbers.
3. **Initial ColorSpace** - Encompasses all colors in histogram.
4. **Iterative Splitting** - Select largest box by count, find longest dimension, find median along dimension, split box at the median
5. **Two-phase Splitting** - Split by pixel `count` until 75% target colors, then split based on `count * volume`.
6. **Map Colors** - based on the average color per box
7. **Find nearest color** - Colors not in palette can try to find the nearest.

**Need to Implement**
- 3D box - `RGBBox`
- Color Map (hash table) - `FrequencyMap` and 
- Priority Queue (or sorting algorithm) - `sort_values`
- Histogram - `Histogram`
- Initial 3D Box builder - `make_min_max_box`
- Median Cut Algorithm - `get_median()`
- Orchestration function - `make_color_palette()`
