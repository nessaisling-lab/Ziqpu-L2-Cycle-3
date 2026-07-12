# Ziqpu brand assets

Ziqpu inherits the **Nisaba "Oracle"** design system as a sub-brand. Voice: scholarly,
sourced, anti-hype — *"measured, not believed."*

- `tokens.css` — color + type + spacing tokens for both themes (light "Sunlit Clay",
  dark "Gold on Bitumen").

## Fonts

The desktop UI ships three **SIL OFL 1.1** faces, base64-embedded as `@font-face` in
`crates/ui/assets/ziqpu.css` (self-hosted, never CDN): **Cinzel Decorative** (display),
**Spectral** (body), and **Noto Sans Cuneiform** (glyph accents). JetBrains Mono is used for
data/monospace as a **system fallback** (not bundled). The OFL text + per-font copyright /
Reserved Font Name notices are in [LICENSES/OFL-1.1.txt](../LICENSES/OFL-1.1.txt).

> Pages carrying cuneiform include `<meta name="google" content="notranslate">` so browsers
> don't mis-detect the glyphs as a modern language.

## Marks

The **Dingir** (8-pointed divine star) and **Nanibgal** (divine sheaf) SVG marks use
`stroke="currentColor"` so they recolor via CSS. They are added alongside the UI.
