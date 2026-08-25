# Design system

Visual identity for the `web/` page. Direction chosen by the owner against a
reference (hilos.sh): warm paper + serif/mono pairing, playful but disciplined.
Light-only by design — do not add a dark theme or reintroduce the default
"dark slate + sky-blue accent" look. Tokens live in `web/style.css` (`:root`);
this doc is their meaning and rules of use.

## Color

### Surfaces & ink

| Token | Value | Use |
|---|---|---|
| `--paper` | `#fbfbf9` | Page background, philosopher heads, inputs |
| `--parchment` | `#f6f1eb` | Panels (config, playback) |
| `--parchment-deep` | `#ede7db` | The table itself |
| `--ink` | `#141510` | Text, faces, fork outlines, primary button |
| `--muted` | `#7c7c67` | Secondary text, labels |
| `--faint` | `#abab9c` | Unheld forks, faintest text |
| `--hairline` | `#0000001a` | All borders |

Warm, olive-tinted neutrals throughout — never pure white, pure black, or cool grays.

### State accents (functional, not decorative)

| Token | Value | Meaning |
|---|---|---|
| `--thinking` | `#8b85f0` | Thinking (periwinkle — philosophizing) |
| `--hungry` | `#e0a93b` | Hungry (mustard) |
| `--eating` | `#4cb782` | Eating (green) |
| `--other` | `#7c7c67` | Unknown states from future strategies |
| `--danger` / `--danger-soft` | `#e40014` / `#ffedee` | Deadlock, engine errors |
| `--dirty` | `#e1791b` | Chandy–Misra dirty fork (clean = `--paper`) |

Accents code simulation state — one meaning each. Don't reuse them for
decoration, and don't rely on color alone: every state also has a face
(see Signature).

## Typography

| Role | Face | Where |
|---|---|---|
| Display | **Fraunces** 500–600 | Page title only |
| Body | **Source Serif 4** 400/600 | Prose, subtitle |
| Utility | **Geist Mono** 400/500 | All controls, labels, ticks, stats, seeds, errors, footer |

Loaded from Google Fonts. Anything that is *data or control* is mono; anything
that is *prose* is serif. Control labels: small uppercase mono with `0.08em`
letter-spacing — this is the page's technical voice.

## Shape & layout

- Radius: 12px panels, 7px controls, pill for the deadlock badge.
- Borders: 1px `--hairline` everywhere; hover borders `--ink`. No shadows.
- Single centered column, max-width 1000px; table SVG max 580px.
- Focus: 2px `--ink` outline, 2px offset (keyboard visibility is a floor, not a feature).

## Signature: philosopher faces

The one deliberately playful element — keep everything else quiet. Philosophers
are paper circles with a 3.5px state-colored ring and ink-stroke faces drawn in
`drawFace()` (`web/app.js`):

- **Thinking** — closed downward eyes, flat mouth, two musing dots floating up-right
- **Hungry** — wide pupil eyes, small open "o" mouth
- **Eating** — happy closed eyes, full dark cheeks
- **Unknown** — neutral dots + flat mouth (forward compatibility)

Faces make state legible without color. Forks are 4-tine cutlery silhouettes
(`FORK_D` path constant — swap point for a Noun Project icon, normalized to a
~16×26 box), resting between philosophers, sliding to the holder's side when held.

## Voice

Plain, specific, sentence case. Errors say what happened and what to do next
("Build it with: …"), never apologize, never vague. No emoji in UI text.
