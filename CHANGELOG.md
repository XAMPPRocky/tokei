# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# Unreleased

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#687](https://github.com/xampprocky/tokei/issues/687)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#687](https://github.com/xampprocky/tokei/issues/687)**
    - Fix all feasible clippy issues ([`0751e3b`](https://github.com/xampprocky/tokei/commit/0751e3bb830614e671b0baa750c38cb9b9946ee1))
 * **Uncategorized**
    - Bump tera from 1.6.0 to 1.6.1 ([`8fdee94`](https://github.com/xampprocky/tokei/commit/8fdee9460191d9c223b89fca44fbed03b373b982))
</details>

# 12.1.1 (2020-12-29)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#686](https://github.com/xampprocky/tokei/issues/686)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#686](https://github.com/xampprocky/tokei/issues/686)**
    - Fix for #684 ([`a64cfcc`](https://github.com/xampprocky/tokei/commit/a64cfcc699fb4ac275f439b54220ab9fffdb702d))
 * **Uncategorized**
    - Release 12.1.1 ([`d20da64`](https://github.com/xampprocky/tokei/commit/d20da64a77cc15deda80424f073306217623b86e))
    - Bump serde_json from 1.0.60 to 1.0.61 ([`f938223`](https://github.com/xampprocky/tokei/commit/f938223a1a830fd2981c569287d2d100a6819f77))
    - Update mean_bean_deploy.yml ([`23895f5`](https://github.com/xampprocky/tokei/commit/23895f54f281a93e1417cc720613814b2d417db0))
</details>

# 12.1.0 (2020-12-23)

## Introduction
Tokei is a fast and accurate code analysis CLI tool and library, allowing you to
easily and quickly see how many blank lines, comments, and lines of code are in
your codebase. All releases and work on Tokei and tokei.rs ([the free companion
badge service][rs-info]) are [funded by the community through
GitHub Sponsors][sponsor].

You can always download the latest version of tokei through GitHub Releases or
Cargo. Tokei is also available through other [package managers][pkg], though
they may not always contain the latest release.

```
cargo install tokei
```

## Bug Fixes

 - <csr-id-e608b7d3443683754e816c8312a9f8c516a9f9f3/> shellcheck warnings
   SC2086: Double quote to prevent globbing and word splitting.
   SC2162: read without -r will mangle backslashe

## New Features

 - <csr-id-b9b022f8c91eaa07b3872faf7e9edb6f7c69c6d7/> number formatted printing
   Adds commandline argument to print numbers with commas, plain, or dots.
   For example, dots would result in 1.324.

## What's New?

- [Added `-n/--num-format=[commas, dots, plain, underscores]` for adding
  separator formatting for numbers.](https://github.com/XAMPPRocky/tokei/pull/591)
- [The total is now included in output formats such as JSON.](https://github.com/XAMPPRocky/tokei/pull/580)
- [`--no-ignore` now implies other ignore flags.](https://github.com/XAMPPRocky/tokei/pull/588)
- [Added `--no-ignore-dot` flag to ignore files such as `.ignore`.](https://github.com/XAMPPRocky/tokei/pull/588)
- [Added single line comments to F\*](https://github.com/XAMPPRocky/tokei/pull/670)
- Updated various dependencies.

## Added

- [ABNF](https://github.com/XAMPPRocky/tokei/pull/577)
- [CodeQL](https://github.com/XAMPPRocky/tokei/pull/604)
- [LiveScript](https://github.com/XAMPPRocky/tokei/pull/607)
- [Stylus](https://github.com/XAMPPRocky/tokei/pull/619)
- [DAML](https://github.com/XAMPPRocky/tokei/pull/620)
- [Tera](https://github.com/XAMPPRocky/tokei/pull/627)
- [TTCN-3](https://github.com/XAMPPRocky/tokei/pull/621)
- [Beancount](https://github.com/XAMPPRocky/tokei/pull/630)
- [Gleam](https://github.com/XAMPPRocky/tokei/pull/646)
- [JSONNet](https://github.com/XAMPPRocky/tokei/pull/634)
- [Stan](https://github.com/XAMPPRocky/tokei/pull/633)
- [Gwion](https://github.com/XAMPPRocky/tokei/pull/659)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 87 commits contributed to the release over the course of 179 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 43 unique issues were worked on: [#574](https://github.com/xampprocky/tokei/issues/574), [#575](https://github.com/xampprocky/tokei/issues/575), [#576](https://github.com/xampprocky/tokei/issues/576), [#577](https://github.com/xampprocky/tokei/issues/577), [#584](https://github.com/xampprocky/tokei/issues/584), [#588](https://github.com/xampprocky/tokei/issues/588), [#590](https://github.com/xampprocky/tokei/issues/590), [#607](https://github.com/xampprocky/tokei/issues/607), [#609](https://github.com/xampprocky/tokei/issues/609), [#615](https://github.com/xampprocky/tokei/issues/615), [#617](https://github.com/xampprocky/tokei/issues/617), [#619](https://github.com/xampprocky/tokei/issues/619), [#620](https://github.com/xampprocky/tokei/issues/620), [#621](https://github.com/xampprocky/tokei/issues/621), [#623](https://github.com/xampprocky/tokei/issues/623), [#625](https://github.com/xampprocky/tokei/issues/625), [#627](https://github.com/xampprocky/tokei/issues/627), [#630](https://github.com/xampprocky/tokei/issues/630), [#631](https://github.com/xampprocky/tokei/issues/631), [#633](https://github.com/xampprocky/tokei/issues/633), [#634](https://github.com/xampprocky/tokei/issues/634), [#637](https://github.com/xampprocky/tokei/issues/637), [#638](https://github.com/xampprocky/tokei/issues/638), [#639](https://github.com/xampprocky/tokei/issues/639), [#640](https://github.com/xampprocky/tokei/issues/640), [#642](https://github.com/xampprocky/tokei/issues/642), [#645](https://github.com/xampprocky/tokei/issues/645), [#646](https://github.com/xampprocky/tokei/issues/646), [#647](https://github.com/xampprocky/tokei/issues/647), [#648](https://github.com/xampprocky/tokei/issues/648), [#650](https://github.com/xampprocky/tokei/issues/650), [#652](https://github.com/xampprocky/tokei/issues/652), [#654](https://github.com/xampprocky/tokei/issues/654), [#656](https://github.com/xampprocky/tokei/issues/656), [#658](https://github.com/xampprocky/tokei/issues/658), [#659](https://github.com/xampprocky/tokei/issues/659), [#661](https://github.com/xampprocky/tokei/issues/661), [#663](https://github.com/xampprocky/tokei/issues/663), [#668](https://github.com/xampprocky/tokei/issues/668), [#670](https://github.com/xampprocky/tokei/issues/670), [#674](https://github.com/xampprocky/tokei/issues/674), [#675](https://github.com/xampprocky/tokei/issues/675), [#677](https://github.com/xampprocky/tokei/issues/677)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#574](https://github.com/xampprocky/tokei/issues/574)**
    - Bump aho-corasick from 0.7.10 to 0.7.13 ([`66a8189`](https://github.com/xampprocky/tokei/commit/66a81899d36bc99f5346f2320023a02785ed06c8))
 * **[#575](https://github.com/xampprocky/tokei/issues/575)**
    - Bump parking_lot from 0.10.2 to 0.11.0 ([`3c2653b`](https://github.com/xampprocky/tokei/commit/3c2653b184d90492ceb704dd1554b130566a8618))
 * **[#576](https://github.com/xampprocky/tokei/issues/576)**
    - Remove duplicated 'json' from --output ([`0f23e24`](https://github.com/xampprocky/tokei/commit/0f23e248c2756f61f8a395eb962fdbac806ccc8f))
 * **[#577](https://github.com/xampprocky/tokei/issues/577)**
    - Add support for ABNF grammar specs ([`404e664`](https://github.com/xampprocky/tokei/commit/404e6642677d8bfa2e3bb121c82ee6c30e437b50))
 * **[#584](https://github.com/xampprocky/tokei/issues/584)**
    - Bump dirs from 2.0.2 to 3.0.1 ([`379b7b4`](https://github.com/xampprocky/tokei/commit/379b7b44d5a9eb35ad3642d5e70cc53d7a1ec759))
 * **[#588](https://github.com/xampprocky/tokei/issues/588)**
    - Make --no-ignore imply all other --no-ignore- flags ([`a6fa401`](https://github.com/xampprocky/tokei/commit/a6fa401dc4d96cc4f12e43e54ae3b3004f74f2a8))
 * **[#590](https://github.com/xampprocky/tokei/issues/590)**
    - Bump log from 0.4.8 to 0.4.11 ([`31547d2`](https://github.com/xampprocky/tokei/commit/31547d2ca41032061a10f4ee0a44c35c28bb9413))
 * **[#607](https://github.com/xampprocky/tokei/issues/607)**
    - Add LiveScript to languages.json ([`724c8e8`](https://github.com/xampprocky/tokei/commit/724c8e8995e8fd6331bf92b12ada014db03ab49d))
 * **[#609](https://github.com/xampprocky/tokei/issues/609)**
    - Bump tera from 1.3.1 to 1.5.0 ([`9218f9a`](https://github.com/xampprocky/tokei/commit/9218f9a1bf27976a94260feb67590d80b5e0da63))
 * **[#615](https://github.com/xampprocky/tokei/issues/615)**
    - Bump git2 from 0.13.6 to 0.13.10 ([`2a8b1c7`](https://github.com/xampprocky/tokei/commit/2a8b1c7693c1f409f89aa5009a44a30f7c1a5bbe))
 * **[#617](https://github.com/xampprocky/tokei/issues/617)**
    - Bump rayon from 1.3.1 to 1.4.0 ([`37df49f`](https://github.com/xampprocky/tokei/commit/37df49fa8f58a09830c689fdd390385305e23eea))
 * **[#619](https://github.com/xampprocky/tokei/issues/619)**
    - Add Stylus language ([`420afd3`](https://github.com/xampprocky/tokei/commit/420afd35738be34711c6745e54ac964239262496))
 * **[#620](https://github.com/xampprocky/tokei/issues/620)**
    - Add definition for DAML ([`bce9242`](https://github.com/xampprocky/tokei/commit/bce9242ba4eb7d7db499960a24de60e606653f21))
 * **[#621](https://github.com/xampprocky/tokei/issues/621)**
    - Add support for TTCN-3 ([`82429f5`](https://github.com/xampprocky/tokei/commit/82429f5c32a4e778b4f0c16621476f10e76d1d7c))
 * **[#623](https://github.com/xampprocky/tokei/issues/623)**
    - Bump crossbeam-channel from 0.4.2 to 0.4.4 ([`7cc20e4`](https://github.com/xampprocky/tokei/commit/7cc20e44eb873ba0b2c9e3191303806c2a8ffded))
 * **[#625](https://github.com/xampprocky/tokei/issues/625)**
    - Bump git2 from 0.13.10 to 0.13.11 ([`4e054c6`](https://github.com/xampprocky/tokei/commit/4e054c67d85705898ce1400ba8329fbc8b026411))
 * **[#627](https://github.com/xampprocky/tokei/issues/627)**
    - add Tera templating language ([`3a89848`](https://github.com/xampprocky/tokei/commit/3a898483feca383f9e712237a2de96a6a474cb05))
 * **[#630](https://github.com/xampprocky/tokei/issues/630)**
    - added beancount file format ([`0d87e84`](https://github.com/xampprocky/tokei/commit/0d87e84b437d7ab1b4d94045abca463dde5143ba))
 * **[#631](https://github.com/xampprocky/tokei/issues/631)**
    - Unify format ([`d7c5485`](https://github.com/xampprocky/tokei/commit/d7c548537cd5828b2d58e09f3207ddacc517b227))
 * **[#633](https://github.com/xampprocky/tokei/issues/633)**
    - add stan language ([`62d2a8d`](https://github.com/xampprocky/tokei/commit/62d2a8db52139f054ad646f5fa4c7797b71b1a1c))
 * **[#634](https://github.com/xampprocky/tokei/issues/634)**
    - Add jsonnet to language list ([`b0720c9`](https://github.com/xampprocky/tokei/commit/b0720c92769dbb1dbb957a518e5183ecb1af7466))
 * **[#637](https://github.com/xampprocky/tokei/issues/637)**
    - Bump toml from 0.5.6 to 0.5.7 ([`9355c7e`](https://github.com/xampprocky/tokei/commit/9355c7e18e6eb6745c40fceb1be914d9dd94b41c))
 * **[#638](https://github.com/xampprocky/tokei/issues/638)**
    - Bump regex from 1.3.9 to 1.4.0 ([`773bd20`](https://github.com/xampprocky/tokei/commit/773bd20a21068d021b248d17fb65904a140a2991))
 * **[#639](https://github.com/xampprocky/tokei/issues/639)**
    - Bump aho-corasick from 0.7.13 to 0.7.14 ([`b742ae1`](https://github.com/xampprocky/tokei/commit/b742ae19725ae57fd46b5f200abe1bb79a0eb0d5))
 * **[#640](https://github.com/xampprocky/tokei/issues/640)**
    - Bump crossbeam-channel from 0.4.4 to 0.5.0 ([`541d968`](https://github.com/xampprocky/tokei/commit/541d9687f68c2a32a0599c94a38b085a5cfd8599))
 * **[#642](https://github.com/xampprocky/tokei/issues/642)**
    - Bump git2 from 0.13.11 to 0.13.12 ([`bd26c80`](https://github.com/xampprocky/tokei/commit/bd26c803abba47fb5070930ee7ec53e2ddeff0ea))
 * **[#645](https://github.com/xampprocky/tokei/issues/645)**
    - Bump env_logger from 0.7.1 to 0.8.1 ([`0ca0858`](https://github.com/xampprocky/tokei/commit/0ca085849d2c5fa25e55001be3d399ccfa6a5c78))
 * **[#646](https://github.com/xampprocky/tokei/issues/646)**
    - Add support for the Gleam language ([`fc327c3`](https://github.com/xampprocky/tokei/commit/fc327c34bcc8bb47cda584d0a1d1ced80bd3eb39))
 * **[#647](https://github.com/xampprocky/tokei/issues/647)**
    - Bump rayon from 1.4.1 to 1.5.0 ([`6ad2826`](https://github.com/xampprocky/tokei/commit/6ad28262c8e4f424bab08597d2229991906dff10))
 * **[#648](https://github.com/xampprocky/tokei/issues/648)**
    - Bump serde_yaml from 0.8.13 to 0.8.14 ([`3174a92`](https://github.com/xampprocky/tokei/commit/3174a9259df9b9e8a505ebc1a6b9de265a4c904e))
 * **[#650](https://github.com/xampprocky/tokei/issues/650)**
    - Bump aho-corasick from 0.7.14 to 0.7.15 ([`55b98f3`](https://github.com/xampprocky/tokei/commit/55b98f358845eee405122f3bda1f256726ded310))
 * **[#652](https://github.com/xampprocky/tokei/issues/652)**
    - Bump once_cell from 1.4.1 to 1.5.1 ([`e0801c7`](https://github.com/xampprocky/tokei/commit/e0801c783325b24b7fb675e46bbf15ded098614a))
 * **[#654](https://github.com/xampprocky/tokei/issues/654)**
    - Add missing language to language list ([`645f800`](https://github.com/xampprocky/tokei/commit/645f8006b2f46ce3ff2765eac38cc15086f669df))
 * **[#656](https://github.com/xampprocky/tokei/issues/656)**
    - Bump parking_lot from 0.11.0 to 0.11.1 ([`7ed0efc`](https://github.com/xampprocky/tokei/commit/7ed0efcef7f19d331617a0b700314b2d70fdf31e))
 * **[#658](https://github.com/xampprocky/tokei/issues/658)**
    - Bump env_logger from 0.8.1 to 0.8.2 ([`62b5e1e`](https://github.com/xampprocky/tokei/commit/62b5e1ed4aae6674e4ea6901f756fd17597d609d))
 * **[#659](https://github.com/xampprocky/tokei/issues/659)**
    - :art: Add support for Gwion ([`feb0631`](https://github.com/xampprocky/tokei/commit/feb0631414e8da85c4d53dfe349b03f682f71e4d))
 * **[#661](https://github.com/xampprocky/tokei/issues/661)**
    - Bump ignore from 0.4.16 to 0.4.17 ([`99ea0d5`](https://github.com/xampprocky/tokei/commit/99ea0d5e2b18b343256ebdd435e11c84a28e8f27))
 * **[#663](https://github.com/xampprocky/tokei/issues/663)**
    - shellcheck warnings ([`e608b7d`](https://github.com/xampprocky/tokei/commit/e608b7d3443683754e816c8312a9f8c516a9f9f3))
 * **[#668](https://github.com/xampprocky/tokei/issues/668)**
    - Add Alpine Linux into Package Managers section ([`231b6a2`](https://github.com/xampprocky/tokei/commit/231b6a2895d4b3632e94e4aa2279fcdffeaf4e26))
 * **[#670](https://github.com/xampprocky/tokei/issues/670)**
    - FStar language: add single-line comments ([`5cd3a38`](https://github.com/xampprocky/tokei/commit/5cd3a3839288a1f9cd7b1f41e32534abae21dff2))
 * **[#674](https://github.com/xampprocky/tokei/issues/674)**
    - Bump tera from 1.5.0 to 1.6.0 ([`ac69f3b`](https://github.com/xampprocky/tokei/commit/ac69f3b49edffc56b5ac58c932073d77dcb77001))
 * **[#675](https://github.com/xampprocky/tokei/issues/675)**
    - Bump toml from 0.5.7 to 0.5.8 ([`b0f18c4`](https://github.com/xampprocky/tokei/commit/b0f18c408d406d872dd439bfd4fe2bece48746e6))
 * **[#677](https://github.com/xampprocky/tokei/issues/677)**
    - Bump git2 from 0.13.12 to 0.13.14 ([`1721bf6`](https://github.com/xampprocky/tokei/commit/1721bf6bec2b749bbacfd38674590ce94a245941))
 * **Uncategorized**
    - Release 12.1.0 ([`4675f21`](https://github.com/xampprocky/tokei/commit/4675f21bd4b3c98e03c537cd7685a7e899b69c9f))
    - Update CHANGELOG for 12.1.0 ([`75a8ff7`](https://github.com/xampprocky/tokei/commit/75a8ff7629b766ef2376ff9ea198d330b4624ef7))
    - Only include children in total ([`34810da`](https://github.com/xampprocky/tokei/commit/34810da843ebb21fb984de0bd8832a3bfd625315))
    - Bump serde from 1.0.117 to 1.0.118 ([`47a066c`](https://github.com/xampprocky/tokei/commit/47a066cc004fa83f81714ede35b6435e14b08911))
    - Bump serde_json from 1.0.59 to 1.0.60 ([`3597cd7`](https://github.com/xampprocky/tokei/commit/3597cd7315d03a88af3ea7feaba677bb4ada2a83))
    - Bump once_cell from 1.5.1 to 1.5.2 ([`f17f1af`](https://github.com/xampprocky/tokei/commit/f17f1af405d024e3d6d4217e0fd36d4ea915ece6))
    - Bump regex from 1.4.1 to 1.4.2 ([`9624d1c`](https://github.com/xampprocky/tokei/commit/9624d1cd2cbe93062384b48151b4d94de4e11108))
    - Update README.md ([`00c30d1`](https://github.com/xampprocky/tokei/commit/00c30d161899673365708fc99f9841cdc9891679))
    - Bump serde from 1.0.116 to 1.0.117 ([`24466b2`](https://github.com/xampprocky/tokei/commit/24466b2503e8143fb31f39d9ad56f9271638a001))
    - Update README.md ([`e5acb95`](https://github.com/xampprocky/tokei/commit/e5acb953e4c908f0f97fc9d0ee749fc6481036d6))
    - Bump regex from 1.4.0 to 1.4.1 ([`c4b699a`](https://github.com/xampprocky/tokei/commit/c4b699a76d39e612e650fc7c269935a656a2e00a))
    - Bump serde_json from 1.0.58 to 1.0.59 ([`e530f46`](https://github.com/xampprocky/tokei/commit/e530f46f7d0b8bbdbf86d020379236b9addcf605))
    - Delete markdown.md ([`ca1888a`](https://github.com/xampprocky/tokei/commit/ca1888a89b9667d972d24e8c1d2a84eddf260055))
    - Update README.md ([`6215bab`](https://github.com/xampprocky/tokei/commit/6215bab62548f50b84d72c47ba306986f9acfceb))
    - Update README.md ([`7c0a288`](https://github.com/xampprocky/tokei/commit/7c0a2881d2c23b29f390e4be87edb4db8e224da5))
    - Update README.md ([`e9631bc`](https://github.com/xampprocky/tokei/commit/e9631bcf498b0e7bb14de86d6fcb0f0db948c169))
    - Bump serde_json from 1.0.57 to 1.0.58 ([`49ed629`](https://github.com/xampprocky/tokei/commit/49ed629525a593ec5029b155ddbd162d2a42267c))
    - Bump rayon from 1.4.0 to 1.4.1 ([`4039785`](https://github.com/xampprocky/tokei/commit/40397851acf28d0c5c97c58d2afabc25b57ebd41))
    - Bump serde from 1.0.115 to 1.0.116 ([`7d4ab16`](https://github.com/xampprocky/tokei/commit/7d4ab16013fb5ba64f27e32b7c4c5f195facc26f))
    - Bump once_cell from 1.4.0 to 1.4.1 ([`90c1e2a`](https://github.com/xampprocky/tokei/commit/90c1e2a01299e01184584e32b3587a8e73830315))
    - Update mean_bean_ci.yml ([`18397f1`](https://github.com/xampprocky/tokei/commit/18397f1cdc268e7769bf0ba6afe8c8f950ff47ea))
    - Update mean_bean_deploy.yml ([`d52bc08`](https://github.com/xampprocky/tokei/commit/d52bc083f9b7f0f9f641dd5f3f585ba97d5fd256))
    - Bump clap from 2.33.2 to 2.33.3 ([`9a6e799`](https://github.com/xampprocky/tokei/commit/9a6e799dd0e3afedba974d1804a089280d2287fd))
    - Bump serde from 1.0.114 to 1.0.115 ([`db949c4`](https://github.com/xampprocky/tokei/commit/db949c435568673e3d29d8af8fa1b1608ae58217))
    - Bump clap from 2.33.1 to 2.33.2 ([`900e069`](https://github.com/xampprocky/tokei/commit/900e069e034bae5a95b29898c7243d576295c6d2))
    - Add summary information to output formats ([`64bb4e1`](https://github.com/xampprocky/tokei/commit/64bb4e10fed20423d11956325a823e1ee871ea5d))
    - Added CodeQL language support ([`2b5ab4f`](https://github.com/xampprocky/tokei/commit/2b5ab4fa1cbdec87b7ba75ecf50880b1430da293))
    - Bump dashmap from 3.11.9 to 3.11.10 ([`a442170`](https://github.com/xampprocky/tokei/commit/a4421704399c1c34342630fbb972dfb4529ddcb1))
    - Fix very minor typo in README ([`3d8e059`](https://github.com/xampprocky/tokei/commit/3d8e0595a8e96e360cbbd93824826b092228841e))
    - Bump dashmap from 3.11.7 to 3.11.9 ([`8c05138`](https://github.com/xampprocky/tokei/commit/8c051381ead47d8691508f3370deec2c1bdc8636))
    - Update LICENCE-APACHE ([`bb3b871`](https://github.com/xampprocky/tokei/commit/bb3b87193af8e006f538e680ed7faaaa50d74772))
    - Update LICENCE-MIT ([`cd2f4d0`](https://github.com/xampprocky/tokei/commit/cd2f4d0be18dbb6a58f751b5d0c70f308775ff2c))
    - Bump serde_json from 1.0.56 to 1.0.57 ([`d3a48d3`](https://github.com/xampprocky/tokei/commit/d3a48d322d9ee20566c380bc2df5ac7955ee7607))
    - Update README.md ([`037b889`](https://github.com/xampprocky/tokei/commit/037b88906b6d4ed102b11c6c4791461c5ad92c60))
    - Export find_char_boundary hidden ([`f37b2db`](https://github.com/xampprocky/tokei/commit/f37b2db17a05a1b5c8f8b446ec5f3a4a1402a671))
    - Added num-format option underscores ([`3f1491e`](https://github.com/xampprocky/tokei/commit/3f1491e582f9965477e19fc6d0a2e4aca0533920))
    - number formatted printing ([`b9b022f`](https://github.com/xampprocky/tokei/commit/b9b022f8c91eaa07b3872faf7e9edb6f7c69c6d7))
    - Update mean_bean_deploy.yml ([`904be2a`](https://github.com/xampprocky/tokei/commit/904be2ac238285600ec32b93f85ce8eb0f109098))
    - Bump dashmap from 3.11.4 to 3.11.7 ([`7659bc1`](https://github.com/xampprocky/tokei/commit/7659bc1f2afecfc75f488087e1eae2ce5dda2924))
    - Update mean_bean_ci.yml ([`92a861a`](https://github.com/xampprocky/tokei/commit/92a861a4c53b4b9db02af2b090724cba6328a9af))
    - Update mean_bean_ci.yml ([`dd6f442`](https://github.com/xampprocky/tokei/commit/dd6f4421bd417ae8fdf4e4d402a605028a656971))
    - Update README.md ([`4e839b8`](https://github.com/xampprocky/tokei/commit/4e839b84bd17e86c7b05e4be97e8c120e49484bf))
    - Bump serde_json from 1.0.55 to 1.0.56 ([`2859f4e`](https://github.com/xampprocky/tokei/commit/2859f4ee4500f41d9e48b83e13ec5dd90a40e483))
    - Delete markdown.md ([`3485cd3`](https://github.com/xampprocky/tokei/commit/3485cd3f584d7be9c01e00f059f0ca2cdfe436ce))
</details>

# 12.0.4 (2020-06-24)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release 12.0.4 ([`ca4bab1`](https://github.com/xampprocky/tokei/commit/ca4bab1124aeccf00e888b666313d1a1a5d78c32))
    - Fix empty tags causing panics and support markdown code identifiers to be split by , ([`af326fd`](https://github.com/xampprocky/tokei/commit/af326fd73c522112149b9ff4f6d838eda3e24f22))
</details>

# 12.0.3 (2020-06-22)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release 12.0.3 ([`189fe14`](https://github.com/xampprocky/tokei/commit/189fe14167472477aa879c04cce392f304d73fe8))
</details>

# 12.0.2 (2020-06-22)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release 12.0.2 ([`47c5ea6`](https://github.com/xampprocky/tokei/commit/47c5ea6dfde4163fb24e5553fb12310bee884558))
    - fix len ([`a8ca1af`](https://github.com/xampprocky/tokei/commit/a8ca1af9b109cabb349af7ae1c52635cf146612c))
</details>

# 12.0.1 (2020-06-22)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release 12.0.1 ([`1c72876`](https://github.com/xampprocky/tokei/commit/1c7287666bb89aac6ec00391ec44fcfd697682c7))
    - fix panic if empty ([`e4ff3c2`](https://github.com/xampprocky/tokei/commit/e4ff3c2b5ba601c0888a5147673da55fe2968d1d))
</details>

# 12.0.0 (2020-06-22)

## What's New? 
Tokei 12 comes with some of the biggest user facing changes since 1.0, now in
the latest version tokei will now **analyse and count multiple languages
embedded in your source code** as well as adding support for
**Jupyter Notebooks**. Now for the first time is able to handle and display
different languages contained in a single source file. This currently available
for a limited set of languages, with plans to add more support for more in the
future. The currently supported languages are;

### HTML + Siblings (Vue, Svelte, Etc...)
Tokei will now analyse and report the source code contained in `<script>`,
`<style>`, and `<template>` tags in HTML and other similar languages. Tokei will
read the value of the`type` attribute from the `<script>` tag and detects the
appropriate language based on its mime type or JavaScript if not present. Tokei
will do the same for `<style>` and `<template>` except reading the `lang`
attribute instead of `type` and defaulting to CSS and HTML each respectively.

### Jupyter Notebooks
Tokei will now read Jupyter Notebook files (`.ipynb`) and will read the source
code and markdown from Jupyter's JSON and output the analysed result.

### Markdown
Tokei will now detect any code blocks marked with specified source language and
count each as their respective languages or as Markdown if not present or not
found. Now you can easily see how many code examples are included in
your documentation.

### Rust
Tokei will now detect blocks of rustdoc documentation  (e.g. `///`/`//!`) and
parse them as markdown.

### Verbatim Strings
Tokei is now also capable of handling "verbatim" strings, which are strings that
do not accept escape sequences like `\`. Thanks to @NickHackman for providing
the implementation! This is initially supported for C++, C#, F#, and Rust.

## New Look
To be able to show these new features, tokei's output has been changed to look
like below. For brevity the CLI only displays one level deep in each language,
however the library's parser is fully recursive and you can get access to the
complete report using the library or by outputting the JSON format.

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 BASH                    4           49           30           10            9
 JSON                    1         1332         1332            0            0
 Shell                   1           49           38            1           10
 TOML                    2           77           64            4            9
-------------------------------------------------------------------------------
 Markdown                5         1230            0          965          265
 |- JSON                 1           41           41            0            0
 |- Rust                 2           53           42            6            5
 |- Shell                1           22           18            0            4
 (Total)                           1346          101          971          274
-------------------------------------------------------------------------------
 Rust                   19         3349         2782          116          451
 |- Markdown            12          351            5          295           51
 (Total)                           3700         2787          411          502
===============================================================================
 Total                  32         6553         4352         1397          804
===============================================================================
```

This feature is not just limited to the default output of tokei. You can see it
broken down by each file with the `--files` option.

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Markdown                5         1230            0          965          265
 |- JSON                 1           41           41            0            0
 |- Rust                 2           53           42            6            5
 |- Shell                1           22           18            0            4
 (Total)                           1346          101          971          274
-------------------------------------------------------------------------------
 ./CODE_OF_CONDUCT.md                46            0           28           18
 ./CHANGELOG.md                     570            0          434          136
-- ./markdown.md --------------------------------------------------------------
 |- Markdown                          4            0            3            1
 |- Rust                              6            4            1            1
 |- (Total)                          10            4            4            2
-- ./README.md ----------------------------------------------------------------
 |- Markdown                        498            0          421           77
 |- Shell                            22           18            0            4
 |- (Total)                         520           18          421           81
-- ./CONTRIBUTING.md ----------------------------------------------------------
 |- Markdown                        112            0           79           33
 |- JSON                             41           41            0            0
 |- Rust                             46           38            4            4
 |- (Total)                         200           79           84           37
===============================================================================
 Total                   5         1346          101          971          274
===============================================================================
```

## Breaking Changes
- The JSON Output and format of `Languages` has changed.
- The JSON feature has been removed and is now included by default.
- `Stats` has been split into `Report` and `CodeStats` to better represent the
  separation between analysing a file versus a blob of code.

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 63 commits contributed to the release over the course of 25 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#535](https://github.com/xampprocky/tokei/issues/535), [#546](https://github.com/xampprocky/tokei/issues/546), [#549](https://github.com/xampprocky/tokei/issues/549), [#550](https://github.com/xampprocky/tokei/issues/550), [#555](https://github.com/xampprocky/tokei/issues/555), [#561](https://github.com/xampprocky/tokei/issues/561)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#535](https://github.com/xampprocky/tokei/issues/535)**
    - Verbatim strings ([`96b47ab`](https://github.com/xampprocky/tokei/commit/96b47ab4048b2589359af7d8061453598f6929ec))
 * **[#546](https://github.com/xampprocky/tokei/issues/546)**
    - Bump ignore from 0.4.15 to 0.4.16 ([`608c404`](https://github.com/xampprocky/tokei/commit/608c40474307012115eeb764187ebe46bf9641e3))
 * **[#549](https://github.com/xampprocky/tokei/issues/549)**
    - Add support for q/k languages ([`82781e3`](https://github.com/xampprocky/tokei/commit/82781e3cdb2872181eec17309955ca390e6d2f2d))
 * **[#550](https://github.com/xampprocky/tokei/issues/550)**
    - Specify the --sort option in the configuration files ([`a69e16b`](https://github.com/xampprocky/tokei/commit/a69e16b225676b08607b7ec4c731f3fb61c29b36))
 * **[#555](https://github.com/xampprocky/tokei/issues/555)**
    - Fixed the language name Vim Script to Vim script ([`c9949fe`](https://github.com/xampprocky/tokei/commit/c9949fe10f23b1aec3489a70746ea5d7b09feff8))
 * **[#561](https://github.com/xampprocky/tokei/issues/561)**
    - Remove duplicate keys from languages.json ([`bf1bc3b`](https://github.com/xampprocky/tokei/commit/bf1bc3b1c65414b34eca93f82614183edff8d112))
 * **Uncategorized**
    - Version bump ([`e6296f6`](https://github.com/xampprocky/tokei/commit/e6296f6d72e0811f7d0cd048a40e82415939dd17))
    - fmt ([`01ec622`](https://github.com/xampprocky/tokei/commit/01ec62275890ead30a19197eff416763dd0cb16b))
    - account for \r ([`ef17da8`](https://github.com/xampprocky/tokei/commit/ef17da80ac7b1a312f9de0163f99288e8af436d7))
    - update deps ([`4c105f2`](https://github.com/xampprocky/tokei/commit/4c105f2e7c9bbc4ba065a4f3dbbd3c1da2e324ad))
    - fmt ([`40b8174`](https://github.com/xampprocky/tokei/commit/40b8174d983a33dc35b7a75069ea587f844a0ec3))
    - Update serialize output ([`f19c218`](https://github.com/xampprocky/tokei/commit/f19c21831d9162d14cd92c38f12eca4c8cf8469b))
    - Enabled support for svelte, vue, rubyhtml and fixed edge case ([`7272d67`](https://github.com/xampprocky/tokei/commit/7272d67ae1751a15aa90d2c576cd6cc9715863ca))
    - change files output slightly ([`98865ad`](https://github.com/xampprocky/tokei/commit/98865ad391f429840e3a1c0ec753ff6f8d95e057))
    - Bump serde from 1.0.112 to 1.0.114 ([`a45a277`](https://github.com/xampprocky/tokei/commit/a45a2778d560f857eac7c5a7694aa722d9e331de))
    - Make output slightly smaller ([`dd385c0`](https://github.com/xampprocky/tokei/commit/dd385c00a5113740b34dfe6b44b60b4c8a25b330))
    - remove markdown test ([`fedb1cd`](https://github.com/xampprocky/tokei/commit/fedb1cd3b0a75c75819e1f83d8ed2095ea8322c7))
    - use raw string in build ([`9d21266`](https://github.com/xampprocky/tokei/commit/9d21266980167f4a8adf2bc425578ccd4991d95a))
    - fmt ([`654726f`](https://github.com/xampprocky/tokei/commit/654726fa9d503252f96fb1a39085e91e0668d9cb))
    - use par iter in jupyter ([`2d063cf`](https://github.com/xampprocky/tokei/commit/2d063cf9ac3df7e646ba1e8b224f0d641049cf2a))
    - Normalize path in build ([`250bff9`](https://github.com/xampprocky/tokei/commit/250bff9edf0d74c8f0e81552e1442021002e0228))
    - fmt ([`5720413`](https://github.com/xampprocky/tokei/commit/5720413bf4c4f39c75403ef8e5017237e561c0f2))
    - rename contexts to blobs ([`7286953`](https://github.com/xampprocky/tokei/commit/728695388d9dc172cf4c84819b22418f156d4551))
    - remove extranous file ([`a5df46d`](https://github.com/xampprocky/tokei/commit/a5df46d00b616c3524706e643b9e2c93fdd96a0d))
    - Add jupyter support ([`28917c4`](https://github.com/xampprocky/tokei/commit/28917c4ad35555210c2ce0d166b27ba3e86c1801))
    - fmt ([`5ce242e`](https://github.com/xampprocky/tokei/commit/5ce242e012299396679afa2d1d7e91fef959d68f))
    - Change files print out ([`3ef29b6`](https://github.com/xampprocky/tokei/commit/3ef29b6e6eac0e96823ebd8916aaac8f0c88eaed))
    - Refactor important syntax and add support for templates ([`efde7bd`](https://github.com/xampprocky/tokei/commit/efde7bdcc71bfd930d6e10f79d5894a6524c7ec3))
    - Add comment ([`427299f`](https://github.com/xampprocky/tokei/commit/427299f7c920912fa1abb188f6c2b8d05f9eedcf))
    - Refactor context parsing and add initial support for HTML. ([`d4ef843`](https://github.com/xampprocky/tokei/commit/d4ef84320b893039adde43d017e71a2756adbfa1))
    - fmt ([`0cbb8bc`](https://github.com/xampprocky/tokei/commit/0cbb8bc08457b6d68bf3689d80f3a7d5e4583aa7))
    - Refactor print and summarise code ([`4fbae49`](https://github.com/xampprocky/tokei/commit/4fbae4984d86544675f9d40996009e034032d9bd))
    - Refactor cli_utils into Printer and add Rust context ([`c06fa65`](https://github.com/xampprocky/tokei/commit/c06fa65dd980a11733243bc5ffbfd6ba9528e5fd))
    - Rename hbs file to tera ([`a9fca66`](https://github.com/xampprocky/tokei/commit/a9fca66a5fb24250dafc9910249208f9b9b5e98f))
    - Switch to using LanguageType ([`c185490`](https://github.com/xampprocky/tokei/commit/c185490d2def996c1a97e1c800fec9438bf176ae))
    - refactor parse_line_comment ([`1e2adff`](https://github.com/xampprocky/tokei/commit/1e2adff040eb23e7d38ee1fe70c45f895e1b79da))
    - Bump rayon from 1.3.0 to 1.3.1 ([`d8d5e3b`](https://github.com/xampprocky/tokei/commit/d8d5e3b423dc8f84aecf762b34c5ed233c81cac8))
    - fmt ([`0e4b0b6`](https://github.com/xampprocky/tokei/commit/0e4b0b6ad7c01b476bb892aafa2ade970f0fc0bd))
    - Adjust CLI output ([`446856c`](https://github.com/xampprocky/tokei/commit/446856c42672482737cd2ca04d0cbcd02eb3f6cb))
    - Bump serde from 1.0.111 to 1.0.112 ([`e12fa02`](https://github.com/xampprocky/tokei/commit/e12fa02b17ee9290d4fb6417dba17cdad89232cb))
    - fmt ([`9147523`](https://github.com/xampprocky/tokei/commit/9147523b4eb1a405c4c9245efe42d1fb7d913d8e))
    - Implement literate languages and context parsing for markdown ([`9068a72`](https://github.com/xampprocky/tokei/commit/9068a7262262e21d886fdb03c58f69e90886554d))
    - Update lock ([`85ed728`](https://github.com/xampprocky/tokei/commit/85ed72849a812f05cbe1b0ff5579d16806a79fb1))
    - Bump serde_json from 1.0.54 to 1.0.55 ([`0209975`](https://github.com/xampprocky/tokei/commit/0209975c64ea5ea262f37670ad136222628d6248))
    - fmt ([`fe6a06a`](https://github.com/xampprocky/tokei/commit/fe6a06a0e3cdae7a9b2e0f205e1ff67b5510241f))
    - Refactor parse_lines, add literate support and continue working on contexts ([`c6ef1d9`](https://github.com/xampprocky/tokei/commit/c6ef1d998874c2ebf3b3daac1b7109589ac28662))
    - move test file ([`7eee4cb`](https://github.com/xampprocky/tokei/commit/7eee4cbea1c4d68d5ba408cd5fd61edc3d82ed88))
    - Bump serde_json from 1.0.53 to 1.0.54 ([`cb5c207`](https://github.com/xampprocky/tokei/commit/cb5c207fd7446abb8320b9a33d4104d6f15949ed))
    - Bump tera from 1.3.0 to 1.3.1 ([`4fe366d`](https://github.com/xampprocky/tokei/commit/4fe366d79c0aecbdadcbb0764c3e18325fee0d62))
    - clean up some code ([`860b53d`](https://github.com/xampprocky/tokei/commit/860b53d63c4448b6c38d85362364fb9aa4b45202))
    - fmt ([`f786793`](https://github.com/xampprocky/tokei/commit/f7867930cdd0660057eb6f62447e3827ad95b5f3))
    - Move parser from using Stats to CodeStats where appropiate ([`1283f0a`](https://github.com/xampprocky/tokei/commit/1283f0a4ce4d97c3a18a7faee686a07158f7e000))
    - Bump dashmap from 3.11.3 to 3.11.4 ([`6d04297`](https://github.com/xampprocky/tokei/commit/6d042978ee09fa1b76e35600579c8e438d519ef4))
    - Update get_config documentation ([`8e37ba8`](https://github.com/xampprocky/tokei/commit/8e37ba8e64b05bf63f3c01368f78631c2af93d00))
    - Bump dashmap from 3.11.2 to 3.11.3 ([`21fc3e6`](https://github.com/xampprocky/tokei/commit/21fc3e65a3ace0f854e4fc183d208b7831bab2f0))
    - Bump serde from 1.0.110 to 1.0.111 ([`a4af517`](https://github.com/xampprocky/tokei/commit/a4af517ef94e39ee3b3d4563018ebe262ccf214c))
    - Added better blank detection ([`9974f01`](https://github.com/xampprocky/tokei/commit/9974f017e897d2aef9820aa602f7e5b150589d22))
    - Re-org syntax.rs ([`0595bd3`](https://github.com/xampprocky/tokei/commit/0595bd3f9415810e6547579f44eea34a192ddd1c))
    - Add initial context to json and stats ([`733bc1f`](https://github.com/xampprocky/tokei/commit/733bc1ff05600bb103e3642fd13c8345281a7cda))
    - Bump regex from 1.3.7 to 1.3.9 ([`80f750e`](https://github.com/xampprocky/tokei/commit/80f750e796a56b232583c9de11a36fe052cb7c07))
    - Move codegen to use Tera ([`2342baa`](https://github.com/xampprocky/tokei/commit/2342baa0dd39381c39b194d640948572d7545d91))
    - fmt ([`0b98f31`](https://github.com/xampprocky/tokei/commit/0b98f310c50657cff374884e4b15e8b7565dba5a))
    - Replace lazy_static with once_cell ([`ec8edf2`](https://github.com/xampprocky/tokei/commit/ec8edf27e74f469fb42a447de3a39c042fa98b87))
</details>

# 11.2.1 (2020-05-28)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 7 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#538](https://github.com/xampprocky/tokei/issues/538), [#539](https://github.com/xampprocky/tokei/issues/539), [#540](https://github.com/xampprocky/tokei/issues/540), [#541](https://github.com/xampprocky/tokei/issues/541)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#538](https://github.com/xampprocky/tokei/issues/538)**
    - Add .csx extension for C# ([`223813a`](https://github.com/xampprocky/tokei/commit/223813a43c3bee36e16674bd652dabf716a21e88))
 * **[#539](https://github.com/xampprocky/tokei/issues/539)**
    - README.md: add instructions for installing via MacPorts ([`10494f2`](https://github.com/xampprocky/tokei/commit/10494f213f423ddcd73bc5eb1171ed9ec64a9a8e))
 * **[#540](https://github.com/xampprocky/tokei/issues/540)**
    - Fix rust-lang.org link in readme ([`ee19d28`](https://github.com/xampprocky/tokei/commit/ee19d28195647556607a7f0465ea96e94cf48eda))
 * **[#541](https://github.com/xampprocky/tokei/issues/541)**
    - Add Alloy language ([`156e7b7`](https://github.com/xampprocky/tokei/commit/156e7b7dc9da3e8c28db6ce0dfcac4e8fc24a09f))
 * **Uncategorized**
    - Bump version ([`b7388e2`](https://github.com/xampprocky/tokei/commit/b7388e2e5d7c6930af040ef80fb302a4ca7de9ed))
    - Update sub-dependencies ([`abddfaf`](https://github.com/xampprocky/tokei/commit/abddfafcf80f15cf6679fe1e06930257d51d4a6f))
    - Add sponsor message ([`fd34fd9`](https://github.com/xampprocky/tokei/commit/fd34fd916c2d23cb5cb4408f2259dcca168ae12f))
    - Update build.bash ([`53e8e0a`](https://github.com/xampprocky/tokei/commit/53e8e0aa12851e146a0da63b315e6533ed40fd38))
    - Update build.bash ([`48c5ed1`](https://github.com/xampprocky/tokei/commit/48c5ed1582e33ac7b40e0d1e5bda9f787accf751))
    - Update CHANGELOG.md ([`5204de7`](https://github.com/xampprocky/tokei/commit/5204de76a162abbdfb78d3dff4114c3013c78815))
    - Update CHANGELOG.md ([`84f58ce`](https://github.com/xampprocky/tokei/commit/84f58ced02195929ee18de175af53b4a6bb4f4c5))
    - Bump dashmap from 3.11.1 to 3.11.2 ([`9561e27`](https://github.com/xampprocky/tokei/commit/9561e2784c01a1a51184ad6622aace28f8a44809))
    - Remove unneeded checkouts ([`e0bfe78`](https://github.com/xampprocky/tokei/commit/e0bfe7859b89b965fd6647d7d8adbf85c9787a65))
</details>

# 11.2.0 (2020-05-20)

- @alexmaco Added shebang and env detection for Crystal.
- @NickHackman Updated both Vue and HTML to count CSS & JS comments as comments.
- @XAMPPRocky renamed Perl6's display name to Rakudo.
- @dbackeus Added `erb` extension for Ruby HTML.
- @kobataiwan Tokei will now check for a configuration file in your home
  directory as well as your current and configuration directory.
- @dependabot Updated dependencies

**Added Languages**
- @alexmaco Dhall
- @NickHackman Svelte
- @athas Futhark
- @morphy2k Gohtml
- @LucasMW Headache
- @rosasynstylae Tsx
- @XAMPPRocky OpenType Feature Files

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 34 commits contributed to the release over the course of 11 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 9 unique issues were worked on: [#516](https://github.com/xampprocky/tokei/issues/516), [#522](https://github.com/xampprocky/tokei/issues/522), [#523](https://github.com/xampprocky/tokei/issues/523), [#524](https://github.com/xampprocky/tokei/issues/524), [#526](https://github.com/xampprocky/tokei/issues/526), [#531](https://github.com/xampprocky/tokei/issues/531), [#532](https://github.com/xampprocky/tokei/issues/532), [#533](https://github.com/xampprocky/tokei/issues/533), [#534](https://github.com/xampprocky/tokei/issues/534)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#516](https://github.com/xampprocky/tokei/issues/516)**
    - Separate TSX and Typescript Support ([`add326e`](https://github.com/xampprocky/tokei/commit/add326ef783abe1f817bce66be754f2f64413a7c))
 * **[#522](https://github.com/xampprocky/tokei/issues/522)**
    - Add language Svelte ([`4448dc4`](https://github.com/xampprocky/tokei/commit/4448dc4ea127c5b8756d3d96bf4d685043e485e7))
 * **[#523](https://github.com/xampprocky/tokei/issues/523)**
    - Vue test ([`89f1fde`](https://github.com/xampprocky/tokei/commit/89f1fde9d3c44ec2596e60d540e4f6e7e259d8af))
 * **[#524](https://github.com/xampprocky/tokei/issues/524)**
    - Bump serde_yaml from 0.8.11 to 0.8.12 ([`b57655c`](https://github.com/xampprocky/tokei/commit/b57655c2675efb731bcd6d53994d65d38119471a))
 * **[#526](https://github.com/xampprocky/tokei/issues/526)**
    - Bump ignore from 0.4.14 to 0.4.15 ([`6eac7fa`](https://github.com/xampprocky/tokei/commit/6eac7fa4994e8b9c603a12d12b829d5ac847df60))
 * **[#531](https://github.com/xampprocky/tokei/issues/531)**
    - Support home directory in from_config_files ([`cb48ca3`](https://github.com/xampprocky/tokei/commit/cb48ca3c579b219c9315d17f286ab52733f815d2))
 * **[#532](https://github.com/xampprocky/tokei/issues/532)**
    - Add support for Dhall lang ([`e88eabe`](https://github.com/xampprocky/tokei/commit/e88eabe7d97cb7cc7e0721fe55eaf6483ceb9704))
 * **[#533](https://github.com/xampprocky/tokei/issues/533)**
    - Add shebang and env for crystal scripts ([`e554b3c`](https://github.com/xampprocky/tokei/commit/e554b3cb89c8d63ce294753193ec95c523117b5b))
 * **[#534](https://github.com/xampprocky/tokei/issues/534)**
    - Fixed HTML counting JS and CSS comments ([`765dfb1`](https://github.com/xampprocky/tokei/commit/765dfb109752a27818cca1b5d03fa906688142e6))
 * **Uncategorized**
    - Update mean_bean_deploy.yml ([`96c2871`](https://github.com/xampprocky/tokei/commit/96c2871ec6c60d74c1d35f405e9c58faf178adae))
    - Update mean_bean_deploy.yml ([`a9cb9c7`](https://github.com/xampprocky/tokei/commit/a9cb9c7776325bb59f0621c2b6dffcead00215a1))
    - Removed unneeded checkout ([`0f37cfa`](https://github.com/xampprocky/tokei/commit/0f37cfa478ea63e405a8a6405a1922fe3bcb93dc))
    - Update version and bump dependencies ([`cd60647`](https://github.com/xampprocky/tokei/commit/cd606478e2aef9ae87f1293e289c302b7170902e))
    - Switch to using dirs for home_dir ([`08e90ae`](https://github.com/xampprocky/tokei/commit/08e90ae6bd603cfe6530df363c14be734b7342fe))
    - Removed deprecated badges ([`e982271`](https://github.com/xampprocky/tokei/commit/e982271ae4f3d90b5d3a3b7cfb579917f225a800))
    - Change Windows to distribute exes instead of zips. ([`b641611`](https://github.com/xampprocky/tokei/commit/b64161144908e0d4da7b18cd604ee7f7511dbc88))
    - Update README.md ([`946f404`](https://github.com/xampprocky/tokei/commit/946f404f3f4b29eecae9ea1a3c937d48e5501a19))
    - Update and rename .tokeirc.example to tokei.example.toml ([`8506182`](https://github.com/xampprocky/tokei/commit/850618272d0be968cd144d1172c77b16562008d7))
    - Update .tokeirc.example ([`7cc40de`](https://github.com/xampprocky/tokei/commit/7cc40dee6a6fac0b6814141b73400ef9aa9e34c7))
    - Update README.md ([`6d9a163`](https://github.com/xampprocky/tokei/commit/6d9a1631063235d46b9b7add2d784a6babe4c721))
    - Update README.md ([`a85e5f6`](https://github.com/xampprocky/tokei/commit/a85e5f6619135ce1fca45aa542fe9778d513c580))
    - Update README.md ([`87b2f72`](https://github.com/xampprocky/tokei/commit/87b2f725d84a20c4b12bd3c4eaf487da5915048f))
    - Bump clap from 2.33.0 to 2.33.1 ([`5816b63`](https://github.com/xampprocky/tokei/commit/5816b631d947105cd5f6cf2f59882c81595254a1))
    - Bump serde_json from 1.0.52 to 1.0.53 ([`b0aceea`](https://github.com/xampprocky/tokei/commit/b0aceea1c36207f8dce82c1ebbff85b49a803adb))
    - Bump serde from 1.0.106 to 1.0.110 ([`f557936`](https://github.com/xampprocky/tokei/commit/f557936a3f099c1d4cb2d708b109f9026c44ea9d))
    - Deleted test.rs file from Rust quote fix ([`46827b6`](https://github.com/xampprocky/tokei/commit/46827b694d5d32245fe1baebd37b37bd0704aad7))
    - Javascript test ([`7ea0feb`](https://github.com/xampprocky/tokei/commit/7ea0feb645aea00632cdad92373ce08dacf380b2))
    - Added Typescript test ([`1a08ae2`](https://github.com/xampprocky/tokei/commit/1a08ae29b152a75af240c3b7731126b8f41f7bef))
    - Update syntax.rs ([`fc0abbb`](https://github.com/xampprocky/tokei/commit/fc0abbb729c8a427bfbb0db3acd35ae75a73fb6d))
    - Fixed typo in CONTRIBUTING.md ([`2d3ff62`](https://github.com/xampprocky/tokei/commit/2d3ff62427b9be20ba9544b9f9b5d393eb57ba35))
    - Updated CONTRIBUTING.md with rust.rs ([`c20b8e3`](https://github.com/xampprocky/tokei/commit/c20b8e31ce27799de74effcfc8bba4c3d479aed3))
    - Added Lifetimes edge case to Rust example ([`c32be30`](https://github.com/xampprocky/tokei/commit/c32be3069cfa52ae08369a552b87e0f01eed82dc))
    - Update README.md ([`45570fd`](https://github.com/xampprocky/tokei/commit/45570fdde9e4b45cfbf5b3b5a2fd6e079b47c1a9))
    - Display Perl6 as Rakudo ([`72dc3ff`](https://github.com/xampprocky/tokei/commit/72dc3ffba95d26da9655b067f1930d041b7c7ae5))
</details>

# 11.1.1 (2020-05-06)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 21 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#502](https://github.com/xampprocky/tokei/issues/502), [#503](https://github.com/xampprocky/tokei/issues/503), [#506](https://github.com/xampprocky/tokei/issues/506), [#509](https://github.com/xampprocky/tokei/issues/509), [#510](https://github.com/xampprocky/tokei/issues/510), [#513](https://github.com/xampprocky/tokei/issues/513)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#502](https://github.com/xampprocky/tokei/issues/502)**
    - Add Futhark ([`1a5c38d`](https://github.com/xampprocky/tokei/commit/1a5c38dd7b04528695526f0aa4d03353b336b982))
 * **[#503](https://github.com/xampprocky/tokei/issues/503)**
    - Added Headache Program Language ([`a98eda4`](https://github.com/xampprocky/tokei/commit/a98eda4f3cee7abb23aa7ba9411f24b67babbfee))
 * **[#506](https://github.com/xampprocky/tokei/issues/506)**
    - Add Go HTML template support ([`ab1b317`](https://github.com/xampprocky/tokei/commit/ab1b317621a4d8a4951fb3f1f8cdd1a79c8bd663))
 * **[#509](https://github.com/xampprocky/tokei/issues/509)**
    - Bump git2 from 0.13.2 to 0.13.5 ([`60e50bd`](https://github.com/xampprocky/tokei/commit/60e50bd848dc87dfb98b27a51292a6df53ad5c0c))
 * **[#510](https://github.com/xampprocky/tokei/issues/510)**
    - Add .erb extension for "Ruby HTML" ([`45ef479`](https://github.com/xampprocky/tokei/commit/45ef479b47bdd2eb4abc5e51d71b63000ca8034b))
 * **[#513](https://github.com/xampprocky/tokei/issues/513)**
    - Bump term_size from 0.3.1 to 0.3.2 ([`bf113b4`](https://github.com/xampprocky/tokei/commit/bf113b45b279cbe25b0d10594519ea242182c424))
 * **Uncategorized**
    - Bump version and sub-dependencies ([`41bbb95`](https://github.com/xampprocky/tokei/commit/41bbb9509ffdbc5980763901a37654e038a84ed2))
    - Fix Rust quotes ([`d860480`](https://github.com/xampprocky/tokei/commit/d86048001a6cc448eec4e6e82df8621d7f489cd0))
    - Bump serde_json from 1.0.51 to 1.0.52 ([`4cf32a5`](https://github.com/xampprocky/tokei/commit/4cf32a562de13257a72640700220b0126a261d4d))
    - Bump regex from 1.3.6 to 1.3.7 ([`bb6f2e1`](https://github.com/xampprocky/tokei/commit/bb6f2e1d682eb7959fa02fd450c02509095e4689))
    - Bump dashmap from 3.11.0 to 3.11.1 ([`3639efe`](https://github.com/xampprocky/tokei/commit/3639efe5b77cf125cef16339e3f5b41e93e9bfcd))
    - Add OpenType Feature File ([`5225f61`](https://github.com/xampprocky/tokei/commit/5225f61d75ff67fbfe6553669b3144ab3b756e28))
    - Don't make pre-releases when deploying ([`95405c6`](https://github.com/xampprocky/tokei/commit/95405c623a23f3114acb2d5f0d72f53eb5db1fd0))
</details>

# 11.1.0 (2020-04-13)

**Added Languages**

- @rubdos Arduino
- @LuqueDaniel Pan
- @itkovian Ren'Py

- Added `LanguageType::shebangs`, `LanguageType::from_file_extension`, and
  `LanguageType::from_shebang`. (@solanav)


## New Features

 - <csr-id-1425c670a7325ea7aa57e96dc8a35e8711adcd31/> add support for the Pan DSL

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 30 commits contributed to the release over the course of 23 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 17 unique issues were worked on: [#469](https://github.com/xampprocky/tokei/issues/469), [#470](https://github.com/xampprocky/tokei/issues/470), [#471](https://github.com/xampprocky/tokei/issues/471), [#473](https://github.com/xampprocky/tokei/issues/473), [#475](https://github.com/xampprocky/tokei/issues/475), [#476](https://github.com/xampprocky/tokei/issues/476), [#477](https://github.com/xampprocky/tokei/issues/477), [#479](https://github.com/xampprocky/tokei/issues/479), [#483](https://github.com/xampprocky/tokei/issues/483), [#485](https://github.com/xampprocky/tokei/issues/485), [#486](https://github.com/xampprocky/tokei/issues/486), [#487](https://github.com/xampprocky/tokei/issues/487), [#493](https://github.com/xampprocky/tokei/issues/493), [#495](https://github.com/xampprocky/tokei/issues/495), [#497](https://github.com/xampprocky/tokei/issues/497), [#499](https://github.com/xampprocky/tokei/issues/499), [#500](https://github.com/xampprocky/tokei/issues/500)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#469](https://github.com/xampprocky/tokei/issues/469)**
    - Added an article ([`3a1ffbb`](https://github.com/xampprocky/tokei/commit/3a1ffbb35f1d53b092659f41d531d290231e0f9c))
 * **[#470](https://github.com/xampprocky/tokei/issues/470)**
    - Updated README.md to reflect support languages ([`386cfce`](https://github.com/xampprocky/tokei/commit/386cfce329a0bcc721ca7b59ac1f6f08fd578927))
 * **[#471](https://github.com/xampprocky/tokei/issues/471)**
    - Fix shebang language detection ([`5cf5bd2`](https://github.com/xampprocky/tokei/commit/5cf5bd235b9745ada7b7cbd5a058a39d99f83462))
 * **[#473](https://github.com/xampprocky/tokei/issues/473)**
    - Bump ignore from 0.4.12 to 0.4.13 ([`d0e9f75`](https://github.com/xampprocky/tokei/commit/d0e9f75b235aae7ec9b75122ba39f0191e0835e4))
 * **[#475](https://github.com/xampprocky/tokei/issues/475)**
    - Add Arduino support ([`da8053c`](https://github.com/xampprocky/tokei/commit/da8053c96ed286bb55d296e6797aee09aa56704f))
 * **[#476](https://github.com/xampprocky/tokei/issues/476)**
    - Bump regex from 1.3.5 to 1.3.6 ([`4716517`](https://github.com/xampprocky/tokei/commit/4716517d482490d08bdd689fe86e8c8e820bdcf7))
 * **[#477](https://github.com/xampprocky/tokei/issues/477)**
    - Adds support for Ren'Py scripting ([`337b7b4`](https://github.com/xampprocky/tokei/commit/337b7b4d8ddc89bea80ab006b62ceb86010990cc))
 * **[#479](https://github.com/xampprocky/tokei/issues/479)**
    - add support for the Pan DSL ([`1425c67`](https://github.com/xampprocky/tokei/commit/1425c670a7325ea7aa57e96dc8a35e8711adcd31))
 * **[#483](https://github.com/xampprocky/tokei/issues/483)**
    - Bump ignore from 0.4.13 to 0.4.14 ([`07d4759`](https://github.com/xampprocky/tokei/commit/07d4759dd4c38278ab34819d1b67bfb48008a5bd))
 * **[#485](https://github.com/xampprocky/tokei/issues/485)**
    - Bump dashmap from 3.7.0 to 3.9.0 ([`34c40f9`](https://github.com/xampprocky/tokei/commit/34c40f939b59c97009ec6bc6de9ff1633654310f))
 * **[#486](https://github.com/xampprocky/tokei/issues/486)**
    - Fast exit parse_end_of_quote with ? operator ([`7d143f1`](https://github.com/xampprocky/tokei/commit/7d143f1c193b5a66a3d4f3913893942e319b5e34))
 * **[#487](https://github.com/xampprocky/tokei/issues/487)**
    - Check option in outer loop ([`38a6c96`](https://github.com/xampprocky/tokei/commit/38a6c96460004e67bc60907635d36fee17ad294c))
 * **[#493](https://github.com/xampprocky/tokei/issues/493)**
    - fixing some minor typos within test strings ([`cabc217`](https://github.com/xampprocky/tokei/commit/cabc2176401f3132253533ce49b4ce379498863c))
 * **[#495](https://github.com/xampprocky/tokei/issues/495)**
    - Bump dashmap from 3.9.1 to 3.10.0 ([`ad77662`](https://github.com/xampprocky/tokei/commit/ad7766215821ae5c62cd63b7edd65455c814a816))
 * **[#497](https://github.com/xampprocky/tokei/issues/497)**
    - Bump dashmap from 3.10.0 to 3.11.0 ([`fcf679e`](https://github.com/xampprocky/tokei/commit/fcf679eab12abbf2463fb57b9e08d31fe4196439))
 * **[#499](https://github.com/xampprocky/tokei/issues/499)**
    - Bump git2 from 0.13.0 to 0.13.2 ([`932c3ed`](https://github.com/xampprocky/tokei/commit/932c3ed94314611d60038760a3e3723cc14d6161))
 * **[#500](https://github.com/xampprocky/tokei/issues/500)**
    - Bump parking_lot from 0.10.0 to 0.10.2 ([`707c36d`](https://github.com/xampprocky/tokei/commit/707c36d288843c3fc1c75ae8ebcca35bc0218f5c))
 * **Uncategorized**
    - Version bump to 11.1.0 and update dependencies ([`c33dfe5`](https://github.com/xampprocky/tokei/commit/c33dfe574925d1c88cae6602e433c4c233db6f80))
    - Bump serde from 1.0.105 to 1.0.106 ([`31c03f1`](https://github.com/xampprocky/tokei/commit/31c03f1011ef291811a4f0220e573e3a6648eb3f))
    - Bump serde_json from 1.0.50 to 1.0.51 ([`bb50208`](https://github.com/xampprocky/tokei/commit/bb50208296892dfc384c949ca9df27a6cdcde49b))
    - Fixed stats statistics and add test ([`c4a4734`](https://github.com/xampprocky/tokei/commit/c4a4734967547021e9f3ddcc5c57d9931eef0c65))
    - Fixed total lines in individual stats being slightly inaccurate ([`d438d8a`](https://github.com/xampprocky/tokei/commit/d438d8aadf96180cca806e2f535d5be8b5f750a6))
    - Bump dashmap from 3.9.0 to 3.9.1 ([`9f7a5f6`](https://github.com/xampprocky/tokei/commit/9f7a5f6b616b47063a9f75bdac0ce1e37dd1376d))
    - Fix dependabot triggering CI twice. ([`e07223d`](https://github.com/xampprocky/tokei/commit/e07223d64a03babc7a97ada54257b6c148bf1717))
    - Bump serde_json from 1.0.48 to 1.0.50 ([`e0ceca1`](https://github.com/xampprocky/tokei/commit/e0ceca1351eac75e276245f4b9db7f9006aed95a))
    - Update language_type.hbs.rs ([`8562963`](https://github.com/xampprocky/tokei/commit/8562963468cedfd8e26eb7bbde28473899a9a698))
    - Add single quotes syntax to Rust. ([`38f4727`](https://github.com/xampprocky/tokei/commit/38f4727f074c91e5f5fad28165bffe95d30f12f7))
    - Update README.md ([`7094fb2`](https://github.com/xampprocky/tokei/commit/7094fb234f20d74c01f0c14bd265a4d31fdc4071))
    - Update README.md ([`a3bf9d2`](https://github.com/xampprocky/tokei/commit/a3bf9d2470cca142089d4afc25716d6b263ec34e))
    - Update README.md ([`8571556`](https://github.com/xampprocky/tokei/commit/8571556bf4b36b2a0eb1398cf1410b653de9679f))
</details>

# 11.0.0 (2020-03-21)

**Added languages**

- @bwidawsk GNU Assembly, GDB Script
- @isker Dust, Apache Velocity
- @andreblanke FreeMarker


Thanks to some major internal refactoring, Tokei has received significant
performance improvements, and is now one of the fastest code counters across any
size of codebase. With Tokei 11 showing up to 40–60% faster results than tokei's
previous version. To showcase the improvements I've highlighted benchmarks
of counting five differently sized codebases. Redis (~220k lines), Rust (~16M
lines), and the Unreal Engine (~37.5M lines). In every one of these benchmarks
Tokei 11 performed the best by a noticeable margin.

*All benchmarks were done on a 15-inch MacBook Pro, with a 2.7GHz Intel Core i7
processor and 16GB 2133 MHz LPDDR3 RAM running macOS Catalina 10.15.3. Your
mileage may vary, All benchmarks were done using [hyperfine], using default
settings for all programs.*

### Tokei
**Note** This benchmark is not accurate due to `tokei` and `loc` both taking
less than 5ms to complete, there is a high degree of error between the times and
should mostly be considered equivalent. However it is included because it is
notable that `scc` takes nearly 3x as long to complete on smaller codebases
(~5k lines).
![Graph comparing programs running on the tokei source code](https://docs.google.com/spreadsheets/d/e/2PACX-1vRN2Um3G9Mn4Bg6UVWwgntsMy4faZMIP3EDjAfY5Y6Tav7T5z1TxVKmPu7wUNIpUSsSJDfCNH0SAKBB/pubchart?oid=1242634543&format=image)

### Redis
![Graph comparing programs running on the redis source code](https://docs.google.com/spreadsheets/d/e/2PACX-1vRN2Um3G9Mn4Bg6UVWwgntsMy4faZMIP3EDjAfY5Y6Tav7T5z1TxVKmPu7wUNIpUSsSJDfCNH0SAKBB/pubchart?oid=2009389097&format=image)

### Rust
![Graph comparing programs running on the rust source code](https://docs.google.com/spreadsheets/d/e/2PACX-1vRN2Um3G9Mn4Bg6UVWwgntsMy4faZMIP3EDjAfY5Y6Tav7T5z1TxVKmPu7wUNIpUSsSJDfCNH0SAKBB/pubchart?oid=424069399&format=image)

### Unreal
![Graph comparing programs running on the unreal source code](https://docs.google.com/spreadsheets/d/e/2PACX-1vRN2Um3G9Mn4Bg6UVWwgntsMy4faZMIP3EDjAfY5Y6Tav7T5z1TxVKmPu7wUNIpUSsSJDfCNH0SAKBB/pubchart?oid=439405321&format=image)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 42 commits contributed to the release over the course of 32 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 14 unique issues were worked on: [#425](https://github.com/xampprocky/tokei/issues/425), [#447](https://github.com/xampprocky/tokei/issues/447), [#449](https://github.com/xampprocky/tokei/issues/449), [#450](https://github.com/xampprocky/tokei/issues/450), [#451](https://github.com/xampprocky/tokei/issues/451), [#453](https://github.com/xampprocky/tokei/issues/453), [#454](https://github.com/xampprocky/tokei/issues/454), [#456](https://github.com/xampprocky/tokei/issues/456), [#457](https://github.com/xampprocky/tokei/issues/457), [#460](https://github.com/xampprocky/tokei/issues/460), [#462](https://github.com/xampprocky/tokei/issues/462), [#463](https://github.com/xampprocky/tokei/issues/463), [#464](https://github.com/xampprocky/tokei/issues/464), [#466](https://github.com/xampprocky/tokei/issues/466)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#425](https://github.com/xampprocky/tokei/issues/425)**
    - Add FreeMarker language support ([`633a77b`](https://github.com/xampprocky/tokei/commit/633a77b982171c91fa0775671637415d6c62f97f))
 * **[#447](https://github.com/xampprocky/tokei/issues/447)**
    - fix heading level, rename macos/homebrew ([`5fad0e9`](https://github.com/xampprocky/tokei/commit/5fad0e933e7d6d3cedf1c8923462f9f30d3b64e7))
 * **[#449](https://github.com/xampprocky/tokei/issues/449)**
    - Update GitHub action ([`84814ab`](https://github.com/xampprocky/tokei/commit/84814ab875287398c067258b63de2221079b2c96))
 * **[#450](https://github.com/xampprocky/tokei/issues/450)**
    - Add support for Velocity and Dust ([`4e97237`](https://github.com/xampprocky/tokei/commit/4e9723719b621dfc3bbc9f1d8b1a9a82dc21374e))
 * **[#451](https://github.com/xampprocky/tokei/issues/451)**
    - Bump hex from 0.4.1 to 0.4.2 ([`1b2328f`](https://github.com/xampprocky/tokei/commit/1b2328fa5cf04cb5dcb5f6b2bbd7ad17c340d120))
 * **[#453](https://github.com/xampprocky/tokei/issues/453)**
    - Bump crossbeam-channel from 0.4.0 to 0.4.2 ([`e52ad2a`](https://github.com/xampprocky/tokei/commit/e52ad2aeae0ea9a0fdba5374fe2a612dd68bdb84))
 * **[#454](https://github.com/xampprocky/tokei/issues/454)**
    - Bump git2 from 0.11.0 to 0.12.0 ([`6ab6ea3`](https://github.com/xampprocky/tokei/commit/6ab6ea306fd83075058b408f391bf9b260d338ab))
 * **[#456](https://github.com/xampprocky/tokei/issues/456)**
    - Add support for GDB scripts ([`1b3e6d0`](https://github.com/xampprocky/tokei/commit/1b3e6d01fa38a06613401bf0e24e883869c567ba))
 * **[#457](https://github.com/xampprocky/tokei/issues/457)**
    - Split out GNU style assembly ([`c8da886`](https://github.com/xampprocky/tokei/commit/c8da8869ae4543dfd7a04609670b4313e1127beb))
 * **[#460](https://github.com/xampprocky/tokei/issues/460)**
    - Bump regex from 1.3.4 to 1.3.5 ([`a9667b9`](https://github.com/xampprocky/tokei/commit/a9667b95c0b5dbc6794b5e4fee8dedb4be2ac3e2))
 * **[#462](https://github.com/xampprocky/tokei/issues/462)**
    - Bump ignore from 0.4.11 to 0.4.12 ([`bbc52ed`](https://github.com/xampprocky/tokei/commit/bbc52ede2a432eaa03c217c694165e2d9106ff2b))
 * **[#463](https://github.com/xampprocky/tokei/issues/463)**
    - Bump git2 from 0.12.0 to 0.13.0 ([`df72338`](https://github.com/xampprocky/tokei/commit/df72338cf710a054b8977f6e5abb38bc5e792471))
 * **[#464](https://github.com/xampprocky/tokei/issues/464)**
    - Bump grep-searcher from 0.1.6 to 0.1.7 ([`a42f043`](https://github.com/xampprocky/tokei/commit/a42f043daf12735b6a23928ad758e0ba75882266))
 * **[#466](https://github.com/xampprocky/tokei/issues/466)**
    - Bump serde from 1.0.104 to 1.0.105 ([`2e12cf6`](https://github.com/xampprocky/tokei/commit/2e12cf6faae66eff5187464e5ef140f5e8c19d9b))
 * **Uncategorized**
    - Version 11.0.0 ([`e3553a4`](https://github.com/xampprocky/tokei/commit/e3553a44f740c69268d131797fe94f49016ab83c))
    - Remove old comparison document ([`8058476`](https://github.com/xampprocky/tokei/commit/80584764d734ba78b50912273937d581b5149a93))
    - Add parallel count heuristic ([`a8afa67`](https://github.com/xampprocky/tokei/commit/a8afa670571a34f5d371d2debcfeac30b57af44d))
    - Set bash shell on windows. ([`2a3f735`](https://github.com/xampprocky/tokei/commit/2a3f735260ceabc302612584b9aefb6760d9e698))
    - Remove 32bit Apple platforms ([`046482f`](https://github.com/xampprocky/tokei/commit/046482fd9137aec75a65ada0a80800d5bacde6fe))
    - Use thinLTO ([`cc6fc12`](https://github.com/xampprocky/tokei/commit/cc6fc124902287a784da0c7647057b0bb620432e))
    - refactor SyntaxCounter to be concurrent, and share language information. ([`bdd1431`](https://github.com/xampprocky/tokei/commit/bdd1431a2c38ce94052b37a6238e50455f37fbc7))
    - update mailmap ([`16008c9`](https://github.com/xampprocky/tokei/commit/16008c93d8c3cf3be4b983f1cdec2b6cb94ba364))
    - Create .mailmap ([`69ad12f`](https://github.com/xampprocky/tokei/commit/69ad12f9642d8bccef54060d58c3b651fb424cbf))
    - Update CONTRIBUTING.md ([`c18712b`](https://github.com/xampprocky/tokei/commit/c18712be71b6b341c886341b2b369c131f42eb1f))
    - Remove base property, use AhoCorasick for searching ([`12e4783`](https://github.com/xampprocky/tokei/commit/12e47836b102e36789182aa8c0a51c7faffc8568))
    - Format JSON ([`f356d27`](https://github.com/xampprocky/tokei/commit/f356d27ab21e0f93839da90393c0edf9225740c2))
    - Use aho_corasick and fix escaping applying outside the string context. ([`a536707`](https://github.com/xampprocky/tokei/commit/a536707b9e805ac400cafc0645b77463510ecf3d))
    - Update README.md ([`abe4f01`](https://github.com/xampprocky/tokei/commit/abe4f017352e05d8b1c981bf51ddea9ab10ad1e7))
    - Update README.md ([`eaad845`](https://github.com/xampprocky/tokei/commit/eaad84508df41da2afdfbfcfc90563deddf11b36))
    - Update cli.rs ([`ae32940`](https://github.com/xampprocky/tokei/commit/ae32940cd044daf41492aed74b5bbf7ff34a9924))
    - Update README.md ([`c59f4c0`](https://github.com/xampprocky/tokei/commit/c59f4c00cdb9d5368982d1dce42b7eb2df2e7198))
    - Update README.md ([`b84d25a`](https://github.com/xampprocky/tokei/commit/b84d25aaf2f3089abb816c09e4e67bcdf5727f6a))
    - Update README.md ([`3b82a78`](https://github.com/xampprocky/tokei/commit/3b82a7892ac9bca3a54a80b08b1a7f8630810f58))
    - Update README.md ([`7189721`](https://github.com/xampprocky/tokei/commit/71897213343af3c55407f9159ce01594c5f3d0cb))
    - Update README.md ([`172ba0e`](https://github.com/xampprocky/tokei/commit/172ba0e8fe8cf423c9d0b8ba3f40b3f52a577167))
    - Update README.md ([`c6c2176`](https://github.com/xampprocky/tokei/commit/c6c2176db32373c48f37fb9b69d5514bca5ecb6d))
    - Delete appveyor.yml ([`9b65392`](https://github.com/xampprocky/tokei/commit/9b65392ee4af48dd6af87a559a7353cdfaa71082))
    - Delete .travis.yml ([`13649ec`](https://github.com/xampprocky/tokei/commit/13649ec79783ca797b66cc35430bf906694c0a45))
    - Delete rust.yml ([`5edda0d`](https://github.com/xampprocky/tokei/commit/5edda0d381199e22dc6643892e50374f0a69087c))
    - move action ([`0edc9d2`](https://github.com/xampprocky/tokei/commit/0edc9d2fa8c617a891d545431b696522cc437632))
    - Create Ferris' Mean Bean Machine ([`2d25978`](https://github.com/xampprocky/tokei/commit/2d2597837da43379387ab9ee6b38afad94ccedf0))
    - Update README.md ([`458af76`](https://github.com/xampprocky/tokei/commit/458af76826e037bc6e927dd80bd03970fdd501aa))
</details>

# 10.1.2 (2020-02-16)

- Added `pyw` extension to Python.
- Updated dependencies

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 32 commits contributed to the release over the course of 40 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 16 unique issues were worked on: [#424](https://github.com/xampprocky/tokei/issues/424), [#427](https://github.com/xampprocky/tokei/issues/427), [#428](https://github.com/xampprocky/tokei/issues/428), [#429](https://github.com/xampprocky/tokei/issues/429), [#432](https://github.com/xampprocky/tokei/issues/432), [#433](https://github.com/xampprocky/tokei/issues/433), [#434](https://github.com/xampprocky/tokei/issues/434), [#435](https://github.com/xampprocky/tokei/issues/435), [#436](https://github.com/xampprocky/tokei/issues/436), [#438](https://github.com/xampprocky/tokei/issues/438), [#439](https://github.com/xampprocky/tokei/issues/439), [#440](https://github.com/xampprocky/tokei/issues/440), [#441](https://github.com/xampprocky/tokei/issues/441), [#442](https://github.com/xampprocky/tokei/issues/442), [#443](https://github.com/xampprocky/tokei/issues/443), [#445](https://github.com/xampprocky/tokei/issues/445)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#424](https://github.com/xampprocky/tokei/issues/424)**
    - Bump handlebars from 2.0.2 to 2.0.4 ([`8984142`](https://github.com/xampprocky/tokei/commit/8984142e999afad0d638954c54d941e25ce842f5))
 * **[#427](https://github.com/xampprocky/tokei/issues/427)**
    - Bump regex from 1.3.1 to 1.3.3 ([`73fc5b0`](https://github.com/xampprocky/tokei/commit/73fc5b0580d7ea8399cf058c20b51bb93a62b9ec))
 * **[#428](https://github.com/xampprocky/tokei/issues/428)**
    - Bump ignore from 0.4.10 to 0.4.11 ([`107058c`](https://github.com/xampprocky/tokei/commit/107058c4885b34635c282b80340e5a9b95ea7d1f))
 * **[#429](https://github.com/xampprocky/tokei/issues/429)**
    - Bump serde_cbor from 0.10.2 to 0.11.1 ([`d30cf74`](https://github.com/xampprocky/tokei/commit/d30cf747b9ba496f35efe47850b3da83e91d3567))
 * **[#432](https://github.com/xampprocky/tokei/issues/432)**
    - Bump toml from 0.5.5 to 0.5.6 ([`090d646`](https://github.com/xampprocky/tokei/commit/090d6463d44ba684a450c23122c76ff11c5a7294))
 * **[#433](https://github.com/xampprocky/tokei/issues/433)**
    - Bump serde_json from 1.0.44 to 1.0.45 ([`bb06a8a`](https://github.com/xampprocky/tokei/commit/bb06a8abef8b240e594df645c93ac26d1fdce508))
 * **[#434](https://github.com/xampprocky/tokei/issues/434)**
    - Bump handlebars from 2.0.4 to 3.0.0 ([`da55ae5`](https://github.com/xampprocky/tokei/commit/da55ae5948d7cd7ee5ed741e41e46ef94a316af5))
 * **[#435](https://github.com/xampprocky/tokei/issues/435)**
    - Bump handlebars from 3.0.0 to 3.0.1 ([`5874a1e`](https://github.com/xampprocky/tokei/commit/5874a1e020b3ac229858d80ecf4166bba7cf72b0))
 * **[#436](https://github.com/xampprocky/tokei/issues/436)**
    - fix contributor link ([`969b772`](https://github.com/xampprocky/tokei/commit/969b772fa0aeca8e33826f8921066b079ea2dd7d))
 * **[#438](https://github.com/xampprocky/tokei/issues/438)**
    - Bump regex from 1.3.3 to 1.3.4 ([`1083084`](https://github.com/xampprocky/tokei/commit/1083084a2111774ff63d4c42df18d7530c96c4c9))
 * **[#439](https://github.com/xampprocky/tokei/issues/439)**
    - remove duplicated CBOR lines, replace with toml ([`4dcb833`](https://github.com/xampprocky/tokei/commit/4dcb8332c1206cb8e7745dc605df4578c9e63644))
 * **[#440](https://github.com/xampprocky/tokei/issues/440)**
    - Bump serde_json from 1.0.45 to 1.0.46 ([`e47ffa1`](https://github.com/xampprocky/tokei/commit/e47ffa1ecfaba5d98ad426cfe83b29056e7a7c8a))
 * **[#441](https://github.com/xampprocky/tokei/issues/441)**
    - Bump hex from 0.4.0 to 0.4.1 ([`10a21ba`](https://github.com/xampprocky/tokei/commit/10a21bae394ad25ab5deed44532b1b631489bc3c))
 * **[#442](https://github.com/xampprocky/tokei/issues/442)**
    - Bump serde_json from 1.0.46 to 1.0.47 ([`21062cd`](https://github.com/xampprocky/tokei/commit/21062cde02cc64a508b49911a68f25dc586c1cf0))
 * **[#443](https://github.com/xampprocky/tokei/issues/443)**
    - Bump encoding_rs_io from 0.1.6 to 0.1.7 ([`a4faf1a`](https://github.com/xampprocky/tokei/commit/a4faf1a5761cb97fb02337f6166ecd3cc01d237c))
 * **[#445](https://github.com/xampprocky/tokei/issues/445)**
    - .pyw is a valid Python extension ([`0d8917d`](https://github.com/xampprocky/tokei/commit/0d8917dc153538087605a9dba6bdece4ae4ac3d1))
 * **Uncategorized**
    - Minor refactor and fix warning ([`fdf3f8c`](https://github.com/xampprocky/tokei/commit/fdf3f8cb279a7aeac0696c87e5d8b0cd946e4f9e))
    - Update appveyor.yml ([`93921c2`](https://github.com/xampprocky/tokei/commit/93921c2deb2c2b5e703aa247c42cbc52452631b1))
    - Update before_deploy.sh ([`b14bd1e`](https://github.com/xampprocky/tokei/commit/b14bd1eddad4394048c22cd377b3de984a539e99))
    - Update CONTRIBUTING.md ([`887a3f4`](https://github.com/xampprocky/tokei/commit/887a3f4439abc2f2025fb7a4e22dce4112f7bedd))
    - Bump version to 10.1.2 ([`4ef21a3`](https://github.com/xampprocky/tokei/commit/4ef21a3e059b21a249daffb76d288750bc8f706c))
    - Use new auth token ([`b56d69b`](https://github.com/xampprocky/tokei/commit/b56d69bced4d93c166c3805a282c823c321c2e7b))
    - fix travis ([`781124c`](https://github.com/xampprocky/tokei/commit/781124caf1e2157a57ab8a8ccd8c31d6a7c63d1e))
    - fix appveyor ([`ce0e542`](https://github.com/xampprocky/tokei/commit/ce0e542f421aded1c6eca91c65ffe5d925ef54f3))
    - Update lock dependencies ([`74b1463`](https://github.com/xampprocky/tokei/commit/74b14639d64241bf0d7da3954e5b46cfb3c0236c))
    - Fixed documentation example ([`38ef393`](https://github.com/xampprocky/tokei/commit/38ef393c9370e1af60e6ce3993b35c1d644bbc8f))
    - Remove use of description ([`4cdc98a`](https://github.com/xampprocky/tokei/commit/4cdc98a76b3facb0a2469332c76995cbaf8e81f8))
    - Disable FreeBSD ([`50af74e`](https://github.com/xampprocky/tokei/commit/50af74e065a754d9de4b134d49f82dbf2a757930))
    - Update Travis & Appveyor to latest trust template ([`17447c5`](https://github.com/xampprocky/tokei/commit/17447c598c5610be515df32f4f75bc26b9515ed8))
    - Create FUNDING.yml ([`8ff120c`](https://github.com/xampprocky/tokei/commit/8ff120cfaae2dd8cf65c3b758bf58666d2ecb4cb))
    - Update README.md ([`b3524a1`](https://github.com/xampprocky/tokei/commit/b3524a1348a7ee9e9a0b770809dc605dad767bf4))
    - Remove unneeded braces ([`4f902fe`](https://github.com/xampprocky/tokei/commit/4f902feb94db7c7791bd0e28404cd20fbd8e6fc5))
</details>

# 10.1.1 (2019-12-28)

- Fixed `.tokeignore` always working even when `--no-ignore` is present.
- Updated dependencies

**Added languages**

- @erikaxel Gherkin (Cucumber)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 37 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on: [#405](https://github.com/xampprocky/tokei/issues/405), [#408](https://github.com/xampprocky/tokei/issues/408), [#410](https://github.com/xampprocky/tokei/issues/410), [#414](https://github.com/xampprocky/tokei/issues/414), [#415](https://github.com/xampprocky/tokei/issues/415), [#416](https://github.com/xampprocky/tokei/issues/416), [#417](https://github.com/xampprocky/tokei/issues/417), [#418](https://github.com/xampprocky/tokei/issues/418)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#405](https://github.com/xampprocky/tokei/issues/405)**
    - Bump rayon from 1.2.0 to 1.2.1 ([`c6645a3`](https://github.com/xampprocky/tokei/commit/c6645a3f8f844913e799eaa0a2baf633c951bcf6))
 * **[#408](https://github.com/xampprocky/tokei/issues/408)**
    - Added Gherkin (Cucumber) ([`e827e38`](https://github.com/xampprocky/tokei/commit/e827e38dcf09d2bf2fd406b889c3fa0c007c294f))
 * **[#410](https://github.com/xampprocky/tokei/issues/410)**
    - Bump serde_derive from 1.0.102 to 1.0.103 ([`300a3a2`](https://github.com/xampprocky/tokei/commit/300a3a23cc9b6534ad5752f85b5642ba219628f0))
 * **[#414](https://github.com/xampprocky/tokei/issues/414)**
    - Bump serde_json from 1.0.41 to 1.0.44 ([`0598b4f`](https://github.com/xampprocky/tokei/commit/0598b4fa90fa853b7f8fa0fcc0503e31ba544cdb))
 * **[#415](https://github.com/xampprocky/tokei/issues/415)**
    - Bump git2 from 0.10.1 to 0.11.0 ([`6ebf893`](https://github.com/xampprocky/tokei/commit/6ebf893dcff3721b6aa083e3d1131fc902f035b2))
 * **[#416](https://github.com/xampprocky/tokei/issues/416)**
    - Bump serde_derive from 1.0.103 to 1.0.104 ([`0b55f21`](https://github.com/xampprocky/tokei/commit/0b55f216e32ff88d5af6f98cdd1dc5e8be7ded66))
 * **[#417](https://github.com/xampprocky/tokei/issues/417)**
    - Bump serde from 1.0.102 to 1.0.104 ([`467cffe`](https://github.com/xampprocky/tokei/commit/467cffeb8c3f9dfd7aea8e60499250558c3cf071))
 * **[#418](https://github.com/xampprocky/tokei/issues/418)**
    - Bump rayon from 1.2.1 to 1.3.0 ([`8f28d4b`](https://github.com/xampprocky/tokei/commit/8f28d4bdc49d887c01dc6ca2cebcba26f29ace10))
 * **Uncategorized**
    - Bump version to 10.1.1 ([`2129e27`](https://github.com/xampprocky/tokei/commit/2129e27d6a69320b19b3e51fac590dd5a74ffde5))
    - Update lockfile ([`3099dd2`](https://github.com/xampprocky/tokei/commit/3099dd2420926a1a6864e911ed8b67b0f8207b2a))
    - Fix custom ignore files not respecting no-ignore ([`fb07169`](https://github.com/xampprocky/tokei/commit/fb071699593196e3ccf19472407563aca056e876))
    - Update rust.yml ([`fd65927`](https://github.com/xampprocky/tokei/commit/fd6592728deee63f3703431fe1e998a8847ba30c))
</details>

# 10.1.0 (2019-11-12)

- Added `cjsx` extension to CoffeeScript.
- Tokei will now recognise files with `#!/usr/bin/env ruby` as Ruby.
- Updated dependencies.
- Tokei now uses `crossbeam` channels over `std::mpsc`, which should have a
  noticeable performance improvement on large repos.
- Improved documentation for `libtokei`.

**Added languages**

- @lzybkr PowerShell
- @turbo MoonScript
- @dtolnay Thrift
- @Tranzystorek FlatBuffers
- @NieDzejkob Emojicode
- @DanteFalzone0 HolyC
- @sci4me Odin
- @fkarg Rusty Object Notation (RON)

## Documentation

 - <csr-id-0a30359227b9c540b2d9e2ad5c365017fd6c9ccb/> fix typo

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 50 commits contributed to the release over the course of 130 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 36 unique issues were worked on: [#346](https://github.com/xampprocky/tokei/issues/346), [#348](https://github.com/xampprocky/tokei/issues/348), [#349](https://github.com/xampprocky/tokei/issues/349), [#351](https://github.com/xampprocky/tokei/issues/351), [#353](https://github.com/xampprocky/tokei/issues/353), [#355](https://github.com/xampprocky/tokei/issues/355), [#358](https://github.com/xampprocky/tokei/issues/358), [#362](https://github.com/xampprocky/tokei/issues/362), [#363](https://github.com/xampprocky/tokei/issues/363), [#364](https://github.com/xampprocky/tokei/issues/364), [#366](https://github.com/xampprocky/tokei/issues/366), [#367](https://github.com/xampprocky/tokei/issues/367), [#368](https://github.com/xampprocky/tokei/issues/368), [#369](https://github.com/xampprocky/tokei/issues/369), [#370](https://github.com/xampprocky/tokei/issues/370), [#372](https://github.com/xampprocky/tokei/issues/372), [#374](https://github.com/xampprocky/tokei/issues/374), [#377](https://github.com/xampprocky/tokei/issues/377), [#378](https://github.com/xampprocky/tokei/issues/378), [#379](https://github.com/xampprocky/tokei/issues/379), [#380](https://github.com/xampprocky/tokei/issues/380), [#381](https://github.com/xampprocky/tokei/issues/381), [#382](https://github.com/xampprocky/tokei/issues/382), [#384](https://github.com/xampprocky/tokei/issues/384), [#385](https://github.com/xampprocky/tokei/issues/385), [#387](https://github.com/xampprocky/tokei/issues/387), [#389](https://github.com/xampprocky/tokei/issues/389), [#392](https://github.com/xampprocky/tokei/issues/392), [#393](https://github.com/xampprocky/tokei/issues/393), [#395](https://github.com/xampprocky/tokei/issues/395), [#397](https://github.com/xampprocky/tokei/issues/397), [#399](https://github.com/xampprocky/tokei/issues/399), [#401](https://github.com/xampprocky/tokei/issues/401), [#402](https://github.com/xampprocky/tokei/issues/402), [#403](https://github.com/xampprocky/tokei/issues/403), [#404](https://github.com/xampprocky/tokei/issues/404)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#346](https://github.com/xampprocky/tokei/issues/346)**
    - Add PowerShell support ([`b36284f`](https://github.com/xampprocky/tokei/commit/b36284f9e13e854526a9f247fad76ca16a05f714))
 * **[#348](https://github.com/xampprocky/tokei/issues/348)**
    - fix typo ([`0a30359`](https://github.com/xampprocky/tokei/commit/0a30359227b9c540b2d9e2ad5c365017fd6c9ccb))
 * **[#349](https://github.com/xampprocky/tokei/issues/349)**
    - Fix warnings on trait objects without dyn ([`5323ca8`](https://github.com/xampprocky/tokei/commit/5323ca8761d411dce37514d50a939c1c0a11a9f7))
 * **[#351](https://github.com/xampprocky/tokei/issues/351)**
    - Add MoonScript to Languages ([`baa2a65`](https://github.com/xampprocky/tokei/commit/baa2a653442619a3cd3eb1e7c7c444fce781b7a4))
 * **[#353](https://github.com/xampprocky/tokei/issues/353)**
    - Added tests for C and SQL ([`ca5c141`](https://github.com/xampprocky/tokei/commit/ca5c141d4b03a42fe53b6a6e6dab27af8f5e3dc4))
 * **[#355](https://github.com/xampprocky/tokei/issues/355)**
    - 63: Add basic test for Go language ([`cdec6ef`](https://github.com/xampprocky/tokei/commit/cdec6efe5eeabe6ca09f21a26aeb581e7e8e924a))
 * **[#358](https://github.com/xampprocky/tokei/issues/358)**
    - Add thrift language ([`d888843`](https://github.com/xampprocky/tokei/commit/d8888438f89a18b9740d6049fdd42718880dbf78))
 * **[#362](https://github.com/xampprocky/tokei/issues/362)**
    - Add support for FlatBuffers ([`ee3c7ff`](https://github.com/xampprocky/tokei/commit/ee3c7ff9c75c37e242ae604c637158b14f62bdc2))
 * **[#363](https://github.com/xampprocky/tokei/issues/363)**
    - fix too long language names misaligning the output ([`5378efd`](https://github.com/xampprocky/tokei/commit/5378efddd5899b3a9685ab2e2842cf5499530503))
 * **[#364](https://github.com/xampprocky/tokei/issues/364)**
    - 63: Add test for YAML files ([`8500cba`](https://github.com/xampprocky/tokei/commit/8500cbaf4dc6d24e5df9d14c63042548e886af54))
 * **[#366](https://github.com/xampprocky/tokei/issues/366)**
    - Bump serde_derive from 1.0.97 to 1.0.101 ([`21e755b`](https://github.com/xampprocky/tokei/commit/21e755b0869d26cb7b0bbeafc58d3d253c15ee96))
 * **[#367](https://github.com/xampprocky/tokei/issues/367)**
    - Bump env_logger from 0.6.2 to 0.7.0 ([`7f748b0`](https://github.com/xampprocky/tokei/commit/7f748b0718172190621b22c757d5de58c214b407))
 * **[#368](https://github.com/xampprocky/tokei/issues/368)**
    - [Security] Bump spin from 0.5.0 to 0.5.2 ([`85e27aa`](https://github.com/xampprocky/tokei/commit/85e27aac4fa8526ab4332795ed6b4da901e06f36))
 * **[#369](https://github.com/xampprocky/tokei/issues/369)**
    - Bump lazy_static from 1.3.0 to 1.4.0 ([`7cd94f6`](https://github.com/xampprocky/tokei/commit/7cd94f6e3682198959a444833aac0e6140cbe598))
 * **[#370](https://github.com/xampprocky/tokei/issues/370)**
    - Bump serde_json from 1.0.40 to 1.0.41 ([`04357b8`](https://github.com/xampprocky/tokei/commit/04357b83a22a3d3cbb5bf4254f3b771e2abb6892))
 * **[#372](https://github.com/xampprocky/tokei/issues/372)**
    - Bump git2 from 0.9.1 to 0.10.1 ([`1659f40`](https://github.com/xampprocky/tokei/commit/1659f40172b1f8ca718b3c857219e21f829bb30c))
 * **[#374](https://github.com/xampprocky/tokei/issues/374)**
    - Bump serde_yaml from 0.8.9 to 0.8.11 ([`8c4aa44`](https://github.com/xampprocky/tokei/commit/8c4aa44b7e98a6aca27b3f62d448ec396a214cc2))
 * **[#377](https://github.com/xampprocky/tokei/issues/377)**
    - Bump regex from 1.1.9 to 1.2.0 ([`091e8d6`](https://github.com/xampprocky/tokei/commit/091e8d600e93a987d9ba91b4ac4c6594fc1214b4))
 * **[#378](https://github.com/xampprocky/tokei/issues/378)**
    - Bump toml from 0.5.1 to 0.5.3 ([`0b34d8f`](https://github.com/xampprocky/tokei/commit/0b34d8fbc029bb19e5965ed74b472c62ab586b82))
 * **[#379](https://github.com/xampprocky/tokei/issues/379)**
    - Bump rayon from 1.1.0 to 1.2.0 ([`94660db`](https://github.com/xampprocky/tokei/commit/94660dbfdadf2abddb4a707109c848a175a7f777))
 * **[#380](https://github.com/xampprocky/tokei/issues/380)**
    - Bump serde_cbor from 0.10.1 to 0.10.2 ([`eb1a627`](https://github.com/xampprocky/tokei/commit/eb1a6274d032de5dd21a92bd4c9660b72d0df782))
 * **[#381](https://github.com/xampprocky/tokei/issues/381)**
    - Bump dirs from 2.0.1 to 2.0.2 ([`1581643`](https://github.com/xampprocky/tokei/commit/1581643cb36d7ab1d95dc770692b7c484745f09e))
 * **[#382](https://github.com/xampprocky/tokei/issues/382)**
    - Bump grep-searcher from 0.1.5 to 0.1.6 ([`54bd55d`](https://github.com/xampprocky/tokei/commit/54bd55d8e44eb3e47ee8a242b8d67d52d494f21c))
 * **[#384](https://github.com/xampprocky/tokei/issues/384)**
    - Add Emojicode support ([`7cc6e01`](https://github.com/xampprocky/tokei/commit/7cc6e01f59e701f5ce191a64134e602e7889d98e))
 * **[#385](https://github.com/xampprocky/tokei/issues/385)**
    - Added support for HolyC (language of TempleOS) ([`591cb20`](https://github.com/xampprocky/tokei/commit/591cb2039325c700bfa8c4579bf9b62bf43a64ab))
 * **[#387](https://github.com/xampprocky/tokei/issues/387)**
    - Bump env_logger from 0.7.0 to 0.7.1 ([`0b432aa`](https://github.com/xampprocky/tokei/commit/0b432aab6d9bc93b99e5ab5bcf19036941900299))
 * **[#389](https://github.com/xampprocky/tokei/issues/389)**
    - Added HolyC to list of supported languages ([`941e426`](https://github.com/xampprocky/tokei/commit/941e4269c1f5b93d8cd4f2aa980b2734fa59caa3))
 * **[#392](https://github.com/xampprocky/tokei/issues/392)**
    - Bump serde_derive from 1.0.101 to 1.0.102 ([`d70b06a`](https://github.com/xampprocky/tokei/commit/d70b06a88fe607421b955f782ee733057ae51562))
 * **[#393](https://github.com/xampprocky/tokei/issues/393)**
    - Bump serde from 1.0.101 to 1.0.102 ([`04a460e`](https://github.com/xampprocky/tokei/commit/04a460ee721e86877e816274ffaac06cde002aee))
 * **[#395](https://github.com/xampprocky/tokei/issues/395)**
    - Added Rusty Object Notation ([`b1d3688`](https://github.com/xampprocky/tokei/commit/b1d3688cb728ffde1d9fd830e8f66fb9d2f64625))
 * **[#397](https://github.com/xampprocky/tokei/issues/397)**
    - Bump toml from 0.5.3 to 0.5.4 ([`60d2812`](https://github.com/xampprocky/tokei/commit/60d281273ad1537e34f66800bc820a59fb2aa4fb))
 * **[#399](https://github.com/xampprocky/tokei/issues/399)**
    - Format code using 'cargo fmt' ([`fcd555a`](https://github.com/xampprocky/tokei/commit/fcd555ad9f1d80078c4b2ed1f6fdabff29d82bab))
 * **[#401](https://github.com/xampprocky/tokei/issues/401)**
    - Explain main input argument and --exclude a bit better ([`0447f1a`](https://github.com/xampprocky/tokei/commit/0447f1a433e535136eedb96778b8853c040dc53d))
 * **[#402](https://github.com/xampprocky/tokei/issues/402)**
    - Add support for Odin ([`1eedbdc`](https://github.com/xampprocky/tokei/commit/1eedbdcab496f4e1280cfdc74760ccafdb79b15f))
 * **[#403](https://github.com/xampprocky/tokei/issues/403)**
    - Bump toml from 0.5.4 to 0.5.5 ([`db4765f`](https://github.com/xampprocky/tokei/commit/db4765f639bd1bb53b429f7438f7332145198198))
 * **[#404](https://github.com/xampprocky/tokei/issues/404)**
    - Added opensuse to install list ([`c363019`](https://github.com/xampprocky/tokei/commit/c36301924e9118cd8b9061cb6feeae1d70007def))
 * **Uncategorized**
    - Bump to 10.1.0 and update dependencies ([`5e4053e`](https://github.com/xampprocky/tokei/commit/5e4053ebed927b11fabd925b5dd33f4ed9c052e2))
    - Switch to crossbeam channels ([`49ae79c`](https://github.com/xampprocky/tokei/commit/49ae79c8ac2841e5507aa34514089c4e2d87a2bb))
    - fix string quote ([`e3f8bf9`](https://github.com/xampprocky/tokei/commit/e3f8bf996d8ec33152426ca4aeff47c639c1534a))
    - Update rust.yml ([`ebcfb1d`](https://github.com/xampprocky/tokei/commit/ebcfb1d8555de6683f469c53ef8cbed50541c723))
    - Update dependencies ([`53d8799`](https://github.com/xampprocky/tokei/commit/53d8799c2b3128fe04407cffb3f2d5f3eb284727))
    - Update .travis.yml ([`6750519`](https://github.com/xampprocky/tokei/commit/6750519c00f06add840459ce47f58c771b819ede))
    - Update .travis.yml ([`cd59250`](https://github.com/xampprocky/tokei/commit/cd59250a9e1c653bc82ab125b11f4f0522fa4dcf))
    - Removed Dockerfile ([`b028933`](https://github.com/xampprocky/tokei/commit/b028933f50b15bec9537e7d96f89418bee358a6b))
    - Update README.md ([`024938c`](https://github.com/xampprocky/tokei/commit/024938c88dc8ce94aab808d76828caf7cc356f78))
    - Update appveyor.yml ([`a35d457`](https://github.com/xampprocky/tokei/commit/a35d45735ddf6f3921a8f275e227da87c6feb566))
    - Update README.md ([`46d08e6`](https://github.com/xampprocky/tokei/commit/46d08e6c0d3e95e31e686e17f07bb396c1a253d7))
    - Update README.md ([`1ff6c06`](https://github.com/xampprocky/tokei/commit/1ff6c06a4ff039656587c645258bc198d565f63d))
    - Update README.md ([`38d6fdc`](https://github.com/xampprocky/tokei/commit/38d6fdca600b649ab38f7522d451f0fdab833691))
    - updated lockfile ([`afc4441`](https://github.com/xampprocky/tokei/commit/afc44415a202504ac3f6990c74c8c1619fddb33a))
</details>

# 10.0.1 (2019-07-04)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 23 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#337](https://github.com/xampprocky/tokei/issues/337), [#340](https://github.com/xampprocky/tokei/issues/340), [#342](https://github.com/xampprocky/tokei/issues/342), [#344](https://github.com/xampprocky/tokei/issues/344)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#337](https://github.com/xampprocky/tokei/issues/337)**
    - Fix formatting of the added language list ([`ea4d7f1`](https://github.com/xampprocky/tokei/commit/ea4d7f12a6507e06d412b70f4ca1d9754e2a0681))
 * **[#340](https://github.com/xampprocky/tokei/issues/340)**
    - Fix spelling for Xcode and Objective-C(++) ([`4951c5d`](https://github.com/xampprocky/tokei/commit/4951c5dec895ee939ca972aaee857360600cafeb))
 * **[#342](https://github.com/xampprocky/tokei/issues/342)**
    - Add .cjsx as a CoffeeScript extension ([`354948d`](https://github.com/xampprocky/tokei/commit/354948df02ffa2637b4141bac9de7c54f2d6cb51))
 * **[#344](https://github.com/xampprocky/tokei/issues/344)**
    - Added config so "/usr/bin/env ruby" works ([`afcaf04`](https://github.com/xampprocky/tokei/commit/afcaf0418fc75e28eeea16fea8211373712a56f7))
 * **Uncategorized**
    - Version bump ([`adf13d6`](https://github.com/xampprocky/tokei/commit/adf13d6b5d661479e270e0c3cd4d3d710316af05))
    - Updated documentation ([`cfbdcce`](https://github.com/xampprocky/tokei/commit/cfbdcce2c742a1eed74502b191dfb6d000fa5f68))
    - Updated gitignore ([`c0af17a`](https://github.com/xampprocky/tokei/commit/c0af17a815d4c39ad1cc1d3cac59e6b9f193f741))
    - Update README.md ([`9051498`](https://github.com/xampprocky/tokei/commit/9051498c792fe45444344619a6531bcb45e409b6))
</details>

# 10.0.0 (2019-06-10)

- Fixed minor parsing bugs.
- Width is now limited to 80 unless you use the `--files` flag.
- Added the `mjs` extension to JavaScript.
- Added the `tpp` extension to C++.
- You can now disable Tokei's git ignore detection, similar to ripgrep. See
  `--help` for options.
- You can now add a `.tokeignore` file to your project to specify file paths
  for tokei to always ignore. This file uses the same syntax as `.gitignore`.
- Improved Pascal representation

**Added languages**

- @hobofan solidity
- @stefanmaric GraphQL
- @jhpratt PostCSS
- @evitalis RPM
- @alexmaco Pony
- @yjhmelody WASM, LLVM, Pest
- @XAMPPRocky ASN.1

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 71 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 7 unique issues were worked on: [#319](https://github.com/xampprocky/tokei/issues/319), [#321](https://github.com/xampprocky/tokei/issues/321), [#323](https://github.com/xampprocky/tokei/issues/323), [#325](https://github.com/xampprocky/tokei/issues/325), [#327](https://github.com/xampprocky/tokei/issues/327), [#328](https://github.com/xampprocky/tokei/issues/328), [#336](https://github.com/xampprocky/tokei/issues/336)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#319](https://github.com/xampprocky/tokei/issues/319)**
    - languages.json tweak for improved Pascal accuracy ([`ca7195b`](https://github.com/xampprocky/tokei/commit/ca7195be2855f506dad49af41ebf3c5c34b47e89))
 * **[#321](https://github.com/xampprocky/tokei/issues/321)**
    - add .tpp extension to C++ ([`abd244d`](https://github.com/xampprocky/tokei/commit/abd244d511cc6a8885baf92ed262559de06707fb))
 * **[#323](https://github.com/xampprocky/tokei/issues/323)**
    - Add support for wasm, llvm ,postCSS, pest ([`b199b5b`](https://github.com/xampprocky/tokei/commit/b199b5b398937b2ff99d7fe0567b3cdc9e1cfaee))
 * **[#325](https://github.com/xampprocky/tokei/issues/325)**
    - Add PostCSS to languages ([`8b16ce6`](https://github.com/xampprocky/tokei/commit/8b16ce6cc677630ed70e2642fdafcfd148960cf5))
 * **[#327](https://github.com/xampprocky/tokei/issues/327)**
    - Add RPM specfile language ([`b55653b`](https://github.com/xampprocky/tokei/commit/b55653ba00cf452d72065053277f7ad19d98e79c))
 * **[#328](https://github.com/xampprocky/tokei/issues/328)**
    - add Pony support with test ([`06270d4`](https://github.com/xampprocky/tokei/commit/06270d4254402d02c9153e6b907eb6b2e6b0e5c6))
 * **[#336](https://github.com/xampprocky/tokei/issues/336)**
    - Bump git2 to 0.9 ([`64b4936`](https://github.com/xampprocky/tokei/commit/64b493684c9082d9b00e4de154938549edc2be6e))
 * **Uncategorized**
    - Version 10 ([`71e0939`](https://github.com/xampprocky/tokei/commit/71e093918aae63e084acbfed9b188d8606a91a0f))
    - Updated documentation ([`3d68640`](https://github.com/xampprocky/tokei/commit/3d686401a333f86428645501e3fa3ee0e2a36ad6))
    - Fixed parsing bug ([`b704501`](https://github.com/xampprocky/tokei/commit/b70450171d19dd67085ee872c0d852e8b8da7816))
    - Updated import styling ([`3e7e9f2`](https://github.com/xampprocky/tokei/commit/3e7e9f24a9ca324021bc4ebd387dae1ae01055d6))
    - Remove whitespace ([`49a7e2d`](https://github.com/xampprocky/tokei/commit/49a7e2df760a208b3e82e52475f8d86986955728))
    - Fixed clippy lints ([`2c41fdb`](https://github.com/xampprocky/tokei/commit/2c41fdb182b7ba1bcc7c9d1cfc0de569538e7f0d))
    - Added non UTF8 parsing ([`2c97dc0`](https://github.com/xampprocky/tokei/commit/2c97dc09aa77b460adb4708bcc96007cea7b44ec))
    - Added back changes lost from force push ([`86c59b2`](https://github.com/xampprocky/tokei/commit/86c59b2b0a53a099e281df9c84fe55cafc6fa49f))
    - Added ASN.1 Support ([`d9bfe81`](https://github.com/xampprocky/tokei/commit/d9bfe81174802471ca8d294cb97234be9d95194e))
    - fixed postcss file ([`e10c19a`](https://github.com/xampprocky/tokei/commit/e10c19a53f6ebe2fc04a0a483dfd486089341b4b))
    - Fixed duplicate definition ([`7f14258`](https://github.com/xampprocky/tokei/commit/7f14258941a0688c031e86a128c1a82242028271))
    - Added .tokeignore ([`827e85d`](https://github.com/xampprocky/tokei/commit/827e85d82ab5a87931ce8b3d25447b86b82c7d7e))
</details>

# 9.1.1 (2019-03-30)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release over the course of 75 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on: [#289](https://github.com/xampprocky/tokei/issues/289), [#294](https://github.com/xampprocky/tokei/issues/294), [#296](https://github.com/xampprocky/tokei/issues/296), [#297](https://github.com/xampprocky/tokei/issues/297), [#303](https://github.com/xampprocky/tokei/issues/303), [#306](https://github.com/xampprocky/tokei/issues/306), [#310](https://github.com/xampprocky/tokei/issues/310), [#312](https://github.com/xampprocky/tokei/issues/312)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#289](https://github.com/xampprocky/tokei/issues/289)**
    - limit width to 80 unless --files is specified ([`4fa3773`](https://github.com/xampprocky/tokei/commit/4fa3773e53a6ebe486fe6a0dcbdfc38c8dfb2198))
 * **[#294](https://github.com/xampprocky/tokei/issues/294)**
    - Recongize ES modules as JavaScript ([`28661e0`](https://github.com/xampprocky/tokei/commit/28661e07bd4b2a01dc4cbc5069672c613ace1d99))
 * **[#296](https://github.com/xampprocky/tokei/issues/296)**
    - Add Stratego language, also known as Stratego/XT, strategoxt.org ([`e4f218c`](https://github.com/xampprocky/tokei/commit/e4f218cbb253354006933d912c049b94b4764ecf))
 * **[#297](https://github.com/xampprocky/tokei/issues/297)**
    - add Docker container instructions ([`158c916`](https://github.com/xampprocky/tokei/commit/158c916ca73685d30b06cfd98eba47e80fa74374))
 * **[#303](https://github.com/xampprocky/tokei/issues/303)**
    - Add solidity support ([`41b120a`](https://github.com/xampprocky/tokei/commit/41b120ad2d5628e007666c791dcaf55594ae70d5))
 * **[#306](https://github.com/xampprocky/tokei/issues/306)**
    - Add GraphQL support ([`d46d511`](https://github.com/xampprocky/tokei/commit/d46d511cd8872c3be5d48083029d215090c88307))
 * **[#310](https://github.com/xampprocky/tokei/issues/310)**
    - add tokei_rs repository link ([`2a3161e`](https://github.com/xampprocky/tokei/commit/2a3161e1b6269c72e55ead49bbec4361b33dc463))
 * **[#312](https://github.com/xampprocky/tokei/issues/312)**
    - add tokei_rs repository link ([`2a3161e`](https://github.com/xampprocky/tokei/commit/2a3161e1b6269c72e55ead49bbec4361b33dc463))
 * **Uncategorized**
    - Version bump (9.1.1) ([`1a4f08b`](https://github.com/xampprocky/tokei/commit/1a4f08b62202262d1f43176c1de049abbef10e43))
    - Name change ([`b2f0437`](https://github.com/xampprocky/tokei/commit/b2f043706d811179da9554ce9cb5844f0c64044c))
    - Removed unneeded git features ([`e8ddc48`](https://github.com/xampprocky/tokei/commit/e8ddc4866b605744e5878b02dbd1cc45ef331fcc))
    - Version 9.1.0 ([`24986e4`](https://github.com/xampprocky/tokei/commit/24986e48f5e217910f20f376be2da0a9abe6b4f4))
    - Added custom ignore file ([`0b81f26`](https://github.com/xampprocky/tokei/commit/0b81f266a0e36a81e4fff7362f456c90f95d6aca))
    - Update README.md ([`3c7ecb0`](https://github.com/xampprocky/tokei/commit/3c7ecb0dc5e6f3589e5b11d710b07253848c6dad))
    - Added arguments to allow for disabling ignoring ([`1035614`](https://github.com/xampprocky/tokei/commit/1035614b9a8cb729d1c7e94b86d23780a796aed4))
    - Updated Cargo.lock ([`fb15e3c`](https://github.com/xampprocky/tokei/commit/fb15e3ccaac340b9b1352501f5561017952b86d8))
    - Update README.md ([`676dd5b`](https://github.com/xampprocky/tokei/commit/676dd5b00b4b0a8b12192104f042a35f78c14f65))
    - Update CHANGELOG.md ([`a1faab9`](https://github.com/xampprocky/tokei/commit/a1faab95b0599002f78a5ba202583992931973b2))
</details>

# 9.0.0 (2019-01-13)

- Tokei now has config files. You can now specify some commonly used arguments
  in a `.tokeirc`/`tokei.toml`. Namely `columns` to set the default column
  output, `types` to filter your count to just a single set of languages, and
  `treat_doc_strings_as_comments` which is a new option that allows you to
  specify whether to treat doc strings such as `"""` in Python as comments
  or code.
  The config files can be specified in two places, the current directory tokei
  is running in and your [system configuration
  directory](//docs.rs/tokei/struct.Config.html#method.from_config_files). The
  priority of options is as follows
  `CLI > <current_directory> > <configuration_directory>`.
- Tokei is now available on [Conda](https://anaconda.org/conda-forge/tokei).
- [Tokei's README has been translated
  to chinese.](https://github.com/chinanf-boy/tokei-zh#tokei-)
- `LanguageType` now implements `Hash`.
- Tokei now batches it's console output, this should result in a small
  performance boost.
- There is now a `--columns` argument for manually setting tokei's output width.
- The `--sort` argument is now case-insensitive.
- Tokei will now mark languages who's files failed to parse correctly as
  potentially inaccurate.
- Due to a bug in trust-ci `x86_64-unknown-netbsd` versions are will not be
  available in GitHub releases. (You will still be able to install from source.)
- Due to toml-rs's lacking enum support the TOML output option has
  been disabled.

**Added languages**

- @t-richards Liquid
- @diaphore Added the `.glsl` extension to GLSL.
- @ahmedelgabri Twig
- @pmoura Logtalk
- @alekratz Perl, Not Quite Perl
- @XAMPPRocky Automake, .NET Resource, HLSL, INI, Unreal Plugin,
  Unreal Project, Unreal Shader, Unreal Shader Header, Unreal Markdown,
  Visual Basic, Visual Studio Solution, Visual Studio Project, Xcode Config,
- @TheMrNomis SWIG
- @xnorme Added the `.vhdl` extension to VHDL

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 29 commits contributed to the release over the course of 137 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 11 unique issues were worked on: [#252](https://github.com/xampprocky/tokei/issues/252), [#260](https://github.com/xampprocky/tokei/issues/260), [#264](https://github.com/xampprocky/tokei/issues/264), [#265](https://github.com/xampprocky/tokei/issues/265), [#270](https://github.com/xampprocky/tokei/issues/270), [#272](https://github.com/xampprocky/tokei/issues/272), [#273](https://github.com/xampprocky/tokei/issues/273), [#274](https://github.com/xampprocky/tokei/issues/274), [#281](https://github.com/xampprocky/tokei/issues/281), [#285](https://github.com/xampprocky/tokei/issues/285), [#287](https://github.com/xampprocky/tokei/issues/287)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#252](https://github.com/xampprocky/tokei/issues/252)**
    - Fix URL scheme documentation. ([`696994f`](https://github.com/xampprocky/tokei/commit/696994f0a342ec0f2bb0f50f2424bf2b1c245d1a))
 * **[#260](https://github.com/xampprocky/tokei/issues/260)**
    - Liquid template language support ([`b556874`](https://github.com/xampprocky/tokei/commit/b55687462b8e2ed8eef3c32752bf719c05ed6e99))
 * **[#264](https://github.com/xampprocky/tokei/issues/264)**
    - Add ".glsl" suffix to GLSL ([`0dd7ac2`](https://github.com/xampprocky/tokei/commit/0dd7ac28e661faeccb4c3189c68014636e0662eb))
 * **[#265](https://github.com/xampprocky/tokei/issues/265)**
    - Add twig support ([`6f84d85`](https://github.com/xampprocky/tokei/commit/6f84d85ae86998e5d4378e265d11b2fc7a35f4df))
 * **[#270](https://github.com/xampprocky/tokei/issues/270)**
    - Show Inaccuracy Warning on permission error ([`51ecef4`](https://github.com/xampprocky/tokei/commit/51ecef450caad1f0c9c5ec473057b0b7f277e4db))
 * **[#272](https://github.com/xampprocky/tokei/issues/272)**
    - Let --sort argument be case insensitive. ([`511b2fd`](https://github.com/xampprocky/tokei/commit/511b2fd3b78675ae4515213fcd3cecdea8b86a5d))
 * **[#273](https://github.com/xampprocky/tokei/issues/273)**
    - Added support for Logtalk ([`9d8e28a`](https://github.com/xampprocky/tokei/commit/9d8e28af5191171dac909f2727bb10fda7fab764))
 * **[#274](https://github.com/xampprocky/tokei/issues/274)**
    - Add language rules for Perl6 and its cousin Not Quite Perl ([`21f6fc0`](https://github.com/xampprocky/tokei/commit/21f6fc0c1108b298ac6e40ff6a2318f2876781e9))
 * **[#281](https://github.com/xampprocky/tokei/issues/281)**
    - clarified cloc's speed vs cloc's time ([`f1ff76a`](https://github.com/xampprocky/tokei/commit/f1ff76a918ee2c7fc4f311efa74d6e8648a08051))
 * **[#285](https://github.com/xampprocky/tokei/issues/285)**
    - Add SWIG support ([`62e000d`](https://github.com/xampprocky/tokei/commit/62e000dec8b4a036b717586aaa81baaf8e8fa82a))
 * **[#287](https://github.com/xampprocky/tokei/issues/287)**
    - Adding "vhdl" file extension for VHDL files ([`d455a22`](https://github.com/xampprocky/tokei/commit/d455a22ac022940da0b8992123582d69c1c619b4))
 * **Uncategorized**
    - Bump version ([`97312d2`](https://github.com/xampprocky/tokei/commit/97312d2ad4b6e8beee2480996dfa9c2f7e2e364f))
    - Removed copyright headers ([`5a4767e`](https://github.com/xampprocky/tokei/commit/5a4767ee2a899f35b07f99d2cb7e0fff00a4abc2))
    - Cleaned up a small section, and updated dependencies ([`0135ac2`](https://github.com/xampprocky/tokei/commit/0135ac23901c4b5ff6d67dbcb492dd2567fdea1b))
    - Move to 2018 edition ([`6687d07`](https://github.com/xampprocky/tokei/commit/6687d0772945d79d11a7277bb88febdf64d524a5))
    - Implemented config files ([`755070e`](https://github.com/xampprocky/tokei/commit/755070ee841904b3b8c8ac3e3cd648a575307db7))
    - Added error message when top level path not found. fixes #161 ([`7c8a6b9`](https://github.com/xampprocky/tokei/commit/7c8a6b9b622a14c6832feadcca52ac4e2ecabaf0))
    - Remove toml support ([`6dd6b74`](https://github.com/xampprocky/tokei/commit/6dd6b744ec186ed27b03fe69603fb07be07bf8ef))
    - Refactored CLI code and added columns option ([`e8c3e24`](https://github.com/xampprocky/tokei/commit/e8c3e24b612f30901b33bf74352b8dec0eaff51b))
    - Reverted API change ([`a48f081`](https://github.com/xampprocky/tokei/commit/a48f0818f5dbf279746e24184221eb1486e1ac42))
    - fixed build issue ([`47a334e`](https://github.com/xampprocky/tokei/commit/47a334e8a8a0279ce8ac5fe663767bf1cbc287a0))
    - Refactored counting code, added Unreal related languages ([`d4998ec`](https://github.com/xampprocky/tokei/commit/d4998ecee3ebe2084ddfe81d7573950eac48ddec))
    - Refactor, and use buffered io for output ([`e589308`](https://github.com/xampprocky/tokei/commit/e5893083c27d760114f409083a323778961b3fe8))
    - Use BufWriter, and refactor printing code ([`eea0fe0`](https://github.com/xampprocky/tokei/commit/eea0fe08b6dc8baed1d57b80288454edd3be7b37))
    - LanguageType impls Hash, fixes #259 ([`2430dc0`](https://github.com/xampprocky/tokei/commit/2430dc00e02e97465bf89507c0bc1e9cbaee55fd))
    - Comment out x86_64-unknown-netbsd ([`c009c56`](https://github.com/xampprocky/tokei/commit/c009c5645aed854b6f4bcfd5b5fa39d017e636d1))
    - Added Chinese translation ([`321ed22`](https://github.com/xampprocky/tokei/commit/321ed22b3d6e0d05044a426ed14019d4171e1e73))
    - Add conda package. ([`a9a185e`](https://github.com/xampprocky/tokei/commit/a9a185e6e9c7d6c472736aadaf8ecc69d202b3c9))
    - Updated Cargo.lock ([`296eeac`](https://github.com/xampprocky/tokei/commit/296eeacb1b888d85248a1e3b2e30301c6f4d2a32))
</details>

# 8.0.1 (2018-08-28)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Version bump ([`e6f5ea0`](https://github.com/xampprocky/tokei/commit/e6f5ea0a0177ca73f32a8ba59ff79ca79f8d471f))
    - Changed back to PathBuf, and corrected documentation ([`ec5db3c`](https://github.com/xampprocky/tokei/commit/ec5db3c2016596dfd250fd72d4d0538944b46341))
    - Added documentation to new methods and added parse_from_str ([`bf53af8`](https://github.com/xampprocky/tokei/commit/bf53af83ddd1fef7cd6c5d9951448c59f1c50034))
    - Removed extra stat syscall ([`cd26407`](https://github.com/xampprocky/tokei/commit/cd26407645562dc12b7e468b42c7a138d64b61ab))
    - Update CHANGELOG.md ([`d16671b`](https://github.com/xampprocky/tokei/commit/d16671b28b3b049d2e126d6dd977267ec2e3de9c))
</details>

# 8.0.0 (2018-08-25)

- A language's comments, and quotes are now available through the `LanguageType`
  enum.
- You can filter by language using the `-t/--type` option. e.g. `tokei -t "Rust,C"`
  will print only Rust and C files.
- Tokei now understands terminal width and will expand to fit it. (Thanks
  to @Veykril)
- Added [comparison](./COMPARISON.md) document to compare Tokei to other
  code counters.
- Updated dependencies

**Added languages**

- @BrandonBoone VB6, VBScript, XSLT
- @ialpert BrightScript
- @PJB3005 Dream Maker
- @schmee edn

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 78 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on: [#232](https://github.com/xampprocky/tokei/issues/232), [#233](https://github.com/xampprocky/tokei/issues/233), [#234](https://github.com/xampprocky/tokei/issues/234), [#235](https://github.com/xampprocky/tokei/issues/235), [#236](https://github.com/xampprocky/tokei/issues/236), [#239](https://github.com/xampprocky/tokei/issues/239), [#240](https://github.com/xampprocky/tokei/issues/240), [#246](https://github.com/xampprocky/tokei/issues/246)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#232](https://github.com/xampprocky/tokei/issues/232)**
    - Added VB6 Support ([`a1ed92d`](https://github.com/xampprocky/tokei/commit/a1ed92df2800694ea616017f3f3aeb9149659b83))
 * **[#233](https://github.com/xampprocky/tokei/issues/233)**
    - Add BrightScript language and test ([`6ec6a36`](https://github.com/xampprocky/tokei/commit/6ec6a363a735a5e2322bbf30862dc942da724dd5))
 * **[#234](https://github.com/xampprocky/tokei/issues/234)**
    - Added VBScript Support ([`1481452`](https://github.com/xampprocky/tokei/commit/14814526becfc8494eb30c0633bdb534ded0b899))
 * **[#235](https://github.com/xampprocky/tokei/issues/235)**
    - Added support for XSLT ([`e0ef907`](https://github.com/xampprocky/tokei/commit/e0ef907b0dd3441a67348482cd9c494a7d327128))
 * **[#236](https://github.com/xampprocky/tokei/issues/236)**
    - Adds Dream Maker support. ([`20414bc`](https://github.com/xampprocky/tokei/commit/20414bccb5f2f8d7bcbcbd07411746289d04ae5b))
 * **[#239](https://github.com/xampprocky/tokei/issues/239)**
    - Add support for edn ([`82df93f`](https://github.com/xampprocky/tokei/commit/82df93f86676618703361872a030a7ffc4866126))
 * **[#240](https://github.com/xampprocky/tokei/issues/240)**
    - update handlebars to 1 ([`6f42d4c`](https://github.com/xampprocky/tokei/commit/6f42d4c30e53d62f567a74f3185c8ff68e89370b))
 * **[#246](https://github.com/xampprocky/tokei/issues/246)**
    - Acknowledge terminal width for output on Unix-systems ([`0e09919`](https://github.com/xampprocky/tokei/commit/0e09919606b33ddb4451b16607c47743014551d2))
 * **Uncategorized**
    - Updated v8.0.0 and added comparison document ([`ee5df4b`](https://github.com/xampprocky/tokei/commit/ee5df4b4d3ee13ef26a3017941d809c29a965432))
    - Added additional tools to the full run of a benchmark ([`2eea66b`](https://github.com/xampprocky/tokei/commit/2eea66bc4476139cf9ba9b76b024b1efd7aedc1e))
    - Updated .gitignore ([`7e9be5e`](https://github.com/xampprocky/tokei/commit/7e9be5e9c417af8e2a2678a0447e667efafe1cc7))
    - Removed additional lookups for language comments ([`b75b377`](https://github.com/xampprocky/tokei/commit/b75b377abba2b00ef8554a6411cdc11b82921a2e))
    - Moved language data to LanguageType, added language filters, and added benchmark script. ([`b8f90c2`](https://github.com/xampprocky/tokei/commit/b8f90c2e6de62885cadb9875806003854d051010))
    - Update README.md ([`e7b9477`](https://github.com/xampprocky/tokei/commit/e7b9477da2d8b84eef3de818a8c33cb9c4a94f14))
    - Update README.md ([`5af5390`](https://github.com/xampprocky/tokei/commit/5af539010b4cd2234bbbdb82d192de7613dfcb86))
</details>

# 7.0.3 (2018-06-02)

Made various optimisations, up to 65% faster in some cases.

**Added languages**

- @DenialAdams Added Forsyth-Edwards-Notation (FEN)
- @DjebbZ Added ClojureC
- @grimm26 Added HCL/Terraform

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 37 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#222](https://github.com/xampprocky/tokei/issues/222), [#224](https://github.com/xampprocky/tokei/issues/224), [#228](https://github.com/xampprocky/tokei/issues/228)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com/xampprocky/tokei/issues/222)**
    - add HCL/Terraform support ([`26663ac`](https://github.com/xampprocky/tokei/commit/26663ac1eef0c0b336e513f57b9bcae46f6e74e7))
 * **[#224](https://github.com/xampprocky/tokei/issues/224)**
    - Add support for ClojureC + tests for all Clojure ([`349fe2b`](https://github.com/xampprocky/tokei/commit/349fe2b585ba949c94a45e38f65e880831687a59))
 * **[#228](https://github.com/xampprocky/tokei/issues/228)**
    - Add Forsyth-Edwards-Notation (FEN) support ([`fff02ca`](https://github.com/xampprocky/tokei/commit/fff02cac94c73fc6a40fd484687f0c015c8b36f0))
 * **Uncategorized**
    - Version 7.0.3 ([`a950ff1`](https://github.com/xampprocky/tokei/commit/a950ff128d5a435a8083b1c7577c0431f98360ca))
    - Added in early line check to speed up counting ([`272da6a`](https://github.com/xampprocky/tokei/commit/272da6aa1cd060e7ae7c80547b126bf77137a303))
    - Pre allocate stack to handle common case. ([`bb5cd95`](https://github.com/xampprocky/tokei/commit/bb5cd956f40b6395cc6c9ead839a3c53521210b2))
    - Refactored some of build.rs ([`51c04a0`](https://github.com/xampprocky/tokei/commit/51c04a030d444755ea69c1e30572898cd8bd6a0a))
    - Updated to use fs::read ([`9d3103e`](https://github.com/xampprocky/tokei/commit/9d3103e3bb83edf7243839c34bb526e9a074689b))
</details>

# 7.0.2 (2018-04-23)

- Updated dependencies.
- Changed how compilied serialization formats are handled.
- Fixed minor parser inaccuracies.
- Tokei should now recognise more python files from their shebang.

**Added languages**

- @ignatenko Added Meson
- @sprang Added Scheme
- @fengcms Added Vue
- @mark.knol Added Haxe
- @rleungx Added ABAP, COBOL, and Groovy
- @tiehuis Added Zig
- @murielsilveira Added Mint
- @notramo Added Elvish Shell and Kakoune
- @aatxe Added Racket
- @kamilchm Added ReasonML
- @cyplp Added XSL

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 39 commits contributed to the release over the course of 87 calendar days.
 - 5 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 24 unique issues were worked on: [#169](https://github.com/xampprocky/tokei/issues/169), [#179](https://github.com/xampprocky/tokei/issues/179), [#181](https://github.com/xampprocky/tokei/issues/181), [#187](https://github.com/xampprocky/tokei/issues/187), [#188](https://github.com/xampprocky/tokei/issues/188), [#190](https://github.com/xampprocky/tokei/issues/190), [#191](https://github.com/xampprocky/tokei/issues/191), [#196](https://github.com/xampprocky/tokei/issues/196), [#197](https://github.com/xampprocky/tokei/issues/197), [#199](https://github.com/xampprocky/tokei/issues/199), [#200](https://github.com/xampprocky/tokei/issues/200), [#201](https://github.com/xampprocky/tokei/issues/201), [#202](https://github.com/xampprocky/tokei/issues/202), [#204](https://github.com/xampprocky/tokei/issues/204), [#205](https://github.com/xampprocky/tokei/issues/205), [#206](https://github.com/xampprocky/tokei/issues/206), [#207](https://github.com/xampprocky/tokei/issues/207), [#211](https://github.com/xampprocky/tokei/issues/211), [#212](https://github.com/xampprocky/tokei/issues/212), [#213](https://github.com/xampprocky/tokei/issues/213), [#214](https://github.com/xampprocky/tokei/issues/214), [#216](https://github.com/xampprocky/tokei/issues/216), [#217](https://github.com/xampprocky/tokei/issues/217), [#219](https://github.com/xampprocky/tokei/issues/219)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#169](https://github.com/xampprocky/tokei/issues/169)**
    - add support for Meson ([`01791bd`](https://github.com/xampprocky/tokei/commit/01791bd7933df580d60b338551e3416a19b48c86))
 * **[#179](https://github.com/xampprocky/tokei/issues/179)**
    - update handlebars to 0.30 ([`39cd9d6`](https://github.com/xampprocky/tokei/commit/39cd9d64c8545ac353225d5905f65371aadb306d))
 * **[#181](https://github.com/xampprocky/tokei/issues/181)**
    - supported vue files ([`946af29`](https://github.com/xampprocky/tokei/commit/946af298f2cd300278e86e91a459e5536d577b48))
 * **[#187](https://github.com/xampprocky/tokei/issues/187)**
    - update handlebars to 0.31 ([`2d9d87d`](https://github.com/xampprocky/tokei/commit/2d9d87d87a865161f696d668759af39e407d37f7))
 * **[#188](https://github.com/xampprocky/tokei/issues/188)**
    - update ignore to 0.4 ([`4c274f4`](https://github.com/xampprocky/tokei/commit/4c274f4fc2c6ef931c31e706c47d841d27378661))
 * **[#190](https://github.com/xampprocky/tokei/issues/190)**
    - Update rayon to 1.0 and run a full cargo update ([`2a471bc`](https://github.com/xampprocky/tokei/commit/2a471bc5f33c4944b41b59af00ca0dd4ae97d995))
 * **[#191](https://github.com/xampprocky/tokei/issues/191)**
    - Refactor how compiled-in serialization formats are handled ([`76110af`](https://github.com/xampprocky/tokei/commit/76110afa63807473b2dcdb10f69ef5c48ae3f3b7))
 * **[#196](https://github.com/xampprocky/tokei/issues/196)**
    - Fix typo ([`8680d6c`](https://github.com/xampprocky/tokei/commit/8680d6c4e833bf590b9afeeab430601439866b5c))
 * **[#197](https://github.com/xampprocky/tokei/issues/197)**
    - Fix typos. Add Scheme. ([`fe4b8b3`](https://github.com/xampprocky/tokei/commit/fe4b8b3b378692455bb5144ebbeb450a75f92d0d))
 * **[#199](https://github.com/xampprocky/tokei/issues/199)**
    - Add Haxe ([`6f5e1fb`](https://github.com/xampprocky/tokei/commit/6f5e1fbd99d5e7758bbc3dc65681c5ad5e3b8573))
 * **[#200](https://github.com/xampprocky/tokei/issues/200)**
    - add support for more languages ([`ab812aa`](https://github.com/xampprocky/tokei/commit/ab812aa577d6e016167795ae8d0a1ae021cec894))
 * **[#201](https://github.com/xampprocky/tokei/issues/201)**
    - Add Mint language support ([`16d49fc`](https://github.com/xampprocky/tokei/commit/16d49fcd69ad47d4d7d6f6571fee3c995177e5d3))
 * **[#202](https://github.com/xampprocky/tokei/issues/202)**
    - Add support for zig ([`503f613`](https://github.com/xampprocky/tokei/commit/503f613eddd860a8f1fbc3f7fe46573dce8f5940))
 * **[#204](https://github.com/xampprocky/tokei/issues/204)**
    - Added Kakoune script. ([`616b0ae`](https://github.com/xampprocky/tokei/commit/616b0ae98414084595e65ab336eb3e941d6abb78))
 * **[#205](https://github.com/xampprocky/tokei/issues/205)**
    - Added Elvish shell. ([`dae2bcd`](https://github.com/xampprocky/tokei/commit/dae2bcdfedc49e07528632daf22e72b0a8e54469))
 * **[#206](https://github.com/xampprocky/tokei/issues/206)**
    - update handlebars to 0.32 ([`f51c02a`](https://github.com/xampprocky/tokei/commit/f51c02aadec869b7a8558cf5cb5a3052f6238e1a))
 * **[#207](https://github.com/xampprocky/tokei/issues/207)**
    - Update the help output in the README ([`4b89883`](https://github.com/xampprocky/tokei/commit/4b89883ac9ddd352f3dea70a9a0b4e7c7aaa29f6))
 * **[#211](https://github.com/xampprocky/tokei/issues/211)**
    - remove notice about "unstable" being required for Nix/NixOS ([`7c366ad`](https://github.com/xampprocky/tokei/commit/7c366ad040e02a82946fb9f979072e965bb94bb0))
 * **[#212](https://github.com/xampprocky/tokei/issues/212)**
    - Add Python identifiers for the env command ([`f585c4c`](https://github.com/xampprocky/tokei/commit/f585c4c87cfc5d270c9117bec58a52b798d72dae))
 * **[#213](https://github.com/xampprocky/tokei/issues/213)**
    - Add ReasonML ([`4d8949c`](https://github.com/xampprocky/tokei/commit/4d8949c3a600789850db4d76d9f3970eca68a3d0))
 * **[#214](https://github.com/xampprocky/tokei/issues/214)**
    - Add xsl support ([`eade044`](https://github.com/xampprocky/tokei/commit/eade044d5352c60841db2660bf7a42ce7d2d417e))
 * **[#216](https://github.com/xampprocky/tokei/issues/216)**
    - update README.md with how install output features ([`0f92a2f`](https://github.com/xampprocky/tokei/commit/0f92a2f4502de68d5d2165cd0c21f434f9ae54df))
 * **[#217](https://github.com/xampprocky/tokei/issues/217)**
    - Fix grammar and spelling ([`f5bd4f6`](https://github.com/xampprocky/tokei/commit/f5bd4f60d2267306985018b34f8e9cd34b0919ae))
 * **[#219](https://github.com/xampprocky/tokei/issues/219)**
    - Add Racket to the supported languages. ([`e6617c1`](https://github.com/xampprocky/tokei/commit/e6617c1d781260062cc80ef46201066640f760c3))
 * **Uncategorized**
    - Version 7.0.2 - Updated dependencies. - Changed how compilied serialization formats are handled. - Fixed minor parser inaccuracies. - Tokei should now recognise more python files from their shebang. ([`41c50a5`](https://github.com/xampprocky/tokei/commit/41c50a5cb53e5c7c4ad5042340d3f32317c0f6d1))
    - Cleaned up unused var and made crate version clearer. ([`804be2e`](https://github.com/xampprocky/tokei/commit/804be2e73bc124e25969873001f2a573ad75cfd2))
    - Update .travis.yml ([`0b1c3ec`](https://github.com/xampprocky/tokei/commit/0b1c3ecb5db6bd9d5d1f125aeb2bb34a78003ab9))
    - Updated CI ([`031c262`](https://github.com/xampprocky/tokei/commit/031c26225629ba65818e8bda80d143fb9a42f771))
    - Removed logo ([`5a32117`](https://github.com/xampprocky/tokei/commit/5a321170988fb84685abf4d993e0f6fcbe8ccc83))
    - Removed CONTRIBUTORS.md, use GitHub to see contributors ([`3665e15`](https://github.com/xampprocky/tokei/commit/3665e15bf9d378c72949603546a9bb2506cd0059))
    - Updated dependencies ([`7021e8f`](https://github.com/xampprocky/tokei/commit/7021e8fdf8b99572b8fb908c533410cfee4613e0))
    - Update README.md ([`3d2f86b`](https://github.com/xampprocky/tokei/commit/3d2f86bd330f7d8e903b8799b0c1b98b2bca51fa))
    - Update CONTRIBUTING.md ([`9201a59`](https://github.com/xampprocky/tokei/commit/9201a590dcf5ecdf37245caf722ba287395c263a))
    - Updated Cargo.lock ([`99a1146`](https://github.com/xampprocky/tokei/commit/99a11460b6e69188aaeea7518f2d0bbb16db336a))
    - Update README.md ([`f431f38`](https://github.com/xampprocky/tokei/commit/f431f38f1f8ae1789af7279b0d6be0b5edaf4cde))
    - Update README.md ([`648fbbf`](https://github.com/xampprocky/tokei/commit/648fbbf6e47446bae18749b7f049335fa5d871d1))
    - Updated CI to test and deploy featured versions ([`6c906f3`](https://github.com/xampprocky/tokei/commit/6c906f3ce00a90efd649cd51f1f9d09e693a2dc0))
    - Update README.md ([`4ed878c`](https://github.com/xampprocky/tokei/commit/4ed878cf27863ac64f7e99253369451e6b80aa53))
    - Fixed minor parser inaccuracies ([`553736f`](https://github.com/xampprocky/tokei/commit/553736fe8fe5128d8c0538c4d0e704efa350392b))
</details>

# 7.0.1 (2018-01-19)

- Updated dependencies

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#176](https://github.com/xampprocky/tokei/issues/176), [#178](https://github.com/xampprocky/tokei/issues/178)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#176](https://github.com/xampprocky/tokei/issues/176)**
    - deps/env_logger: 0.5.0-rc.2 → 0.5.0 ([`db4d85f`](https://github.com/xampprocky/tokei/commit/db4d85f6ca385062826bd6ec427e2f7a1270cee6))
 * **[#178](https://github.com/xampprocky/tokei/issues/178)**
    - fix build with features enabled ([`4715855`](https://github.com/xampprocky/tokei/commit/471585537cfa65a4b921abd814c7b8e07437d412))
 * **Uncategorized**
    - v7.0.1; Updated dependencies ([`672179e`](https://github.com/xampprocky/tokei/commit/672179ee204a5404d8fb314501503d850226902c))
</details>

# 7.0.0 (2018-01-15)

- Fixed parsing corner cases
- Changed storage of comments and quotes from `Vec` to static slices.
- Added tracing for debugging single files. Not recommended for use on
  multiple file
- Updated `log`

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 3 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#172](https://github.com/xampprocky/tokei/issues/172), [#174](https://github.com/xampprocky/tokei/issues/174)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#172](https://github.com/xampprocky/tokei/issues/172)**
    - update instructions for Fedora ([`af7b6d0`](https://github.com/xampprocky/tokei/commit/af7b6d025ec6edd06ebe6922808f14fda778f693))
 * **[#174](https://github.com/xampprocky/tokei/issues/174)**
    - Add FreeBSD install instructions ([`742a767`](https://github.com/xampprocky/tokei/commit/742a767e4153dc35f235fb7985c003f9653e7369))
 * **Uncategorized**
    - Version 7 ([`77380bc`](https://github.com/xampprocky/tokei/commit/77380bc4b2ece95baf6151ff7307c3228ecb7559))
    - Updated log, made changes recommended by clippy ([`8817762`](https://github.com/xampprocky/tokei/commit/8817762d7b8f5cd10886147b828f95d19dc2a80a))
</details>

# 6.1.3 (2018-01-06)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 23 commits contributed to the release over the course of 117 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 16 unique issues were worked on: [#140](https://github.com/xampprocky/tokei/issues/140), [#141](https://github.com/xampprocky/tokei/issues/141), [#142](https://github.com/xampprocky/tokei/issues/142), [#143](https://github.com/xampprocky/tokei/issues/143), [#144](https://github.com/xampprocky/tokei/issues/144), [#146](https://github.com/xampprocky/tokei/issues/146), [#147](https://github.com/xampprocky/tokei/issues/147), [#149](https://github.com/xampprocky/tokei/issues/149), [#151](https://github.com/xampprocky/tokei/issues/151), [#152](https://github.com/xampprocky/tokei/issues/152), [#153](https://github.com/xampprocky/tokei/issues/153), [#155](https://github.com/xampprocky/tokei/issues/155), [#158](https://github.com/xampprocky/tokei/issues/158), [#167](https://github.com/xampprocky/tokei/issues/167), [#170](https://github.com/xampprocky/tokei/issues/170), [#171](https://github.com/xampprocky/tokei/issues/171)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#140](https://github.com/xampprocky/tokei/issues/140)**
    - Fix #106: Add AutoHotKey filetype support ([`3dc17b3`](https://github.com/xampprocky/tokei/commit/3dc17b36ba7866fbf8dad8b92999bb5eb7f61d4e))
 * **[#141](https://github.com/xampprocky/tokei/issues/141)**
    - Updated README.md supported languages based on languages.json file ([`110b64c`](https://github.com/xampprocky/tokei/commit/110b64c7fc4431702f5fa414d6ba290dc30b4471))
 * **[#142](https://github.com/xampprocky/tokei/issues/142)**
    - Add QML support ([`6eee990`](https://github.com/xampprocky/tokei/commit/6eee9909565ddf783f82b840a103e0bf150d9d5a))
 * **[#143](https://github.com/xampprocky/tokei/issues/143)**
    - Added ClojureScript language ([`c327c3d`](https://github.com/xampprocky/tokei/commit/c327c3d19383cedb03644d3919d871eef6989bde))
 * **[#144](https://github.com/xampprocky/tokei/issues/144)**
    - Added Processing Language support ([`201a007`](https://github.com/xampprocky/tokei/commit/201a0070599cbd324447259a52d924cf7061a7c9))
 * **[#146](https://github.com/xampprocky/tokei/issues/146)**
    - Add the visual basic and msbuild language ([`86ffefd`](https://github.com/xampprocky/tokei/commit/86ffefd2596dbb471f1cc0a361dd789d400cdb8b))
 * **[#147](https://github.com/xampprocky/tokei/issues/147)**
    - Use eprintln! ([`7811295`](https://github.com/xampprocky/tokei/commit/78112951ae96c8221366d35c7a808fc552d69205))
 * **[#149](https://github.com/xampprocky/tokei/issues/149)**
    - Add SVG support ([`78f4a24`](https://github.com/xampprocky/tokei/commit/78f4a24e4cad18d80ddb46863d44c29d37f6279a))
 * **[#151](https://github.com/xampprocky/tokei/issues/151)**
    - Add support for .el, .ede, .org and .srt files ([`b5e415d`](https://github.com/xampprocky/tokei/commit/b5e415d6e2157aa5da670945af62e30f7eb715ab))
 * **[#152](https://github.com/xampprocky/tokei/issues/152)**
    - Add support for .psl, .e, .ckt, .sv/svh, .vg/vh, .irunargs/xrunargs ([`043a780`](https://github.com/xampprocky/tokei/commit/043a780cf2236682e49e5c40a5abdaf384c6363d))
 * **[#153](https://github.com/xampprocky/tokei/issues/153)**
    - Added another installation method ([`1a10689`](https://github.com/xampprocky/tokei/commit/1a10689c0567e4999ae55ebf4978762190e8f0fb))
 * **[#155](https://github.com/xampprocky/tokei/issues/155)**
    - Add Xtend support ([`5726c46`](https://github.com/xampprocky/tokei/commit/5726c46efd1034d76ef964594a6dcab2aa7f69e9))
 * **[#158](https://github.com/xampprocky/tokei/issues/158)**
    - bump ignore to 0.3, rayon to 0.9, handlebars to 0.29, hex to 0.3 ([`c3b6fd7`](https://github.com/xampprocky/tokei/commit/c3b6fd7b774f6eb54127d4b71677ac907bf77aa5))
 * **[#167](https://github.com/xampprocky/tokei/issues/167)**
    - Bump serde_cbor to 0.8 ([`46ee5ff`](https://github.com/xampprocky/tokei/commit/46ee5ff92f94e7cf7c855aa1629325a8eb28d990))
 * **[#170](https://github.com/xampprocky/tokei/issues/170)**
    - Fix broken copyright link in ToC ([`3549407`](https://github.com/xampprocky/tokei/commit/3549407ed65e60594d36cc7ddd6da93b02dacd16))
 * **[#171](https://github.com/xampprocky/tokei/issues/171)**
    - Update crossbeam to 0.2.12 ([`eb19471`](https://github.com/xampprocky/tokei/commit/eb19471328fe82fcf61dcebc8e049e4b61852f10))
 * **Uncategorized**
    - Updated benchmarks ([`1c43ff0`](https://github.com/xampprocky/tokei/commit/1c43ff066cd94b1d502a988fc0ccaf1432cd36f5))
    - Version 6.1.3 ([`34163e2`](https://github.com/xampprocky/tokei/commit/34163e234acfdc6d86cb3daae177e7ff5df2430e))
    - Updated log ([`14f0573`](https://github.com/xampprocky/tokei/commit/14f05734b46ce5c9dfa25cee75210ede2249943b))
    - Updated dependencies ([`477503e`](https://github.com/xampprocky/tokei/commit/477503e1aedbf58783550fee96bf487c03aa9337))
    - Added donate to libreapay ([`18381d4`](https://github.com/xampprocky/tokei/commit/18381d497533383a9d0b630f797ac0d8d19aa600))
    - Update README.md ([`0152772`](https://github.com/xampprocky/tokei/commit/0152772a9fcd6ad17627cfcd7de71952d53f09ff))
    - Trying to upgrade rustup ([`f6804f9`](https://github.com/xampprocky/tokei/commit/f6804f9670988da6801ab323fd363cdc61f49263))
</details>

# 6.1.2 (2017-09-03)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Version bump, Removed extra println ([`a69cf95`](https://github.com/xampprocky/tokei/commit/a69cf952255bdd12af941a3dadcfe37d63314dbc))
    - Removed extra debug println ([`9132112`](https://github.com/xampprocky/tokei/commit/9132112f1b045a281403c22efb001ffa4d141ff3))
</details>

# 6.1.1 (2017-08-31)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 28 commits contributed to the release over the course of 189 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 12 unique issues were worked on: [#108](https://github.com/xampprocky/tokei/issues/108), [#110](https://github.com/xampprocky/tokei/issues/110), [#114](https://github.com/xampprocky/tokei/issues/114), [#115](https://github.com/xampprocky/tokei/issues/115), [#122](https://github.com/xampprocky/tokei/issues/122), [#123](https://github.com/xampprocky/tokei/issues/123), [#125](https://github.com/xampprocky/tokei/issues/125), [#129](https://github.com/xampprocky/tokei/issues/129), [#131](https://github.com/xampprocky/tokei/issues/131), [#133](https://github.com/xampprocky/tokei/issues/133), [#135](https://github.com/xampprocky/tokei/issues/135), [#137](https://github.com/xampprocky/tokei/issues/137)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#108](https://github.com/xampprocky/tokei/issues/108)**
    - Updated list of supported languages ([`96e4dd7`](https://github.com/xampprocky/tokei/commit/96e4dd71fb560e87c71c304063d14ba50790be07))
 * **[#110](https://github.com/xampprocky/tokei/issues/110)**
    - Add support for the fish shell ([`fa58efe`](https://github.com/xampprocky/tokei/commit/fa58efefbb6d54d0f9d6b16858b1ab42e583baff))
 * **[#114](https://github.com/xampprocky/tokei/issues/114)**
    - Allow multiple --exclude options ([`fd37e94`](https://github.com/xampprocky/tokei/commit/fd37e940a0cc38c736f895f5a549844dbe701684))
 * **[#115](https://github.com/xampprocky/tokei/issues/115)**
    - Support "filenames" key, mk. 2 ([`7849edf`](https://github.com/xampprocky/tokei/commit/7849edfa34bbf5f62e30e9a0219b27881063769d))
 * **[#122](https://github.com/xampprocky/tokei/issues/122)**
    - Move CI to trust ([`d6910d1`](https://github.com/xampprocky/tokei/commit/d6910d1808106bae0289cff58845d3db282531a2))
 * **[#123](https://github.com/xampprocky/tokei/issues/123)**
    - Add Vala to `languages.json` ([`e4d06c9`](https://github.com/xampprocky/tokei/commit/e4d06c9af29109a51a20fb45ce3477241e64062b))
 * **[#125](https://github.com/xampprocky/tokei/issues/125)**
    - Add xaml support ([`65ef68d`](https://github.com/xampprocky/tokei/commit/65ef68d778b94f86f5e1c99ebfe19992975f2fbd))
 * **[#129](https://github.com/xampprocky/tokei/issues/129)**
    - Add tsx extension for typescript. ([`5770feb`](https://github.com/xampprocky/tokei/commit/5770feb2758d11b4b347350d604f8d952be2df45))
 * **[#131](https://github.com/xampprocky/tokei/issues/131)**
    - Add Ceylon. ([`dc34096`](https://github.com/xampprocky/tokei/commit/dc340960bbb3469da137f2b5120ba44b60516131))
 * **[#133](https://github.com/xampprocky/tokei/issues/133)**
    - added hamlet, cassius, lucius, cabal, nix, happy, alex, and madlang support ([`177d289`](https://github.com/xampprocky/tokei/commit/177d28988ab7512fbbc1f86cdad4928bac73cc38))
 * **[#135](https://github.com/xampprocky/tokei/issues/135)**
    - Added test for html ([`c6abf39`](https://github.com/xampprocky/tokei/commit/c6abf392563050e1d9293754c913c1594374ef04))
 * **[#137](https://github.com/xampprocky/tokei/issues/137)**
    - Added Crystal language to languages.json ([`c603040`](https://github.com/xampprocky/tokei/commit/c603040c956d1fbb11d8a35c8f750bdb1098cb73))
 * **Uncategorized**
    - Updated version ([`742e0e1`](https://github.com/xampprocky/tokei/commit/742e0e17544614ae7439051b73feafb89f1c54df))
    - Fixes #138, updated Cargo.lock ([`37e0824`](https://github.com/xampprocky/tokei/commit/37e0824cf7ca29729d30b8b66c368f36cd996afb))
    - Version 6.1.0 ([`e4db9b7`](https://github.com/xampprocky/tokei/commit/e4db9b76b0782f6e256625f761f242625a6b86e1))
    - Switched if cases for better performance ([`2f1b452`](https://github.com/xampprocky/tokei/commit/2f1b452c984ddaeadd01c749bcc5362960a3888f))
    - added test header ([`564a746`](https://github.com/xampprocky/tokei/commit/564a74608742d6f9e98efff810c0bb006e2c56b0))
    - Changed how tokei searches through files. ([`f53c796`](https://github.com/xampprocky/tokei/commit/f53c796f27d337f3160cb8e6357866d29c6ff5be))
    - Update README.md ([`6761589`](https://github.com/xampprocky/tokei/commit/6761589112f1cb77f8900128506666d8b2a791ea))
    - Fixed #134, updated dependencies, clarified errors Fixed an issue where if the ending comment was shorter than the starting multi line comment the endding comment would be ignored. ([`3b5aea4`](https://github.com/xampprocky/tokei/commit/3b5aea4b9111cf18a618ffb14655c1b0eb6b52f3))
    - Create CODE_OF_CONDUCT.md ([`8a2384d`](https://github.com/xampprocky/tokei/commit/8a2384dab2e223767b9c2964733165ced3d00083))
    - v6.0.2 ([`cd8bc04`](https://github.com/xampprocky/tokei/commit/cd8bc0434a7d34218811e41b4bb8c925cdd500af))
    - v6.0.2 ([`0969a95`](https://github.com/xampprocky/tokei/commit/0969a9553d127974249f333b20463cacb988a372))
    - Version bump ([`a55e3e5`](https://github.com/xampprocky/tokei/commit/a55e3e51a5be173763d771c1c8c9a98ed5da5e90))
    - Update README.md ([`b25faa0`](https://github.com/xampprocky/tokei/commit/b25faa0cdd6fbedd53a200d9b5f5115a61e5cd0b))
    - Added Module-Definition ([`732f8a0`](https://github.com/xampprocky/tokei/commit/732f8a09af9aef68cfa7e2b016a654e68560ce17))
    - updated cargo.lock and changed trim ([`2ac6693`](https://github.com/xampprocky/tokei/commit/2ac6693ca1423f9d584b498e59078fc119541ece))
    - Update README.md ([`42f8060`](https://github.com/xampprocky/tokei/commit/42f8060eea39958267f6bd5e68a5d278f6dea8b6))
</details>

# 6.1.0

- Fixed inaccuracies relating to the end comment being smaller than start
  comment.

**Added languages**

- @mattico Added Xaml
- @weakish Added Ceylon
- @theduke Added tsx extension to typescript
- @vmchale Added Hamlet, Cassius, Lucius, Cabal, Nix, Happy, Alex, and Madlang
- @notramo Added Crystal

# 6.0.2

- Now can recognise file languages based on their filename.

**Added Languages:**

- @kazimuth CMake, Dockerfile, Rakefile, Scons

# 6.0.1

- Multiple exclude flags now allowed.

**Added Languages:**

- @seiks Added Fish Shell
- @XAMPPRocky Added Module-Definition
- @tbu- Added Vala

# 6.0.0 (2017-02-18)

- Reworked internals
- Now uses serde*derive(\_and thusly requires rust v1.15*)
- Now has better file based testing

**Added languages:**

- @tuncer Added Ur/Web
- @svisser Added PureScript
- @tjodden Add some common extensions for HTML, C++ and Makefile
- @xd009642 Added VHDL

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 33 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 9 unique issues were worked on: [#101](https://github.com/xampprocky/tokei/issues/101), [#103](https://github.com/xampprocky/tokei/issues/103), [#104](https://github.com/xampprocky/tokei/issues/104), [#107](https://github.com/xampprocky/tokei/issues/107), [#94](https://github.com/xampprocky/tokei/issues/94), [#95](https://github.com/xampprocky/tokei/issues/95), [#96](https://github.com/xampprocky/tokei/issues/96), [#98](https://github.com/xampprocky/tokei/issues/98), [#99](https://github.com/xampprocky/tokei/issues/99)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#101](https://github.com/xampprocky/tokei/issues/101)**
    - Added support for PureScript ([`6026db5`](https://github.com/xampprocky/tokei/commit/6026db5ed53349522a42f3bde3c17f0f5d236035))
 * **[#103](https://github.com/xampprocky/tokei/issues/103)**
    - Update `env_logger` to 0.4 ([`0867d97`](https://github.com/xampprocky/tokei/commit/0867d9739843b5bf0b0b7e7ffd8b021bd279bef5))
 * **[#104](https://github.com/xampprocky/tokei/issues/104)**
    - Add some common extensions for HTML, C++ and Makefile ([`669464f`](https://github.com/xampprocky/tokei/commit/669464fadaeeecb96936845f42c7c9c2eac0c456))
 * **[#107](https://github.com/xampprocky/tokei/issues/107)**
    - Added tests for c++ source and header files and java files. ([`14870d7`](https://github.com/xampprocky/tokei/commit/14870d7625e992690dc1df0ed4fe8bafcf57e3df))
 * **[#94](https://github.com/xampprocky/tokei/issues/94)**
    - Extend Standard ML name ([`ec28dd2`](https://github.com/xampprocky/tokei/commit/ec28dd208db4c7f68a558af1914e95eb9eda75ab))
 * **[#95](https://github.com/xampprocky/tokei/issues/95)**
    - Add Ur/Web support ([`825f9be`](https://github.com/xampprocky/tokei/commit/825f9becdd45d62e88b0b34081a9fefdf24b7fa1))
 * **[#96](https://github.com/xampprocky/tokei/issues/96)**
    - fixed `--languages` option always conflicting with input ([`b20e410`](https://github.com/xampprocky/tokei/commit/b20e410a72ccb20d7e82e70e725b40c93c3c8f2e))
 * **[#98](https://github.com/xampprocky/tokei/issues/98)**
    - Ur/Web: fix reference in Ur/Web project file ([`f19bcd9`](https://github.com/xampprocky/tokei/commit/f19bcd994a3e048bd28701034be84abc3f7c1fa3))
 * **[#99](https://github.com/xampprocky/tokei/issues/99)**
    - Improve and extend test data ([`6590512`](https://github.com/xampprocky/tokei/commit/65905122b88b4b9be319c4c182dcd26da9503446))
 * **Uncategorized**
    - Update CHANGELOG.md ([`6cf2d65`](https://github.com/xampprocky/tokei/commit/6cf2d65725236e2d73a094cc4e2d024282118cfa))
    - Version bump ([`2b69096`](https://github.com/xampprocky/tokei/commit/2b69096243825319399ae1b2c5b45d3181ab3dc4))
    - Update README.md ([`4ad5010`](https://github.com/xampprocky/tokei/commit/4ad50103285817cfd6cfe2c93723ada5f90a1735))
    - Updated to use serde_derive, better tests, now uses macro for cli, updated dependencies ([`d3dc298`](https://github.com/xampprocky/tokei/commit/d3dc298708cc15ed0ce755d60319297f4ed26aba))
    - Updated metadata ([`7e2289f`](https://github.com/xampprocky/tokei/commit/7e2289f089af83b42347261eb5f4d222d71898b3))
    - Added some package info ([`bc560ad`](https://github.com/xampprocky/tokei/commit/bc560ad268f9b47464565c209a896250bcb15676))
</details>

# 5.0.6 (2017-01-15)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - v5.0.6: Updated dependencies, fixed exclude bug, and now defaults to running the current directory over failing. ([`b7a892f`](https://github.com/xampprocky/tokei/commit/b7a892f4753de6816de88e4df231e3ec7502269d))
</details>

# 5.0.5 (2017-01-03)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - version bump ([`721f648`](https://github.com/xampprocky/tokei/commit/721f6480f5e5719ceaad796232c6e3bb5d9ceeb3))
</details>

# 5.0.4 (2017-01-03)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - fixed json test ([`f6871e1`](https://github.com/xampprocky/tokei/commit/f6871e101c1e57b6133b1464a4f9332443d11105))
</details>

# 5.0.3 (2017-01-03)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 23 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#87](https://github.com/xampprocky/tokei/issues/87), [#88](https://github.com/xampprocky/tokei/issues/88), [#90](https://github.com/xampprocky/tokei/issues/90), [#91](https://github.com/xampprocky/tokei/issues/91)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#87](https://github.com/xampprocky/tokei/issues/87)**
    - Add F* support ([`a06c5e1`](https://github.com/xampprocky/tokei/commit/a06c5e10bf49273723b7530df390e249aad97a48))
 * **[#88](https://github.com/xampprocky/tokei/issues/88)**
    - Add Cogent support ([`9a31c1c`](https://github.com/xampprocky/tokei/commit/9a31c1cb849745d3aae99298f1984cf58f1187c3))
 * **[#90](https://github.com/xampprocky/tokei/issues/90)**
    - Add F# support ([`f3f21ae`](https://github.com/xampprocky/tokei/commit/f3f21ae6f1b81bc43ac6750a28df89cbfb0b6199))
 * **[#91](https://github.com/xampprocky/tokei/issues/91)**
    - Add Agda Support ([`5e11c48`](https://github.com/xampprocky/tokei/commit/5e11c4852fe4aa086b0e4fe5885822fbe57ba928))
 * **Uncategorized**
    - fixed features ([`229f2a3`](https://github.com/xampprocky/tokei/commit/229f2a34af74d6b5158156bb9cbf274b56c9a421))
    - version 5: optimised stats, language generation ([`c08f113`](https://github.com/xampprocky/tokei/commit/c08f113c2d6cd0036ec1373a64d0ec62d0dfa7c0))
    - Update README.md ([`37ed304`](https://github.com/xampprocky/tokei/commit/37ed304b0850544ed6324f6bfee61c8d8a442f5e))
    - Updated README, and updated some metadata ([`e48dfe8`](https://github.com/xampprocky/tokei/commit/e48dfe8f397743c5e18857b01bbe1dd0f350e01f))
    - Fixed musl builds ([`ed7ba86`](https://github.com/xampprocky/tokei/commit/ed7ba866ccdb7d92e0756cefc2cab9a35200eca6))
</details>

# 5.0.0

- Optimised internals

**Added languages:**

- @GungnirInd Added GDScript
- @tuncer Differentiate between sh and Bash, Added Cogent, F\*, F#
- @pthariensflame Added Agda

# 4.5.4 (2016-12-10)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 12 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#81](https://github.com/xampprocky/tokei/issues/81), [#84](https://github.com/xampprocky/tokei/issues/84), [#85](https://github.com/xampprocky/tokei/issues/85)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#81](https://github.com/xampprocky/tokei/issues/81)**
    - Add GDScript support ([`0510477`](https://github.com/xampprocky/tokei/commit/05104778b1e793b5154715da936d3f6efda15120))
 * **[#84](https://github.com/xampprocky/tokei/issues/84)**
    - Differentiate between bash and sh ([`698e79a`](https://github.com/xampprocky/tokei/commit/698e79aa151de473ca830244a38d31bdae5ec465))
 * **[#85](https://github.com/xampprocky/tokei/issues/85)**
    - Update to serde_json 0.8.4 ([`d8c77fa`](https://github.com/xampprocky/tokei/commit/d8c77fa5f6d08b2eebbcf5c953722d34e4f958c0))
 * **Uncategorized**
    - Auto deployed releases now come with all serialisation formats ([`c3bcca8`](https://github.com/xampprocky/tokei/commit/c3bcca8caff3573f69d0b36d8fd14eea7ff8cc86))
    - Version bump ([`759e49d`](https://github.com/xampprocky/tokei/commit/759e49d1eac65cc97335ea4da92755a8402e3211))
    - Updated dependencies, changed some unwraps with expect, and made build process drier ([`5c11e76`](https://github.com/xampprocky/tokei/commit/5c11e76c88f39aea7ffd39a038f93eecddf62248))
    - Fixed serialisation features ([`ca9d192`](https://github.com/xampprocky/tokei/commit/ca9d1926fe6be402ec8236db7cdcc08f9b116c9a))
    - Update README.md ([`ce1cae2`](https://github.com/xampprocky/tokei/commit/ce1cae217840e0346489c985c0f6be11c79427b8))
    - Update README.md ([`b0ed886`](https://github.com/xampprocky/tokei/commit/b0ed88636e7f57d1220e5dba634e2ee4ceeac199))
    - Added CI from rust-everywhere ([`c659808`](https://github.com/xampprocky/tokei/commit/c65980889702e5219121efadbf658820778d9dbd))
</details>

# 4.5.2 (2016-11-27)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix for languages with no single line comments panicing ([`bf304dd`](https://github.com/xampprocky/tokei/commit/bf304dd8ebb1b215776ceba221e4875ef4e3eb06))
    - Version bump ([`02c8ebe`](https://github.com/xampprocky/tokei/commit/02c8ebe122d5d5bb47aad906e6b457ac9fef9737))
    - Implemented fix for Sender never dropping Special thanks to: mbrubeck ([`f732e7c`](https://github.com/xampprocky/tokei/commit/f732e7cb0d1e4b0a80c82a0be178ab3956cc3b56))
    - included build file ([`a0b12fc`](https://github.com/xampprocky/tokei/commit/a0b12fc4a2cbdbf1d6aadfa87d1a5c760beb974b))
</details>

# 4.5.0 (2016-11-27)

- Added Regex based hueristics so more expensive multi line handling isn't used
  if there are no multi line comments in the file.
- Now uses the `ignore` crate for getting files. Which now also makes
  determining language from path/file parallelised
- File counting used to only be parallelised per language, now it is also
  parallelised per file per language.
- Updated homepage, and documentation links
- @rmbreak Tokei will now not add directories with `foo.bar` like syntax
  to a language.
- @Michael-F-Bryan tokei will now exit gracefully when a feature is missing
  instead of panicing

**Added languages:**

- @hauleth Added Elixir support

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 34 commits contributed to the release over the course of 96 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 13 unique issues were worked on: [#56](https://github.com/xampprocky/tokei/issues/56), [#59](https://github.com/xampprocky/tokei/issues/59), [#61](https://github.com/xampprocky/tokei/issues/61), [#64](https://github.com/xampprocky/tokei/issues/64), [#66](https://github.com/xampprocky/tokei/issues/66), [#69](https://github.com/xampprocky/tokei/issues/69), [#70](https://github.com/xampprocky/tokei/issues/70), [#71](https://github.com/xampprocky/tokei/issues/71), [#72](https://github.com/xampprocky/tokei/issues/72), [#74](https://github.com/xampprocky/tokei/issues/74), [#75](https://github.com/xampprocky/tokei/issues/75), [#77](https://github.com/xampprocky/tokei/issues/77), [#78](https://github.com/xampprocky/tokei/issues/78)

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#56](https://github.com/xampprocky/tokei/issues/56)**
    - Avoid panicking on non-character-boundary inside string slice ([`df2689a`](https://github.com/xampprocky/tokei/commit/df2689a2d5f8f872604e7e0a1a262662cd9a5aa8))
 * **[#59](https://github.com/xampprocky/tokei/issues/59)**
    - Made a series of modifications to the languages comments: ([`d13af4d`](https://github.com/xampprocky/tokei/commit/d13af4d177cefe7d297b04796cd87a1abd4181b5))
 * **[#61](https://github.com/xampprocky/tokei/issues/61)**
    - Fixes issue #52 ([`3f5fb6c`](https://github.com/xampprocky/tokei/commit/3f5fb6ca117c3981a3db5f70015e464cb7866eaf))
 * **[#64](https://github.com/xampprocky/tokei/issues/64)**
    - support tcl ([`c9ca419`](https://github.com/xampprocky/tokei/commit/c9ca419b715ac1f0069d29a1429a4f49b99b12c2))
 * **[#66](https://github.com/xampprocky/tokei/issues/66)**
    - add .pm to Perl extensions ([`0b7a249`](https://github.com/xampprocky/tokei/commit/0b7a2493b6ed9220df60ab0c63ec97fd55b591b4))
 * **[#69](https://github.com/xampprocky/tokei/issues/69)**
    - pulled two if statements together and removed one needless reference ([`2fffe17`](https://github.com/xampprocky/tokei/commit/2fffe17787e59f57e6baf000797ad1b3b5a42e40))
 * **[#70](https://github.com/xampprocky/tokei/issues/70)**
    - Add support for lean, and fixed single comment regression ([`9faa8d4`](https://github.com/xampprocky/tokei/commit/9faa8d4210ba19ad76d736e46e4963cfcef329d6))
 * **[#71](https://github.com/xampprocky/tokei/issues/71)**
    - Added support for elm ([`94ac1f6`](https://github.com/xampprocky/tokei/commit/94ac1f6f2ccb43da7a614fb737428d43eba5ecb9))
 * **[#72](https://github.com/xampprocky/tokei/issues/72)**
    - Added GLSL support ([`d981333`](https://github.com/xampprocky/tokei/commit/d98133347eb7fbb4d60f11fb9bd40ac8b31c421d))
 * **[#74](https://github.com/xampprocky/tokei/issues/74)**
    - Added long verbose arg name ([`f028783`](https://github.com/xampprocky/tokei/commit/f0287837b3fea0c0b860983fd6847fef05cbda26))
 * **[#75](https://github.com/xampprocky/tokei/issues/75)**
    - Only add regular files to path list ([`24fec67`](https://github.com/xampprocky/tokei/commit/24fec6788f4c9157876dfd355ae213ac51b4774f))
 * **[#77](https://github.com/xampprocky/tokei/issues/77)**
    - Exit gracefully instead of panicking when using a feature that isn't compiled in ([`ed4f898`](https://github.com/xampprocky/tokei/commit/ed4f89816dfdbaddde99f051d0f6aa7301acb231))
 * **[#78](https://github.com/xampprocky/tokei/issues/78)**
    - Add Elixir support ([`04f6e1e`](https://github.com/xampprocky/tokei/commit/04f6e1e3e4980749b5f45a65a2d4175bf142dae6))
 * **Uncategorized**
    - Version bump, updated README, and CHANGELOG ([`9c0e7a2`](https://github.com/xampprocky/tokei/commit/9c0e7a204f68dda789dccb4fc9e2296b90e0f785))
    - Switched to using ignore, more parallelisation ([`913752d`](https://github.com/xampprocky/tokei/commit/913752de0cce80ea7817183e0eaf43ed8c2f4991))
    - Added heuristics ([`a72121f`](https://github.com/xampprocky/tokei/commit/a72121f67b206f42057fc01533ec3e5eb3ffa178))
    - updated docs link to docs.rs ([`8f028b7`](https://github.com/xampprocky/tokei/commit/8f028b7aacf7d567645a6e9d6260836bd6d58cee))
    - Version bump ([`9699db7`](https://github.com/xampprocky/tokei/commit/9699db777e1899d3f8c8a6eeaee7f66ef2a064b2))
    - Simplified language definitions. ([`5fab02a`](https://github.com/xampprocky/tokei/commit/5fab02a76d34c8312439d918db9d48fd4c733f6c))
    - fixed regression where lines and files weren't being sorted ([`5ced28a`](https://github.com/xampprocky/tokei/commit/5ced28ab86233351e1e6dd3e339bb493053d4882))
    - Update CHANGELOG.md ([`86c4517`](https://github.com/xampprocky/tokei/commit/86c4517977481aad4483769caf93797967257ad3))
    - Update README.md ([`f730915`](https://github.com/xampprocky/tokei/commit/f73091587b5d27a56d87379b2e515f9ab98842b1))
    - fixed typo in cargo.toml ([`3c3da8f`](https://github.com/xampprocky/tokei/commit/3c3da8f8f3032871b5c31893997c87b3b8db0091))
    - updated cargo.lock ([`073c64f`](https://github.com/xampprocky/tokei/commit/073c64fc728e36f6c4af26d4c61a5af359e5f56d))
    - Version bump, and updated changelog ([`4541d2f`](https://github.com/xampprocky/tokei/commit/4541d2f394d7e7a520a3d113753b12faa8bec3eb))
    - Changed logging behaviour based on feedback. ([`1839132`](https://github.com/xampprocky/tokei/commit/1839132aa50a1a2f26e3d4b42a196a686f810252))
    - Enabled logging, using log & env_logger. Fixes #54 ([`2e5c46d`](https://github.com/xampprocky/tokei/commit/2e5c46d71ade3b262d5df90850180a925dd133aa))
    - Added encoding dependency, so now tokei can handle all files that encoding supports, fixes #53 ([`9ec7499`](https://github.com/xampprocky/tokei/commit/9ec7499a2dd404b82d333b66e44082b391a8507e))
    - fixes #57 ([`5f86629`](https://github.com/xampprocky/tokei/commit/5f86629e17520a9b9a824a8d2de7022575019f28))
    - fixes #58 ([`0da50fc`](https://github.com/xampprocky/tokei/commit/0da50fc5448c3d8c665b86822f21694f662d171f))
    - Support lossy over strict utf8 ([`dc1fe7b`](https://github.com/xampprocky/tokei/commit/dc1fe7bf1f589eb3eb9f7ba900ec079c056684b9))
    - fixed regression, moved to tempdir for testing. ([`bfc80d5`](https://github.com/xampprocky/tokei/commit/bfc80d5b69a2d54dd93a5fac40956510c29836cd))
    - add hex ihex rst ([`e2b7328`](https://github.com/xampprocky/tokei/commit/e2b732827553322c81d6f38d729cb7b0e27cbc80))
    - Added Asp, Asp.NET, Assmebly extension, Razor ([`88bc193`](https://github.com/xampprocky/tokei/commit/88bc1935c84b6025e959b5e8f87778768f71c066))
</details>

# 4.4.0

- Simplified language definitions, now consolidated into a single JSON file.
- Fixed regression where lines and files weren't sorted.
- @llogiq : made clippy fixes
- @lligo : Added long verbose name

**Added languages:**

- @little-dude : Tcl(_tcl_)
- @svenstaro : GLSL(_vert, tesc, tese, geom, frag, comp_)
- @not-fl3 : Elm(_elm_)

**Changes to existing languages:**

- @xpayn : Added `pm` extension to Perl.

# 4.3.0

- @lligo : Tokei no longer panics on non-character-boundary when printing file names.
- Fixed regression where no comment style files(_json, markdown_) weren't counted.
- Tokei can now handle files in different encodings.(_using the [encoding](https://crates.io/crates/encoding) library_)
- Tokei now prints errors instead of sliently skipping them.
- Tokei can now print unused extensions using `-v` option.

**Added languages:**

- Asp(_asa, asp_)
- Asp.NET(_asax, ascx, asmx, aspx, master, sitemap, webinfo_)
- Hex(_hex_)
- Intel Hex(_ihex_)
- ReStructuredText(_rst_)
- Razor(_cshtml_)

**Changes to existing languages Thanks to @mwilli20 :**

- Another Ada extension(_pad_)
- Assembly - Uses `' '` or `" "` and added another extension(_asm_)
- Bash - Uses `' '` or `" "`
- Batch - They don't use quotes for strings, added `::`
- Cold Fusion - Uses `' '` or `" "`
- D - Uses `" "` or
- Dart - Uses `" "` or `' '` or `""" """` or `''' '''`
- Forth - Uses `" "` but new, doesn't have a preset
- Fortrans - Use `" "` or `' '`
- Idris - Uses `" "` or `""" """`
- Julia - Uses `" "` or `""" """`
- Kotlin - Uses `" "` or `""" """`
- Lisp - Comments can be nested
- Moustache - Uses `" "` or `' '`
- Nim - Uses `" "` or `""" """`
- Pascal - Uses `' '`
- Perl - Uses `" "` or `' '`
- Php - Uses `" "` or `' '`
- Python - Uses `" "` or `' '` or `""" """` or `''' '''`
- Ruby - Uses `" "` or `' '`
- Sass - Uses `" "` or `' '`
- Sql - Uses `' '`
- Toml - Uses `" "` or `' '` or `""" """` or `''' '''`
- Typescript - Uses `" "` or `' '` or
- Vimscript - Uses `" "` or `' '`
- Yaml - Uses `" "` or `' '`
- Zsh - Uses `" "` or `' '`
- Clojure - Removed `#`
- Forth - `( Comment)` style comments need a space after the opening paren
- Haskell - Has nested comments
- Idris - Has nested comments
- Jai - Has nested block comments
- Julia - Has nested block comments
- Kotlin - Has nested block comments
- Pascal - Pascal should be multiline from `{` or `(*` to `}` or `*)`
- Perl - Perl5 and earlier for multiline comments need `=pod` to `=cut`.
- Swift - Has nested block comments

### Tokei's code count

```
-------------------------------------------------------------------------------
 Language            Files        Lines         Code     Comments       Blanks
-------------------------------------------------------------------------------
 Rust                   13         2413         1596          601          216
-------------------------------------------------------------------------------
 |ib\language\languages.rs          693          420          197           76
 |anguage\language_type.rs          500          386          102           12
 .\src\main.rs                      314          256           17           41
 |lib\language\language.rs          356          166          166           24
 .\src\lib\utils\fs.rs              129          107            9           13
 |\lib\utils\multi_line.rs          149           89           39           21
 .\src\lib\utils\macros.rs           59           50            3            6
 .\src\lib\stats.rs                  63           45           12            6
 .\src\lib\lib.rs                    76           25           47            4
 .\src\lib\build.rs                  31           23            0            8
 .\src\lib\sort.rs                   28           19            6            3
 .\src\lib\language\mod.rs           11            6            3            2
 .\src\lib\utils\mod.rs               4            4            0            0
-------------------------------------------------------------------------------
 Markdown                4          492          492            0            0
-------------------------------------------------------------------------------
 .\README.md                        252          252            0            0
 .\CHANGELOG.md                     202          202            0            0
 .\CONTRIBUTING.md                   25           25            0            0
 .\CONTRIBUTORS.md                   13           13            0            0
-------------------------------------------------------------------------------
 YAML                    2           70           67            3            0
-------------------------------------------------------------------------------
 .\cli.yml                           53           50            3            0
 .\.travis.yml                       17           17            0            0
-------------------------------------------------------------------------------
 TOML                    1           80           65            0           15
-------------------------------------------------------------------------------
 .\Cargo.toml                        80           65            0           15
-------------------------------------------------------------------------------
 Autoconf                1            9            7            1            1
-------------------------------------------------------------------------------
 .\src\lib\lib.rs.in                  9            7            1            1
-------------------------------------------------------------------------------
 Total                  21         3064         2227          605          232
-------------------------------------------------------------------------------
```

# 4.2.0 (2016-08-21)

Tokei is now more precise, and shouldn't ever panic.
Tokei now handles comments in quotes and more precise nested comments properly.
Fixes #53

### Tokei's code count.

```
-------------------------------------------------------------------------------
 Language            Files        Lines         Code     Comments       Blanks
-------------------------------------------------------------------------------
 Rust                   13         2303         1487          594          222
-------------------------------------------------------------------------------
 |ib\language\languages.rs          682          401          198           83
 |anguage\language_type.rs          467          359           96           12
 .\src\main.rs                      302          243           17           42
 |lib\language\language.rs          356          166          166           24
 .\src\lib\utils\fs.rs              116           95            9           12
 |\lib\utils\multi_line.rs          156           93           41           22
 .\src\lib\stats.rs                  54           36           12            6
 .\src\lib\build.rs                  31           23            0            8
 .\src\lib\lib.rs                    69           22           43            4
 .\src\lib\utils\macros.rs           27           20            3            4
 .\src\lib\sort.rs                   28           19            6            3
 .\src\lib\language\mod.rs           11            6            3            2
 .\src\lib\utils\mod.rs               4            4            0            0
-------------------------------------------------------------------------------
 YAML                    2           68           65            3            0
-------------------------------------------------------------------------------
 .\cli.yml                           49           46            3            0
 .\.travis.yml                       19           19            0            0
-------------------------------------------------------------------------------
 TOML                    1           71           58            0           13
-------------------------------------------------------------------------------
 .\Cargo.toml                        71           58            0           13
-------------------------------------------------------------------------------
 Autoconf                1            9            7            1            1
-------------------------------------------------------------------------------
 .\src\lib\lib.rs.in                  9            7            1            1
-------------------------------------------------------------------------------
 Total                  17         2451         1617          598          236
-------------------------------------------------------------------------------
```

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 13 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - updated changelog ([`704e47a`](https://github.com/xampprocky/tokei/commit/704e47a01a16c4eac59c941082ca83b14bc834ec))
    - Merge branch 'tune-up' ([`2b75021`](https://github.com/xampprocky/tokei/commit/2b75021a8d0b4d16c902f5769f8c1b76e8ce4b53))
    - 4.2.0 fixes #51 ([`68c62f8`](https://github.com/xampprocky/tokei/commit/68c62f8dc29f803183cbf8ae3d04208966baeb6e))
    - Implmented stack based functionality ([`e1a4529`](https://github.com/xampprocky/tokei/commit/e1a4529e2db89547c274c55cf4a0bb680d81ea6a))
    - half completed rework of handling multilines ([`e2d3de4`](https://github.com/xampprocky/tokei/commit/e2d3de4a2ea76d0dbc66a3aabd637404e24d49d4))
    - version bump, speed increase ([`c52597c`](https://github.com/xampprocky/tokei/commit/c52597c806ca1835bc706ba8b77d11cffd861465))
    - Version 4.1.0 ([`d5fa7da`](https://github.com/xampprocky/tokei/commit/d5fa7da93719a6a6aee69909d058280219719af6))
</details>

# 4.1.0

Tokei is now **~40%** faster.

**Added languages**

- Ada
- Forth

# 4.0.0 (2016-08-01)

Tokei now has a minimal version without `serde` for faster compilation.

Updated various dependencies.

Internal dependencies removed.

## Regressions

- CBOR is not supported till it supports `serde 0.8`

**Added languages**

- Handlebars

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 50 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #49 from acdha/patch-1 ([`e44a0c7`](https://github.com/xampprocky/tokei/commit/e44a0c7ce78b7eb240141d5859fb319200d1cb2e))
    - changed to toml on crates.io ([`a266e6e`](https://github.com/xampprocky/tokei/commit/a266e6e7dabb7a8bb64179b223371a391953c50d))
    - Increased stack size for builds ([`87a7b44`](https://github.com/xampprocky/tokei/commit/87a7b44af5f22b48a1927fadf160321718cd934e))
    - fixed travis ([`62853f7`](https://github.com/xampprocky/tokei/commit/62853f7840ae998bfc98fa22cd03249af64f7d9d))
    - fixed build issue on main ([`2d7829c`](https://github.com/xampprocky/tokei/commit/2d7829c9562687eec51f2e96848798bba80c3c50))
    - 4.0.0 ([`0f44c5b`](https://github.com/xampprocky/tokei/commit/0f44c5b291ff8097b9c2589debeda31f5bc93860))
    - Minimal version ([`e443540`](https://github.com/xampprocky/tokei/commit/e4435402de64443ed1712d3674495d9c75fc17ae))
    - Weird trait problem ([`b2790c4`](https://github.com/xampprocky/tokei/commit/b2790c4eee1b7b2b331171a3284d97ca23832be0))
    - unfisnished move ([`c0c916b`](https://github.com/xampprocky/tokei/commit/c0c916beabf1317a06cfc3b38fc0a90bcec7e30e))
    - First draft ([`bfbfaa6`](https://github.com/xampprocky/tokei/commit/bfbfaa60fc0c5cc2d6f225ad99e30529ee7f843b))
    - Document that `--exclude` accepts a list of values ([`de22ee8`](https://github.com/xampprocky/tokei/commit/de22ee831b6b347b952e1fd20983841bbc4a01e4))
    - removed syntex used new serde build process ([`4e671b5`](https://github.com/xampprocky/tokei/commit/4e671b5fa9a13752aec5f017f873879d157b9a7e))
    - forgot to add cli to whitelist ([`f56efb6`](https://github.com/xampprocky/tokei/commit/f56efb6f22683a883a6c4da3d34705a180cc1077))
    - moved to cargo whitelist ([`31a8ff0`](https://github.com/xampprocky/tokei/commit/31a8ff02cb44cbce92256035205aa9f958545a9e))
    - version 3.0.1 updated dependencies ([`a2bf592`](https://github.com/xampprocky/tokei/commit/a2bf592ff668c8d6a407e73be7a26e918359573f))
</details>

# 3.0.0 (2016-06-11)

Tokei is now available as a library.

Tokei now has a lot more tests.

Tokei now supports TOML

Fixed #41

Fixed #44

Fixed #45

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 34 commits contributed to the release over the course of 19 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - removed sublime files ([`e996bd9`](https://github.com/xampprocky/tokei/commit/e996bd9841ef32cf6c7ef97218190832ad306f15))
    - updated readme ([`4b49052`](https://github.com/xampprocky/tokei/commit/4b49052c979de1d0ed8087f4007d542c41f260fd))
    - updated readme ([`350324a`](https://github.com/xampprocky/tokei/commit/350324a18d6832820a8d6fd7f5666bc54835806b))
    - updated readme ([`f908968`](https://github.com/xampprocky/tokei/commit/f908968d01a3b17d73267b4981ebac5ef0c5a07d))
    - version bump plus updated readme and changelog ([`466a154`](https://github.com/xampprocky/tokei/commit/466a154243cc1fa1b6b58d233da8572af7cd5533))
    - Fixed tests ([`b46bf60`](https://github.com/xampprocky/tokei/commit/b46bf60ab806f91ad414d07d9925c0c8000ca291))
    - added documentation ([`9a3dbf1`](https://github.com/xampprocky/tokei/commit/9a3dbf1a4a8ec33c780e17f415a7480bcb1a0540))
    - removed print from language ([`b48de87`](https://github.com/xampprocky/tokei/commit/b48de870d67c6aaf30403c877c924a42566d8629))
    - forgot files ([`4f6cf40`](https://github.com/xampprocky/tokei/commit/4f6cf406e7cedd72b9960128f03ca7ac61fac753))
    - feature parity ([`1e6a4c1`](https://github.com/xampprocky/tokei/commit/1e6a4c1015680243e7774b04900db03b4ba4b9df))
    - resolved merge ([`73a777e`](https://github.com/xampprocky/tokei/commit/73a777e72049d5ac5442733435021771927bb338))
    - fix for no-comment languages: Json, Markdown, Text ([`1db48f1`](https://github.com/xampprocky/tokei/commit/1db48f1e7c5fec1aaedef5b71e2465bb5f540bb6))
    - version bump ([`3f81006`](https://github.com/xampprocky/tokei/commit/3f81006293df5c3eb6a642f51d99551d9a56ea40))
    - Merge pull request #43 from liigo/fix-no-comment-languages ([`ed814cb`](https://github.com/xampprocky/tokei/commit/ed814cbb2cf9a2038d21d67cb72c6f1eff261625))
    - fixed merge conflict ([`08bc17c`](https://github.com/xampprocky/tokei/commit/08bc17c4dcbaea550184ad85c916ce29428e919d))
    - version bump 2.1.2 ([`afaa994`](https://github.com/xampprocky/tokei/commit/afaa994ca8a528b4cc38b50a39bea7ce154748e8))
    - Updated syntex, and serde_codegen dependencies ([`461568e`](https://github.com/xampprocky/tokei/commit/461568e53d2fda230660a0e254350977b400592a))
    - first attempt at moving stuff around ([`76be54b`](https://github.com/xampprocky/tokei/commit/76be54b67dee90de193c7950fac8ad70db7618df))
    - Re ordering the files ([`5ecad64`](https://github.com/xampprocky/tokei/commit/5ecad644f4df7d2b8ab9218729f30f5e783f5f75))
    - Merge pull request #40 from Phlosioneer/fix-tests-patch ([`fcbf5c7`](https://github.com/xampprocky/tokei/commit/fcbf5c7edb29d4a94019a7abe4c051b374c8f2bb))
    - Fixed parameter count mismatch. ([`33395b9`](https://github.com/xampprocky/tokei/commit/33395b9c567b4257afc0778d5adec4941fa7de4d))
    - Merge branch 'master' of github.com:Aaronepower/tokei ([`6cedcb0`](https://github.com/xampprocky/tokei/commit/6cedcb072ae4b3923d3b7c5ed5e535ff95dea22d))
    - Added logo. ([`fd191ff`](https://github.com/xampprocky/tokei/commit/fd191ff273aaea4e1f63047d03eeb10573b56731))
    - Update README.md ([`a4b2404`](https://github.com/xampprocky/tokei/commit/a4b2404a3f9fb3fee349144c129a905d5425e7f1))
    - Update CHANGELOG.md ([`450d75b`](https://github.com/xampprocky/tokei/commit/450d75b03ea930621fdd18c33333e19733bdc285))
    - fixes #39 ([`f65a8c9`](https://github.com/xampprocky/tokei/commit/f65a8c98252886c791552af4e6cc5a3161785d60))
    - version bump ([`2b1bfa9`](https://github.com/xampprocky/tokei/commit/2b1bfa9f7df21e010e0df75dabc4036c52cd0fed))
    - fixed comments sorting ([`42a7059`](https://github.com/xampprocky/tokei/commit/42a7059202274cb3ca6beb01059e81cbd4c406e5))
    - fixed tests ([`c5750f9`](https://github.com/xampprocky/tokei/commit/c5750f9911940305f7a4ff6fce4cf021f7ba5ffa))
    - removed more wildcards ([`dc4d37a`](https://github.com/xampprocky/tokei/commit/dc4d37aa014dd4f8aabc1bb91f4ec85d944ff45e))
    - fremoved wildcards ([`dc9b27f`](https://github.com/xampprocky/tokei/commit/dc9b27f0b6cbed82c6d88085ae985dd99f8bca2a))
    - Merge branch 'master' of github.com:Aaronepower/tokei ([`4a891d2`](https://github.com/xampprocky/tokei/commit/4a891d2c551536e7bb6e525aa4e6b7f937d74495))
    - version bump, and added explaintion of file_input ([`2e4a39b`](https://github.com/xampprocky/tokei/commit/2e4a39b58ad8e59298f5d30f88b098fa0dc3b25b))
    - Update CHANGELOG.md ([`5f66c64`](https://github.com/xampprocky/tokei/commit/5f66c64a3dbdb15e06cc57fb16e624d8394f50af))
</details>

# 2.1.0 (2016-05-22)

Tokei, can now output results in various formats(_cbor, json, yaml_)

Conversely tokei can now take in results in those formats, and add them to the current run.

Premilarily support for nested comments(_currently only supported for rust_)

Change in the output format [PR #35](https://github.com/XAMPPRocky/tokei/pull/35)

Moved `.sc` from Lisp to Scala.

Internals changed to allow for multiple multi line comment formats.

**Added languages:**

- Isabelle

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 7 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update CHANGELOG.md ([`a381584`](https://github.com/xampprocky/tokei/commit/a3815840b03d87d5d99e1a44f8d09c6e2bc6d464))
    - Update CHANGELOG.md ([`8112cbb`](https://github.com/xampprocky/tokei/commit/8112cbb0d439d3d199a418b95a2f05759d91adae))
    - 2.1.0 IO ([`350cb5f`](https://github.com/xampprocky/tokei/commit/350cb5f30f9c7260fc2f7a83bdb0d3bb9759c9a6))
    - ICE error ([`bc83358`](https://github.com/xampprocky/tokei/commit/bc83358d820a8912bd9f3b9fe1c33e3aacf301d2))
    - Prints code lines before comments and blanks ([`315f088`](https://github.com/xampprocky/tokei/commit/315f0882d1d963776f466c24cd7b6ce0814ceb94))
    - fixed miscounting total files ([`77c52b4`](https://github.com/xampprocky/tokei/commit/77c52b4aea8f1afe704bc0c2de9ee9c21a67e0b1))
    - Merge branch 'master' into io ([`a126328`](https://github.com/xampprocky/tokei/commit/a126328d2e6d843637f50655c34e578adfc56663))
    - resolved conflict ([`ee14e7e`](https://github.com/xampprocky/tokei/commit/ee14e7ed5c93276ad8aaef5fc6a1fdd45116b24d))
    - change language.total, to something more descriptive ([`ff5db06`](https://github.com/xampprocky/tokei/commit/ff5db0626d71cebaaa31c44b084cf3b64a2a8be4))
    - Merge pull request #35 from liigo/master ([`5fa350e`](https://github.com/xampprocky/tokei/commit/5fa350e988220b6adcfa1882b943185ddbf48ee0))
    - reformatted cli.yml, and added additional constructor ([`43ea4d4`](https://github.com/xampprocky/tokei/commit/43ea4d425682849bedfbbf7c31d7b37cc408e6e9))
    - Update README.md ([`a6de3bb`](https://github.com/xampprocky/tokei/commit/a6de3bbd78e2da730649baa376f7ad6fbd984001))
    - Update README.md ([`39e771f`](https://github.com/xampprocky/tokei/commit/39e771fdcb10ff840167a0146511ef8728260cc1))
    - Merge pull request #34 from llogiq/clippy ([`8bc0356`](https://github.com/xampprocky/tokei/commit/8bc035605aa5f4f5cf7cae26d1a35a6545f180ec))
    - another two clippy warnings ([`945ec91`](https://github.com/xampprocky/tokei/commit/945ec91484ae040165c9604064d6021f61a1785d))
</details>

# 2.0.0 (2016-05-13)

Major rewrite, now parallelized.
Can now support sorting files.
Added a progress message for when it is counting files.
Fixed #29

**Added languages:**

- Coq
- Erlang
- Kotlin
- Idris
- Nim
- Oz
- Prolog
- Qcl
- Scala
- Unreal Script
- Wolfram

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 11 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - updated cargo.lock ([`a4ae97f`](https://github.com/xampprocky/tokei/commit/a4ae97f6b2e055e56e5011fda31f88203e4b4fad))
    - updated dependencies ([`cccc460`](https://github.com/xampprocky/tokei/commit/cccc460c8ebc64fcee283a7dd86f9e53879363c3))
    - Merge branch 'master' of github.com:Aaronepower/tokei ([`f9652fd`](https://github.com/xampprocky/tokei/commit/f9652fd1f209cbad825f2ab3fc1c26adb2f2049a))
    - forgot file ([`8c6b6f2`](https://github.com/xampprocky/tokei/commit/8c6b6f28b2bf15186e13ab7c197c91b9572498d5))
    - version 2.0 ([`0287ab0`](https://github.com/xampprocky/tokei/commit/0287ab0946d30e94997acb11a4037110daea7ba0))
    - moved functions to fsutil, and switched from RefCell to an Enum based system ([`ebee004`](https://github.com/xampprocky/tokei/commit/ebee004a8f8c9995b96d89be6e3fdc0e393f1f3d))
    - Update README.md ([`9aae543`](https://github.com/xampprocky/tokei/commit/9aae5431e7badb2e52970d6397ce7ec12d54bd86))
</details>

# 1.6.0 (2016-05-01)

Added file counting.

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 35 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - fixed unused import ([`db2060e`](https://github.com/xampprocky/tokei/commit/db2060e0f2cb944ec47b666774560204c4a3ca8a))
    - implemented file counting. ([`c28bfa8`](https://github.com/xampprocky/tokei/commit/c28bfa8e9ef215b105fc24cd858de9945480c51d))
    - Merge branch 'master' of github.com:Aaronepower/tokei ([`e0fa667`](https://github.com/xampprocky/tokei/commit/e0fa667f8f36c8df019b7bef4f105b50305468d2))
    - Added .markdown, reduced runtime borrows, fixed 25 ([`2a97d41`](https://github.com/xampprocky/tokei/commit/2a97d41c20ccf9a6f9b8ce15ca2815472160b353))
    - Merge pull request #26 from hhatto/support-more-language ([`fe1cb5d`](https://github.com/xampprocky/tokei/commit/fe1cb5dc2443444d746375c53f028b042a08f342))
    - support .zsh ([`e827aaf`](https://github.com/xampprocky/tokei/commit/e827aaf15a178c71070715cbb8be665a48740850))
    - Merge branch 'master' into support-more-language ([`4f88d5d`](https://github.com/xampprocky/tokei/commit/4f88d5dabf21aeccaba7a9c304a0c6d12ed91615))
    - support protocol buffers file ([`2aefa97`](https://github.com/xampprocky/tokei/commit/2aefa97a3c0ba6460daa968b5ece5d38809388f3))
    - supported mustache file ([`bb593b6`](https://github.com/xampprocky/tokei/commit/bb593b6271f1bda828276818c16aa88227314966))
    - fixed skipped Makefile ([`7738a67`](https://github.com/xampprocky/tokei/commit/7738a67771b18823ce9275bf9d4a365980999dbc))
    - Merge pull request #24 from hhatto/support-mustache ([`116dfe7`](https://github.com/xampprocky/tokei/commit/116dfe7f16f1fe7e66ad369e1c326857657eb81f))
    - Merge pull request #23 from hhatto/fix-skipped-makefile ([`2416a66`](https://github.com/xampprocky/tokei/commit/2416a66f1afe18bb7ae4a40a71a663c4a0fd9129))
    - Fixes #22 ([`c06cadd`](https://github.com/xampprocky/tokei/commit/c06cadd4d48cd9621a19b8b719891716df7ea800))
    - fixed some clippy warnings ([`a8fce7c`](https://github.com/xampprocky/tokei/commit/a8fce7c33e1f070ea367155a71eb0840253b273c))
    - Merge pull request #21 from llogiq/clippy ([`3229eaa`](https://github.com/xampprocky/tokei/commit/3229eaa690d703509f785d400010d51fea194826))
</details>

# 1.5.0 (2016-03-26)

Added Shebang support.

**Added languages:**

- Assembly
- LD Scripts
- Device Trees
- Makefiles
- Plain Text
- C Shell

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 57 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Updated language list ([`c7c670b`](https://github.com/xampprocky/tokei/commit/c7c670be5e379861071c19ddb3baebc0cb7090c4))
    - updated changelog ([`528c761`](https://github.com/xampprocky/tokei/commit/528c761b39c793e84c5068de9559d2444d2c206d))
    - Added shebang support, added Assembly, Plain Text, LD Scripts, Makefiles, C Shell, and Device Trees ([`db71efa`](https://github.com/xampprocky/tokei/commit/db71efadc04720cf7893829107c551810fb5dd4a))
    - version bump, and changelog ([`307bae1`](https://github.com/xampprocky/tokei/commit/307bae125858a305a9a534ea4843a722d04456b0))
    - changed formatting ([`8d74969`](https://github.com/xampprocky/tokei/commit/8d7496905b95ca6d893e285b38b9f75ac146252a))
    - removed * version dependency ([`f33c615`](https://github.com/xampprocky/tokei/commit/f33c615f152c5fec66c8ea26d95c5553ef80acb3))
    - version bump, added polly, now using walkdir ([`2cc8471`](https://github.com/xampprocky/tokei/commit/2cc84717de5883eec863a295cf0bd8cd79c99edb))
    - moved canonical source to gitlab ([`454a97c`](https://github.com/xampprocky/tokei/commit/454a97cbc030f564fad64e967bf8875d323fe430))
    - Merge branch 'master' of gitlab.com:Aaronepower/tokei ([`d19ebfa`](https://github.com/xampprocky/tokei/commit/d19ebfab2fed72f9cc2fd63902fb6d19c3b2eee1))
    - Update README.md ([`794a094`](https://github.com/xampprocky/tokei/commit/794a094799ae821c7e447003273bf35fc433e928))
    - Update README.md ([`4c7367f`](https://github.com/xampprocky/tokei/commit/4c7367f4f2e41a2f5c5da2e0f06d8121b561fe93))
    - Update README.md ([`37316e0`](https://github.com/xampprocky/tokei/commit/37316e060e3ca2ec01f0cdd953f8a4aaa92364eb))
    - Update README.md ([`b3c1fc3`](https://github.com/xampprocky/tokei/commit/b3c1fc36b56c46f7fe43be1e4b963e954c0f7f7b))
    - resloved merge conflict ([`73b51b7`](https://github.com/xampprocky/tokei/commit/73b51b763e394d2c4b6d4d4e3a2704330403d600))
    - made the main.rs file more ergonomic, used btreemap litreal, and updated to clap 2.0 ([`0a03887`](https://github.com/xampprocky/tokei/commit/0a03887a3443db270b489308704fcdfe540459a3))
    - Add .hxx extension for C++ header ([`7dd3d2d`](https://github.com/xampprocky/tokei/commit/7dd3d2de3a23a060cd43cac9394e67e6a7b2e8cb))
    - Add Lua language ([`1077b5c`](https://github.com/xampprocky/tokei/commit/1077b5ca2329a186e32a7745efa40e0219faa527))
    - Merge pull request #20 from Luthaf/master ([`d865176`](https://github.com/xampprocky/tokei/commit/d86517616c55f70abe5a3603fcc642c95b4352db))
</details>

# 1.4.1

Changed the formatting so tokei looks nice for consoles of 80 column width.

# 1.3.1 (2016-01-26)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 93 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - updated cargo.toml ([`0c1a609`](https://github.com/xampprocky/tokei/commit/0c1a609fe80e1b5d093db9ec515c48d89ed39a32))
    - Merge branch 'master' of github.com:Aaronepower/tokei ([`d1e68c3`](https://github.com/xampprocky/tokei/commit/d1e68c3dc0586313b277e4f095b7e2be98422634))
    - reimplemented comment syntax again, added OCaml, and Standard ML ([`c488932`](https://github.com/xampprocky/tokei/commit/c4889327f8ab4006e6d701b827d7a63a3ad57b7c))
    - Update README.md ([`c86d0b0`](https://github.com/xampprocky/tokei/commit/c86d0b01aff3460177437e83341795722f458182))
    - Update README.md ([`a850690`](https://github.com/xampprocky/tokei/commit/a85069081626c19034a55c88d5ca36ac9896f6b1))
    - reimplemented multi line comment detection, and updated the readme to include cargo install ([`3293a71`](https://github.com/xampprocky/tokei/commit/3293a7195a41cf6c2ad761058ddad0ff7542a862))
    - added files flag fixes: #19 ([`5a2ed87`](https://github.com/xampprocky/tokei/commit/5a2ed87863f9308231804e19042dfe39e104ab55))
    - Merge branch 'master' of https://github.com/Aaronepower/tokei ([`5016096`](https://github.com/xampprocky/tokei/commit/5016096a35c18a58a85457d1ee32e773e3e62052))
    - Trying out travis stuff ([`a262428`](https://github.com/xampprocky/tokei/commit/a26242815ab6579a5675a20c8f38962a94720036))
    - More modern fortran extensions ([`1817de1`](https://github.com/xampprocky/tokei/commit/1817de15856b3e9652d9797e4284166321574321))
    - Merge pull request #18 from Luthaf/master ([`a92c512`](https://github.com/xampprocky/tokei/commit/a92c512d262c3a1588beb78fdcac54ac77f48e8b))
    - version bump, and removed version independent dependencies ([`6e81d72`](https://github.com/xampprocky/tokei/commit/6e81d7235ba51bca5c30b43b3eabbc5cad71dbfc))
    - Updated Readme.md ([`93c7b96`](https://github.com/xampprocky/tokei/commit/93c7b9663c9e60bee5ec04ae31b28ff220315f84))
    - Added metadata, and added jai, and TeX as supported languages ([`60062fa`](https://github.com/xampprocky/tokei/commit/60062fa010cde7c75b149e3e83feffafef53a2c1))
    - Merge branch 'RefCell' ([`7c72ff9`](https://github.com/xampprocky/tokei/commit/7c72ff906a709dddbd4f0b05d9abda3052e47bae))
    - Version bump, and fixed formatting ([`0d116dc`](https://github.com/xampprocky/tokei/commit/0d116dcccb914d053e2c53fb548dc2557a91b838))
</details>

# 1.2.0 (2015-10-11)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 28 commits contributed to the release over the course of 19 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #17 from Aaronepower/RefCell ([`c06871b`](https://github.com/xampprocky/tokei/commit/c06871b56faa0c3cb4d515dce979b455b256b3ea))
    - fixed formatting ([`ee4712b`](https://github.com/xampprocky/tokei/commit/ee4712be1d9a201eb88d5bc9100e0130cb85cbe4))
    - fixed RefCell borrow ([`770590c`](https://github.com/xampprocky/tokei/commit/770590c1ad741b137e56528934a5a4eecb9243cb))
    - fixed merge ([`42b98cc`](https://github.com/xampprocky/tokei/commit/42b98ccc2bbe0fa0e421b214d0efe609854c2597))
    - added support for multiple extensions ([`eb806c9`](https://github.com/xampprocky/tokei/commit/eb806c90174cf5c666307f8aeb1ac6e700e91ccd))
    - made it faster? ([`1481e27`](https://github.com/xampprocky/tokei/commit/1481e27bd2cfda165be3f1c1f6c4c7252ec00327))
    - Added coveralls ([`088ce3c`](https://github.com/xampprocky/tokei/commit/088ce3c4df9ab54450e2f9b507437cb0871559f0))
    - merged ([`ed8b70f`](https://github.com/xampprocky/tokei/commit/ed8b70f15d14e64ca9ae6ef1d7deb2e5188ba5f8))
    - Added coveralls ([`d4e29df`](https://github.com/xampprocky/tokei/commit/d4e29df55512ecf97ab6c90db701193682761692))
    - Update README.md ([`0b2b120`](https://github.com/xampprocky/tokei/commit/0b2b120bf791eaccbdb987cbaf0a16e034a95160))
    - Update README.md ([`9cbca17`](https://github.com/xampprocky/tokei/commit/9cbca17d081d1cb92970284e30087a966e723b2f))
    - Update README.md ([`52a782f`](https://github.com/xampprocky/tokei/commit/52a782fda87cce54bcbabc100cc67064340fee80))
    - Update README.md ([`aa0a450`](https://github.com/xampprocky/tokei/commit/aa0a4508e5ab0d005c9f194daaa7d12e95d5a0d9))
    - Fixes #13 ([`401885e`](https://github.com/xampprocky/tokei/commit/401885ea5d085012646c5fa27fdbb4cb0be2ad13))
    - Accdiently kept invalid arg in cli ([`1986931`](https://github.com/xampprocky/tokei/commit/1986931215c66fa797f8665a9f16c3e04692a2d7))
    - Added TOML, and MD, and removed rest args for exclude Fixes #11 #12 #13 ([`5c7abe0`](https://github.com/xampprocky/tokei/commit/5c7abe0633faaa03a9eb1c4fde4de4682ca55115))
    - merge ([`1fdcea5`](https://github.com/xampprocky/tokei/commit/1fdcea507002c9fd0e9de3367908834d8620f3cc))
    - version bump ([`48d7a10`](https://github.com/xampprocky/tokei/commit/48d7a109f38169ee2701e4cc327481b5339b301d))
    - Update README.md ([`98ef8a6`](https://github.com/xampprocky/tokei/commit/98ef8a6f5d80fc6de6823a8fc573b1eb7279107c))
    - Update README.md ([`70077de`](https://github.com/xampprocky/tokei/commit/70077de829473cd2f69e5442335c82b7a7a522bd))
    - Update README.md ([`a992741`](https://github.com/xampprocky/tokei/commit/a9927413ece991536b92b51faf081058db78e5b1))
    - fixed badge link ([`df0b89c`](https://github.com/xampprocky/tokei/commit/df0b89c05e33b4707aa1a8670887d3206b051581))
    - Added more badges ([`9e9be25`](https://github.com/xampprocky/tokei/commit/9e9be258836f8fee9939506b6dc0d962a481746e))
    - Added Luthaf to CONTRIBUTORS.md ([`7537ba7`](https://github.com/xampprocky/tokei/commit/7537ba7f40d8421b319c2c6e8ba5b53f5ad14c6a))
    - Merge pull request #7 from PhnxRbrn/readme ([`1dfef8e`](https://github.com/xampprocky/tokei/commit/1dfef8ebfd2e380c6324c9df888aa4a17e47d1fa))
    - Merge pull request #6 from PhnxRbrn/copyright ([`273c9fc`](https://github.com/xampprocky/tokei/commit/273c9fc25a5b0be77165f5523a53c5920f4f30ca))
    - Merge pull request #5 from PhnxRbrn/Travis ([`1530fd1`](https://github.com/xampprocky/tokei/commit/1530fd1d945503664add8a39498de52e193bd9fd))
    - Update README.md ([`163dd5b`](https://github.com/xampprocky/tokei/commit/163dd5b5589a21541d02653ed0336372f6cee3e6))
</details>

# 1.1.1 (2015-09-21)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - VERSION 1.1 ([`b5ccff6`](https://github.com/xampprocky/tokei/commit/b5ccff6d37d3f1e2513d83257a4a64206734b877))
    - Merge pull request #4 from PhnxRbrn/E0106WindowsError ([`99b4207`](https://github.com/xampprocky/tokei/commit/99b420783f24e3d564883dad7b6b35124bde987a))
    - Updated README ([`31e6477`](https://github.com/xampprocky/tokei/commit/31e647794572e51574bc114514e332840fced57c))
    - Added copyright and contributors ([`0d84ae0`](https://github.com/xampprocky/tokei/commit/0d84ae01b2dd1c17bf74dfa0ed62133730a65ef1))
    - Added Travis to setup automated testing of commits and PRs ([`8bac036`](https://github.com/xampprocky/tokei/commit/8bac03680aa67570ab322a0019c63df3981210ab))
    - Fixed E0106 error on windows ([`f32fa88`](https://github.com/xampprocky/tokei/commit/f32fa88ea2ea79ba5506e25e5aa95fa0625238be))
    - Add Julia language ([`fc10554`](https://github.com/xampprocky/tokei/commit/fc105545f17e0a870cdecdfa29d4b3e15cc9e725))
    - Fixed FORTRAN Modern comments Fixes #2 ([`077e707`](https://github.com/xampprocky/tokei/commit/077e7075ca224eb2f1b72a26c9d608af5a50df95))
    - Added FORTRAN support Fixes #2 ([`4ff812e`](https://github.com/xampprocky/tokei/commit/4ff812ea7fcbf08167b2b789e5c0a9b16de6ae40))
    - Merge pull request #1 from Luthaf/patch-1 ([`cb59bac`](https://github.com/xampprocky/tokei/commit/cb59bac17c75a3e328eb5778e649a761bdd2ed36))
    - Update README.md ([`5a2d8a0`](https://github.com/xampprocky/tokei/commit/5a2d8a04cd511027fdf71213977111eae3706771))
</details>

# 1.1.0 (2015-09-20)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 80 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update README.md ([`84b2d06`](https://github.com/xampprocky/tokei/commit/84b2d062d1bd80e1bae7ba3ed40a1336ffc7c9d9))
    - removed .lock file ([`020eb07`](https://github.com/xampprocky/tokei/commit/020eb0715b4cedc178c2a59358763511d28580fa))
    - Update README.md ([`3f9a180`](https://github.com/xampprocky/tokei/commit/3f9a18022e839cabbb139c8ef3d5a8df606fe5d9))
    - Update README.md ([`2c7341f`](https://github.com/xampprocky/tokei/commit/2c7341f708a529588dcd04897f92872987e2ea1c))
    - fixed typo ([`4d1b681`](https://github.com/xampprocky/tokei/commit/4d1b681df22719192e074f562ffe16366d62f6a4))
    - Added usage instructions ([`c21eef7`](https://github.com/xampprocky/tokei/commit/c21eef79bb4c0b0b43e13a47014890c34bf610a1))
    - VERSION 1.1, added sorting, added support for 26 languages, replaced getopts with clap ([`3bbc39f`](https://github.com/xampprocky/tokei/commit/3bbc39f9374adffe63b6254a86cc126a1c9a931a))
    - Cleaned up the code. ([`ee045d3`](https://github.com/xampprocky/tokei/commit/ee045d302c9acaa96c4a38264b6fcebe84308dce))
</details>

# 1.0.0 (2015-05-26)

## Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Create README.md ([`93a9d85`](https://github.com/xampprocky/tokei/commit/93a9d85637744ef10a010dc67d822f27860e282f))
    - Removed test folder structure ([`73c666a`](https://github.com/xampprocky/tokei/commit/73c666a7f64695ce60e6570400b9bdd8d1a49c99))
    - Removed test folder structure ([`b60a1d7`](https://github.com/xampprocky/tokei/commit/b60a1d754d5f7bdac5294c2c138b3934cafaeddc))
    - First draft of Rusty CLOC ([`dbd679f`](https://github.com/xampprocky/tokei/commit/dbd679fa2a7e82c27da26eafd3a266c9171a8482))
</details>

# 1.4.0

Changed from handmade recursive file opening to [walkdir](https://github.com/BurntSushi/walkdir)

