# JSON (.track.json) Format

```js
{
  // Shared properties
  "label"?: string,
  "version": "6.0" | "6.1" | "6.2",
  "startPosition"?: { "x": f64, "y": f64 },
  "lineArray"?: [ // Legacy line array format, not in current web version
    [
      [0, u32, f64, f64, f64, f64, 0 | 1 | 2 | 3, bool],
      [1, u32, f64, f64, f64, f64, 0 | 1 | 2 | 3, bool, -1, -1, f64],
      [2, -u32, f64, f64, f64, f64],
    ],
    ...
  ],

  // Web-only properties
  "creator"?: string,
  "description"?: string,
  "script"?: string,
  "duration"?: u32,
  "riders": [
    {
      "startPosition": { "x": f64, "y": f64 },
      "startVelocity": { "x": f64, "y": f64 },
      "startAngle"?: f64,
      // true/false for remount rider v1, 0/1 for remount rider v2
      "remountable"?: 0 | 1 | false | true,
    },
    ...
  ],
  "layers"?: [
    {
      // "type" and "folderId" were undefined until folders existed
      // "editable" was a feature added later
      "id": u32,
      "type"?: 0,
      "name": string,
      "visible": boolean,
      "editable"?: boolean,
      "folderId"?: -1 | u32,
    },
    {
      "id": u32,
      "type": 1,
      "name": string,
      "visible": boolean,
      "editable": boolean,
      "size": u32,
    },
    ...
  ],
  "lines"?: [
    {
      "id": u32,
      "type": 0,
      "x1": f64,
      "y1": f64,
      "x2": f64,
      "y2": f64,
      "flipped": boolean | 0 | 1,
      // Either leftExtended and rightExtended are present, or extended is present
      "leftExtended"?: boolean | 0 | 1,
      "rightExtended"?: boolean | 0 | 1,
      "extended"?: 0 | 1 | 2 | 3,
      "layer"?: u32,
    },
    {
      "id": u32,
      "type": 1,
      "x1": f64,
      "y1": f64,
      "x2": f64,
      "y2": f64,
      "flipped": boolean | 0 | 1,
      // Either leftExtended and rightExtended are present, or extended is present
      "leftExtended"?: boolean | 0 | 1,
      "rightExtended"?: boolean | 0 | 1,
      "extended"?: 0 | 1 | 2 | 3,
      "multiplier"?: f64,
      "layer"?: u32,
    },
    {
      "id": u32,
      "type": 2,
      "x1": f64,
      "y1": f64,
      "x2": f64,
      "y2": f64,
      "width"?: f64,
      "layer"?: u32,
    },
    ...
  ],

  // LRA+ only properties
  "startZoom"?: f32 > 0,
  "zeroStart"?: boolean,
  "bgR"?: u8,
  "bgG"?: u8,
  "bgB"?: u8,
  "lineR"?: u8,
  "lineG"?: u8,
  "lineB"?: u8,
  "xGravity"?: f32,
  "yGravity"?: f32,
  "gravityWellSize"?: f64,
  "lines"?: null,
  "triggers"?: [ // Legacy line-based triggers, not in LRO
    {
      "zoom": boolean,
      "ID": u32,
      "target": f32 > 0,
      "frames": u32,
    },
    ...
  ],
  "gameTriggers"?: [
    {
      "start": u32,
      "end": u32,
      "triggerType": 0,
      "zoomTarget": f32 > 0,
      "backgroundred": -999,
      "backgroundgreen": -999,
      "backgroundblue": -999,
      "lineRed": -999,
      "lineGreen": -999,
      "lineBlue": -999,
    },
    {
      "start": u32,
      "end": u32,
      "triggerType": 1,
      "zoomTarget": -999,
      "backgroundred": u8,
      "backgroundgreen": u8,
      "backgroundblue": u8,
      "lineRed": -999,
      "lineGreen": -999,
      "lineBlue": -999,
    },
    {
      "start": u32,
      "end": u32,
      "triggerType": 2,
      "zoomTarget": -999,
      "backgroundred": -999,
      "backgroundgreen": -999,
      "backgroundblue": -999,
      "lineRed": u8,
      "lineGreen": u8,
      "lineBlue": u8,
    },
  ],
}
```
