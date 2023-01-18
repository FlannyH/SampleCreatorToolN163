# SampleCreatorToolN163
Very scuffed. Edit these values to get the result you want
```
    const samples_per_loop: usize = 169;  // How many samples should be squashed into a single wavetable
    let skip_first_samples = 0; // How many samples to ignore from the start
    let sample_stride = 2; // How many "loops" you wanna skip inbetween wavetables (do you want the sample to advance faster? increase this number)
    let output_sample_size = 64; // N163 wavetable size. Change this to 32 for Game Boy
    let output_sample_pos = 0; // N163 wavetable position. Doesn't matter for Game Boy
    let name_string = "Output File"; // Name of the instrument. Change this to whatever you want
```
