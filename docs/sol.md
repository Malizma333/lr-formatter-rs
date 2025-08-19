# SOL Format

- Uses the [Action Message Format v0](https://rtmp.veriskope.com/pdf/amf0-file-format-specification.pdf) for the actual data
- Assume all values are BigEndian per the AMF0 spec
- This format specification does not detail every flash mod, only Beta 2 v6.0 - v6.2 and LRA

# Header

- **0x00:** Magic number 0x00BF, version of SOL
- **0x02:** Unsigned 32 bit integer, number of bytes that follow
- **0x06:** Tag 0x5443534F, spelling out "TCSO"
- **0x0A:** Marker 0x000400000000
- **0x10:** Unsigned 16 bit integer, SOL name length (0x0A)
- **0x12:** SOL name "savedLines"
- **0x1C:** Padding 0x00000000
- **0x20:** Unsigned 16 bit integer, data name length (0x09)
- **0x22:** Data name "trackList"

# Data

- `label`
  - Title of the track object
- `version`
  - Grid version of the track
  - Defaults to 6.0, which is the version before the "version" field was added
- `startLine`
  - The [x, y] values denoting start position
  - Written as an Object instead of ECMAArray in the LRA writer
- `level`
  - The total number of lines
- `data`
  - The list of track lines, as line arrays: [x1, y1, x2, y2, ext, flipped, prevLineId, nextLineId, id, type]
  - `flipped`
    - A number in the beta 2 writer, but a boolean in the LRA writer
    - 0/false for green lines
  - `ext`
    - Bit flags representing line extensions: 0=None, 1=Left, 2=Right, 3=Both
    - 0 for green lines
    - Undefined in 6.7
  - `type`
    - 0 for blue lines, 1 for red lines, and 2 for green lines
  - `prevLineId`
    - Id of the previous line, usually ignored by most writers
  - `nextLineId`
    - Id of the next line, usually ignored by most writers
- `trackData`
  - A property added by LRA that (at present) only describes whether zero start is enabled

```
[
  {
    label: String,
    version?: String,
    startLine: [Number (f64), Number (f64)],
    level: Number (u32),
    trackData: [
      null,
      [null, null, null],
      [null, null, null, null, true]
    ],
    data: [
      [
        Number (f64),
        Number (f64),
        Number (f64),
        Number (f64),
        Number (0 - 3) | Undefined,
        Number (0 - 1) | Boolean,
        Number (u32),
        Number (u32),
        Number (u32),
        Number (0 - 2)
      ],
      ...previous lines
    ],
  },
  ...previous tracks
]
```
