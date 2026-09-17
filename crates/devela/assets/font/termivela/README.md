<!-- /devela/assets/font/termivela/README.md -->

# Termivela

Termivela is a fixed-metric monochrome bitmap-font family distributed
in DVBF format for use with devela.

Its initial release provides regular and bold 8×16 strikes derived from
Terminus Font 4.49.1 by Dimitar Toshkov Zhekov and contributors. The
initial transformation converts the upstream BDF bitmap strikes into
DVBF without intentionally changing glyph shapes or metrics.

Termivela uses a distinct family name because "Terminus Font"
is a Reserved Font Name under the SIL Open Font License.
The upstream authors do not endorse this derivative.

## Files

- `source.toml`: pinned upstream provenance, checksums and generated-output metadata.
- `OFL.txt`: the applicable SIL Open Font License and upstream copyright notice.
- `FONTLOG.txt`: the history of conversions and subsequent modifications.
- `termivela-8x16-regular.dvbf`: regular 8×16 face.
- `termivela-8x16-bold.dvbf`: bold 8×16 face.

## Representation

The currently included strikes use DVBF 0.1.0:
- one bit per pixel;
- fixed 8×16 glyph bitmaps;
- sorted Unicode scalar mapping;
- rows ordered from top to bottom;
- fixed horizontal and vertical metrics;
- pixels ordered most-significant bit first within each byte.

## Reproduction

The generated DVBF files are derived from the source archive
and BDF faces identified in `source.toml`.

The canonical conversion command will be documented here
when the devela BDF-to-DVBF converter is integrated.

## License

Termivela is distributed under the SIL Open Font License, Version 1.1.

See `OFL.txt` for the complete license, copyright information and Reserved Font Name declaration.

