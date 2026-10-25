---
name: Theseus
description: C parsers to memory-safe Rust, with evidence a reviewer can rerun; every mark printed from two plates.
colors:
  ink: "#151411"
  panel: "#1c1b17"
  line: "#2f2d27"
  dim: "#948e80"
  paper: "#ece7dc"
  clang: "#74b3ff"
  rust: "#ff7a3a"
  match: "#a9d7a8"
  div: "#ff5a3c"
  crash: "#f2b84b"
typography:
  display:
    fontFamily: "Schibsted Grotesk, ui-sans-serif, system-ui, sans-serif"
    fontSize: "clamp(44px, 7vw, 112px)"
    fontWeight: 800
    lineHeight: 0.94
    letterSpacing: "-0.035em"
  figure:
    fontFamily: "Schibsted Grotesk, ui-sans-serif, system-ui, sans-serif"
    fontSize: "54px"
    fontWeight: 800
    lineHeight: 1
    letterSpacing: "-0.03em"
  body:
    fontFamily: "Schibsted Grotesk, ui-sans-serif, system-ui, sans-serif"
    fontSize: "1.125rem"
    fontWeight: 400
    lineHeight: 1.375
  label:
    fontFamily: "Schibsted Grotesk, ui-sans-serif, system-ui, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.43
  data:
    fontFamily: "Fragment Mono, ui-monospace, SFMono-Regular, Menlo, monospace"
    fontSize: "1.25rem"
    fontWeight: 400
    lineHeight: 1.4
rounded:
  cell: "2px"
  sm: "4px"
  md: "6px"
  lg: "8px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "16px"
  lg: "20px"
  xl: "24px"
  section: "128px"
components:
  button-primary:
    backgroundColor: "{colors.paper}"
    textColor: "{colors.ink}"
    rounded: "{rounded.md}"
    padding: "8px 16px"
  button-primary-hover:
    backgroundColor: "#ffffff"
  button-approve:
    backgroundColor: "rgb(169 215 168 / 0.15)"
    textColor: "{colors.match}"
    rounded: "{rounded.md}"
    padding: "16px 24px"
  input:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.paper}"
    rounded: "{rounded.md}"
    padding: "10px 12px"
---
# Design System: Theseus
## Overview
**Creative North Star: "The Two-Plate Press Sheet"**

Theseus is a dark press sheet on which every piece of evidence is printed twice: the C reference from a blue plate, the Rust translation from an orange plate. Where the two outputs agree the plates register and read as one paper-white mark; where they disagree the plates stay apart and the mark turns red. The hero headline, the 578-mark canvas, every run-view row and every verifier cell are that one mechanism at different scales, so a judge at the back of a room and an engineer rerunning the evidence see the same thing: the verifier decided, not the model.

The ground is warm charcoal, never black; type is paper-white, heavy and tight at display scale and plain at reading scale; anything the verifier produced is set in a monospace with ligatures off. Colour is semantic or absent. Surfaces are flat and divided by hairlines; nothing casts a shadow. Density is editorial on the home page (one idea per pinned viewport) and operational on the run view (a 1440px sheet of hairline-ruled rows). It is a migration tool producing evidence, not a dashboard: no grid of equal stat cards, no score without its inputs.

**Key Characteristics:**
- Two plates: C is always blue, Rust is always orange; agreement registers to paper-white.
- Verdict colours only: red is a divergence, amber a sanitizer crash, sage agreement; nothing else is coloured.
- Flat press sheet: one tonal step (ink to raised sheet) plus 1px hairlines; no shadows, no glows, no gradients.
- Motion is the mechanism (plates registering, cells filling, rounds replaying), never ornament; reduced motion renders the final state.
## Colors
A warm two-ink press palette: five neutrals, two plate inks, three verdicts; every value is a Tailwind `@theme` token in `app/globals.css`.
### Primary (the plates)
- **C Plate Blue** (`clang`, #74b3ff): the C side. Column heads, the "C ·" label over code and crash panels, the left dot of an apart mark, the `::before` copy of a registration headline.
- **Rust Plate Orange** (`rust`, #ff7a3a): the Rust side. Column heads, the "Rust ·" label, the `::after` copy of a registration headline. Also text selection and the focus ring, the only chrome it touches.
### Secondary (the verdicts)
- **Sage** (`match`, #a9d7a8): agreement the verifier reported. A full total, 64/64 held-out, the repair line, evidence ticks, running and completed status, the Approve button, match cells.
- **Divergence Red** (`div`, #ff5a3c): an input whose outputs differed, a failed job, an API error, a divergent cell, the right dot of an apart mark, the strike through a superseded draft output. A highlighted code line is red at 18% with a 3px inset left rule.
- **Sanitizer Amber** (`crash`, #f2b84b): an AddressSanitizer crash and nothing else. The crash mark and cell, the CVE step's rule, the CVE label.
### Neutral
- **Ink** (`ink`, #151411) is the page and every input well; **Raised Sheet** (`panel`, #1c1b17) is a selected row, the repair row and the event log.
- **Hairline** (`line`, #2f2d27) is every border, rule and divider and the disabled fill; **Dim** (`dim`, #948e80) is secondary text, mono metadata, the spine head and a pending mark; **Paper** (`paper`, #ece7dc) is primary text, the registered mark and the filled button, at 85/80/70% for long prose, code and superseded text.
### Named Rules
**The Two Plates Rule.** Blue means C and orange means Rust, everywhere, and nothing else: never a button, a chart line or a decoration.
**The Verdict-Only Rule.** Red, amber and sage each name one verifier outcome (divergence, sanitizer crash, agreement). If the verifier did not report it, it is paper or dim.
**The Registration Rule.** Agreement is not a green tick; it is two plates registering into one paper-white mark. Sage goes on the number, the mark stays white.
## Typography
**Display and Body Font:** Schibsted Grotesk (variable 400–900, `ss01` and `cv05` on; self-hosted in `public/fonts`, OFL)
**Data Font:** Fragment Mono (400; ligatures and `calt` off; self-hosted, OFL)

**Character:** A heavy, tight grotesk that reads from the back of a room, against a plain monospace for anything the verifier printed. Sentence case throughout: no uppercase labels, no tracked-out kickers.
### Hierarchy
Two ladders. The home page is fluid: display sizes are `clamp()` tied to viewport width. The run view is fixed: a rem ladder with one 54px figure, so the projected frame and a laptop read the same rows.
- **Display** (800, `clamp(44px, 7vw, 112px)`, 0.94, −0.035em, `text-wrap: balance`): the hero. The counter is `clamp(64px, 8vw, 132px)` tabular; the footer line `clamp(34px, 5vw, 80px)`.
- **Headline** (800, `clamp(34px, 4.4vw, 68px)`, 0.94, −0.035em): section heads; heads inside the pinned overlays are `clamp(28px, 3.4vw, 52px)`.
- **Figure** (800, 54px, 1, −0.03em): run-view totals, C left and Rust right. The wall count is 3.75rem over a 1.875rem dim denominator; comparison values 1.25rem/700, tabular.
- **Body** (400, 1.125rem, 1.375, max 42–52ch, paper at 85%): explanatory prose. The hero lede is 1.3rem/1.4 at 40ch; captions are 1rem dim.
- **Title and Label** (700, 1rem): panel heads such as "Evidence report"; the wordmark is 1.125rem/800 at −0.02em. Labels are 0.875rem dim; plate heads 0.875rem/700 in their plate colour; pipeline detail 0.75rem mono.
- **Data** (mono, 1.25rem): an output in a row. Inputs on the spine are 0.875rem mono dim; code is 0.88rem/1.5 with 3.2ch dim line numbers; sanitizer reports 0.75rem/1.625.

**The Mono Evidence Rule.** Inputs, outputs, ids, patterns, offsets and reports are Fragment Mono; the argument around them is the grotesk. Numbers that change are `tabular-nums`.
## Layout
Home: full-bleed sections with a 24/48/80px gutter (base/768/1024px), 128px vertical rhythm and a 5fr/7fr split at 1024px and up (argument left, evidence right; the rewrite is 1fr/1fr), two of them pinned for the scroll story. Run view: one column, max 1440px, 24/48px gutter, 20px between blocks, 20px inside panels. The evidence section is a three-column grid `1fr 11rem 1fr`: C output left, the registration spine (mark over input) centred, Rust output right-aligned; rows are hairline-separated with 16px vertical padding and expand in place on click. Pipeline steps share the width equally with 8px gaps. Spacing is the 4px scale; steps in use: 4, 6, 8, 12, 16, 20, 24, 32, 40, 48, 56, 64, 80, 128. Breakpoints 640, 768, 1024.
**The Spine Rule.** An evidence row puts C on the left plate, Rust on the right and the verdict mark on the spine between them; totals come first at figure scale, never a card per stat.
## Elevation & Depth
Flat. No `box-shadow`, no glow (the `.glow-*` classes resolve to `text-shadow: none`). Depth is one tonal step, ink to raised sheet, for a selected row, the repair row and the event log, plus 1px hairlines; the home nav floats on ink at 90%.
**The Flat Press Rule.** A change of state changes colour, never elevation.
## Shapes
Rectangles with hairline borders and small radii: 4px for pills and badges, 6px for buttons, panels and fields, 8px for code wells and the try textarea, 2px for verifier cells. Marks and chips are circles (`rounded-full`): the mark is 18px, cells 12/9/7px by input count. Rules carry meaning: a 3px top rule on each pipeline step in its verdict colour, a 3px inset left rule on a highlighted code line, a 1px red ring around a divergent canvas mark, a strike-through on a superseded draft output. The only clipping is the top-down clip-path wipe on the home page.
## Components
### Buttons
- **Primary** ("Start a live run", the header's "Approve and open PR"): paper on ink, 6px, 700, 16×8px at 1rem up to 28×12px at 1.25rem; hover lifts to pure white (#ffffff); disabled is hairline fill with dim text. Until the job completes the header button renders as a hairline outline with dim text.
- **Approve** (evidence panel): sage outline on a 15% sage fill, sage text 1.25rem/700, 24×16px, full width; hover fills sage with ink text; disabled is hairline, transparent, dim, `cursor: not-allowed`.
- **Secondary and links:** "Load" is a dim outline that fills raised sheet on hover; text links underline in hairline (8px offset in prose, 4px inline) and switch the underline to paper on hover. Focus on any control that is not a text field: 2px rust outline at 3px offset.
### Chips
- **Preset** (try panel): circle-ended, hairline border, mono 1rem, 12×4px, hairline fill on hover. **Status pill:** 4px, 600, 12×4px at 1rem (8×2px at 0.75rem inline); running and verifying are sage with a pulsing 8px dot; completed sage on 10% sage; failed red on 10% red; queued and cancelled hairline. **Sample badge:** dashed dim border, 0.75rem.
### Cards / Containers
- 6px radius, 1px hairline, no fill (ink shows through), 20px padding, a 1rem/700 head with 16px beneath. Raised-sheet fill only for selected and repair rows and the event log. No shadow.
### Inputs / Fields
- Ink well, hairline border, 6px, 12×10px, 1.125rem paper, mono when the value is bytes or an id; focus steps the border from hairline to dim. The try textarea is 8px, 12px padding, 1.25rem mono. An error is a 1.125rem red line, or a red-outlined box on 10% red.
### Navigation
- Home: a fixed bar on ink at 90%, wordmark 1.125rem/800 at −0.02em, dim links that turn paper, one primary button. Run view: a hairline-ruled header reading wordmark / library · status pill · mono id · runner · cost, with the sample badge and Approve pinned right.
### Plate Row and Mark (signature)
The three-column row with a Mark on its spine. Mark states: **registered**, one 18px paper circle; **apart**, a blue circle at −38% and a red circle at +38% of its size, `mix-blend-mode: screen`; **crash**, amber; **pending**, dim with the live pulse. The hero sets the same signature in type (`.reg`: blue `::before` and orange `::after` copies of the text offset by `--rx`/`--ry`, screen-blended, animating from 14px/−7px to 0) and on canvas (`Field`: 578 marks, 34 per row, each a blue and an orange dot that register to paper; divergent marks are red, 1.25× and ringed until the repair brings them in).
### Pipeline Strip and Verifier Wall (signature)
Seven equal steps, each a 3px top rule in its verdict colour (hairline when neutral, paper with a pulsing dot when active), 0.875rem/700 label, 0.75rem mono detail. The wall is one 2px cell per input, 3px gaps, 8% paper at rest, filling match/div/crash with a 260ms transition staggered across 1.4s; verify rounds replay every 3.2s; a `scan` cell shimmers on a 1.6s loop while the verifier runs.
## Do's and Don'ts
### Do:
- **Do** print evidence twice: C left in blue, Rust right in orange, the verdict mark on the spine, totals first at figure scale.
- **Do** animate only what the verifier changed: plates registering, cells filling, a count rising (900ms ease-out), a new line rising 8px over 500ms `cubic-bezier(0.2, 0.8, 0.2, 1)`, a live dot pulsing at 1.4s. Pins, scrubs and momentum scroll (GSAP ScrollTrigger pins of +170% and +420%, Lenis `lerp: 0.09`) belong to the home page only, and under `prefers-reduced-motion` the story renders its final state (plates registered, counter final, last overlay shown) while every animation and transition collapses to 1ms.
- **Do** say "N tested behaviors preserved" or "independently checks agreement on N tested inputs", "byte-identical", "registered after the repair", "rejects this historical crashing input", "hit its 64K output cap", and "likely in" for an inferred function name; keep every claim traceable to a verifier event with its input and both outputs.
- **Do** keep sentence case everywhere, mono for anything the verifier printed, and `tabular-nums` on any number that changes.
### Don't:
- **Don't** use red, amber or sage for anything the verifier did not report, or blue and orange for anything but the plates.
- **Don't** add shadows, glows, gradients or a grid of equal stat cards, and don't call any surface a dashboard.
- **Don't** write "proves it behaves the same", "equivalent", "100% compatible", "secure", "CVE fixed", or that Flash "can't"; never fill in libexpat results.
- **Don't** load fonts or scripts from a CDN; the faces are self-hosted in `public/fonts`.
