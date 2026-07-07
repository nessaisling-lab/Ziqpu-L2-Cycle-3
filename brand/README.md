# Ziqpu brand assets

Ziqpu inherits the **Nisaba "Oracle"** design system as a sub-brand. Voice: scholarly,
sourced, anti-hype — *"measured, not believed."*

- `tokens.css` — color + type + spacing tokens for both themes (light "Sunlit Clay",
  dark "Gold on Bitumen").

## Fonts (added in Phase 4)

Self-host these under `brand/fonts/` (all SIL Open Font License 1.1):
Cinzel Decorative (display), Spectral (body), JetBrains Mono (data), and
**Noto Sans Cuneiform** for glyph accents.

> Noto Sans Cuneiform must be **self-hosted, never CDN**, and pages carrying cuneiform
> should include `<meta name="google" content="notranslate">` so browsers don't mis-detect
> the glyphs. Font binaries are not committed at Genesis; they land with the UI in Phase 4.

## Marks

The **Dingir** (8-pointed divine star) and **Nanibgal** (divine sheaf) SVG marks use
`stroke="currentColor"` so they recolor via CSS. They are added alongside the UI.
