<div align="center">

# 🇳🇱 pizza-analysis-dutch

**Dutch text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--dutch-blue)](https://github.com/pizza-rs/analysis-dutch)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Dutch language analysis with Snowball-based stemming and stop word removal.
Handles Dutch morphology including compound words and common suffix patterns.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `dutch_stem` | Dutch Snowball stemmer |
| TokenFilter | `dutch_stop` | Dutch stop words (101 entries) |
| Analyzer | `dutch` | Full pipeline: lowercase → stem → stop |

### Stemmer Behavior

The Dutch stemmer handles common suffix patterns:
- Plural: `-en`, `-s` removal
- Diminutive: `-je`, `-tje`, `-pje` handling
- Verb forms: past tense `-de`/`-te` stripping

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_dutch::register_all(&mut factory);

let analyzer = factory.get_analyzer("dutch").unwrap();
// "fietsers" → "fietser"
```

## Installation

```toml
[dependencies]
pizza-analysis-dutch = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["dutch"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
