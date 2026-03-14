# Compact Sparkline Status Line

Replaces the 40-character progress bar in the Claude Code status line with a
single Unicode block character (sparkline) that doubles as a color-coded
context usage indicator.

## Before

```
git:roachdev/main  |  Claude Opus 4.6  |  $1.23  |  12m  |  [████████████████░░░░░░░░░░░░░░░░░░░░░░░░ 40%]
```

## After

```
git:roachdev/main  |  Claude Opus 4.6  |  $1.23  |  12m  |  40%[▃]
```

## How it works

The context window usage percentage (0-100%) is mapped to one of 9 Unicode
block characters that visually represent fill level:

```
" ▁▂▃▄▅▆▇█"
```

The character is color-coded by usage level with a light grey (254) background
so the unfilled portion is visible:

- **Green** (0-50%): comfortable usage
- **Yellow** (51-80%): moderate usage
- **Red** (81-100%): high usage, approaching context limit

The format is `<percent>[<sparkline>]` — percentage outside the brackets,
colored block character inside, e.g. `65%[▅]`.
