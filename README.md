# mdbook-image-slider[中文](./README_ZH.md)
## Installation

Add it as a preprocessor to your `book.toml`:

```toml
[preprocessor.image-slider]
command = "mdbook-image-slider"
renderer = ["html"]
```

## Usage

```
{{image-slider}}
image: http://vinciyan.com/image-public/image-20240831215150-edx0d4e.png caption: Picture 1 Description
image: http://vinciyan.com/image-public/0a41dd35-7079-44d1-93f8-995a99d011b0.png caption: Picture 2 Description
image: http://vinciyan.com/image-public/0a1e65f4-7ced-4ef0-ba7d-120014a0d4.png caption: Picture 3 Description
{{/image-slider}}
```
