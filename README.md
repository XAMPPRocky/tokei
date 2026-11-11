# kodestats

[![Crates.io](https://img.shields.io/crates/v/kodestats.svg)](https://crates.io/crates/kodestats)
[![Documentation](https://docs.rs/kodestats/badge.svg)](https://docs.rs/kodestats)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/kodestats.svg)](#license)

**kodestats** is the core library and CLI behind **Codestats**, a **Bricks** tool that lets AI agents quickly and accurately count lines of code, comments, blanks, and language distribution across any codebase.

Built on top of [Tokei](https://github.com/XAMPPRocky/tokei), it features a faster parallel file scanner and additional reporting capabilities tailored for developer and AI workflows.

## Example (running on Bricks)

```text
$ kodestats --top 5 -s code ~/Documents/Bricks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language              Files        Lines         Code     Comments       Blanks       %
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 TypeScript              291        30372        24594         2707         3071   10.0%
 JSON                     32        23793        23791            0            2    9.7%
 JavaScript               11        20325        19061         1171           93    7.8%
 TOML                     42         2193         1749          219          225    0.7%
 Python                   17         1523         1247           52          224    0.5%
 TSX                      10          779          677           33           69    0.3%
 Shell                    11          782          565          139           78    0.2%
 CSS                       7          314          291            7           16    0.1%
 SVG                     135          204          203            0            1    0.1%
 BASH                      1           88           51           28            9    0.0%
 PowerShell                1           52           41           11            0    0.0%
 Dockerfile                1           34           14           13            7    0.0%
 MDX                      54        12034            0         9194         2840    0.0%
 Plain Text               45         2189            0         1979          210    0.0%
─────────────────────────────────────────────────────────────────────────────────────────
 Rust                    479       170404       154922         3503        11979   63.1%
 |- Markdown             471        11893           69        11161          663    0.0%
 (Total)                           182297       154991        14664        12642   63.1%
─────────────────────────────────────────────────────────────────────────────────────────
 Svelte                  228         9022         8464           46          512    3.4%
 |- CSS                    5          279          269            6            4    0.1%
 |- JavaScript           228         8507         6968          665          874    2.8%
 (Total)                            17808        15701          717         1390    6.4%
─────────────────────────────────────────────────────────────────────────────────────────
 HTML                     16          187          187            0            0    0.1%
 |- CSS                    1            1            1            0            0    0.0%
 |- JavaScript             2            2            2            0            0    0.0%
 (Total)                              190          190            0            0    0.1%
─────────────────────────────────────────────────────────────────────────────────────────
 Markdown                 74         8361            0         6622         1739    0.0%
 |- BASH                  13          130           96           20           14    0.0%
 |- JSON                   4           48           48            0            0    0.0%
 |- Markdown               4           34            0           31            3    0.0%
 |- Rust                   9         2502         2234           13          255    0.9%
 |- Svelte                 1           11           11            0            0    0.0%
 |- TOML                   6           55           53            1            1    0.0%
 (Total)                            11141         2442         6687         2012    1.0%
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                  1455       306118       245608        37621        22889  100.0%
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────────────────────────────────────────────
 Top 5 files
─────────────────────────────────────────────────────────────────────────────────────────
 |quivalence.js (JavaScript)        18834        17726         1108            0    7.2%
 |s/package-lock.json (JSON)        11550        11550            0            0    4.7%
 |p/package-lock.json (JSON)         4058         4058            0            0    1.7%
 |icons/manifest.json (JSON)         2921         2921            0            0    1.2%
 |ntrol/controller.rs (Rust)         2752         2584           75           93    1.1%
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Highlights

- **Fast parallel scanning** — parses files directly inside walker threads with per-thread aggregation (~25% faster on large repositories).
- **Language share (`%` column)** — displays each language's percentage of total code lines.
- **Top N largest files (`--top <n>`)** — surfaces the largest files across all languages.
- **CSV & Markdown exports (`-o csv`, `-o md`)** — built-in structured outputs ready for reports or LLM context.
- **`.gitattributes` support** — automatically skips files marked `linguist-generated`, `linguist-vendored`, or `linguist-documentation`.
- **Line count filters (`--min-lines`, `--max-lines`)** — filter files by size.
- **Custom file extensions** — map custom extensions to any language in `tokei.toml` / `.tokeirc` or via `Config`.

## Installation

### As a library (`Cargo.toml`)

```toml
[dependencies]
kodestats = "0.1"
```

### As a CLI

```bash
cargo install kodestats
```

## Library Usage

```rust
use kodestats::{Config, LanguageType, Languages};

let mut languages = Languages::new();
languages.get_statistics(&["src"], &[".git", "target"], &Config::default());

if let Some(rust) = languages.get(&LanguageType::Rust) {
    println!("Rust code lines: {}", rust.code);
}
```

## CLI Usage

```bash
# Count current directory
kodestats

# Show the 10 largest files
kodestats --top 10

# Only count files with at least 200 lines
kodestats --min-lines 200

# Export as Markdown or CSV
kodestats -o md
kodestats -o csv > stats.csv
```

## License

Dual-licensed under [MIT](LICENCE-MIT) or [Apache-2.0](LICENCE-APACHE), at your option.
Originally forked from [Tokei](https://github.com/XAMPPRocky/tokei) by Erin Power and contributors.
