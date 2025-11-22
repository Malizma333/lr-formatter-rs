## Main Formats

### SOL

Original format used in [Beta 2 and other flash builds](https://github.com/kevansevans/Line-Rider-Flash-Builds-Decompiled-). The specification can be found [here](https://github.com/Malizma/lr-formatter-rs/blob/main/docs/sol.md).

### TRK

Created for [LRA](https://github.com/jealouscloud/linerider-advanced), with continued use in derivative builds ([LRA:CE](https://github.com/RatherBeLunar/LRA-Community-Edition), [LRTran](https://github.com/Tran-Foxxo/LRTran), [LRO](https://github.com/LunaKampling/LROverhaul)). A detailed specification can be found [here](https://github.com/Conqu3red/TRK-Docs/blob/master/The-TRK-Format.md).

### JSON (.track.json)

Created for linerider.com, with a modified version created in LRA for compatibility purposes. However, while the linerider.com writer received updates to the file structure, the LRA implementation did not catch up to these updates and added support for LRA-native features, diverting the initial format into almost two separate JSON formats. Documentation for the overall format can be found [here](https://github.com/Malizma/lr-formatter-rs/blob/main/docs/json.md).

## Other Formats (Unsupported)

### LRPK

Created with the intent to update the TRK format, and used in the [OpenLR](https://github.com/kevansevans/OpenLR) project. However, mainly focused on supporting flash features, and did not receive many updates beyond that. The specification can be found [here](https://github.com/kevansevans/OpenLR/wiki/The-LRPK-Format).

### BoshTF

Created for the [Line Rider Rust project](https://github.com/deanveloper/bosh) as a custom save format to serialize well with the internal structure. However, has not seen much use outside of that project. The serialization code can be found [here](https://github.com/deanveloper/bosh/blob/main/src-tauri/src/serialization/boshtf.rs).

### LRB

Created with the intent to update the TRK format, inspired by the modularity of LRPK's format. Support is currently being implemented in LRO, with the intention of universal support among all Line Rider builds. Proposals for the specification are being worked on [here](https://github.com/lrbspec). Will be supported if finished.
