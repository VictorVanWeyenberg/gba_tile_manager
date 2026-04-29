# GBA Tile Manager

This tool converts PNGs to GBA VRAM interpretable binary data. An example GBATM project can be found in the [resources](./resources) directory.

## config.json

The `config.json` tells GBATM which files to use and which files depend on each other. The structure looks like this.

```json
{
  "screens": [
    // -- snip --  Refer to [Screens](## Screens) below.
  ],
  "boops": [
    // -- snip --  Refer to [Boops](## Boops) below.
  ]
}
```

## Screens

A screen defenition in the config.json file has three fields. Each fields must point to a PNG image file. The requirements of these files are described below.

These files are grouped because:

- In order to derive screen information from a png we require its tile set.
- In order to derive tile information from a png we require its palette.

> We chose not to use the PNG PLTE chunk for this because optimizing your PNGs could lead to removed colors you might need at runtime.

```json
{
  "palette": "PNG image",
  "character": "PNG image",
  "screen": "PNG image"
}
```

### Palette PNG

This image must be 16x16 pixels in size. All colors are interpreted in 5bpc.

### Character PNG

This image must be 256x256 in size and may only be drawn in color that are present in the palette image. A tile (character) is 8x8 pixels so you can allocate 1024 tiles in one image. This is meant for [BG Mode 0,1 Tile Maps](https://www.akkit.org/info/gbatek.htm#lcdvramoverview).

### Screen PNG

This image must be 256x256 in size and may only be drawn in color that are present in the palette image. This is meant for a [BG Control Screen Size of 0](https://www.akkit.org/info/gbatek.htm#lcdiobgcontrol).

## Boops

The `boops` field in the `config.json` file is a list or csv file references. The binary data exported from these inputs is not interpretable by the GBA; this is a self-made format.

An example CSV file is included in the [resources](./resources) directory.

The binary output is two files:

- \<boops file name\>_boops.bin: The binary data of the boops. Structure described below.
- \<boops file name\>_boops_args.bin: All args from the boops CSV file consecutively in one file.

```plaintext
Bit (LE)    Expl.
00-07       Boop X in pixels - top left corner - from the CSV file.
08-15       Boop Y in pixels - top left corner - from the CSV file.
16-23       Boop width in pixels - from the CSV file.
24-31       Boop height in pixels - from the CSV file.
32-39       The boop north from here. 255 if there is none
40-47       The boop east from here. 255 if there is none
48-55       The boop south from here. 255 if there is none
56-63       The boop west from here. 255 if there is none
64-71       Boop callback index from the CSV file.
72-79       Boop args index. The index in the args file where the boop's args are located.
80-87       Boop args len. The amount of args from the CSV file.
```

# Example

```
cargo build --release
./target/release/gba_tile_manager --help
./target/release/gba_tile_manager -i resources/ -o bin/ -f
```
