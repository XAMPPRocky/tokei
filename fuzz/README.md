## Fuzzing Tokei

Tokei can be fuzzed using libFuzzer, via [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz/).

To launch a fuzzing job: `cargo +nightly fuzz run <target>`

To use multiple cores: `cargo +nightly fuzz run <target> --jobs=6`

Available fuzz targets:

- `parse_from_slice_panic` - checks that all of the LanguageType instances' `parse_from_slice` function doesn't panic.
- `parse_from_slice_total` - checks that the language stats pass a basic test of reporting no more total lines than
  there are new lines in the file. At the time of writing there are low-hanging bugs here.

With the two `parse_from_slice` fuzz targets, it makes sense to share a common corpus directory as they have identical
input formats, e.g.: `cargo +nightly fuzz run parse_from_slice_{panic,total} fuzz/corpus/common`

Potential improvements:

- Cleaner failing test case output. The corpus files include additional data that is used to select the language type,
  so don't represent clean inputs to tokei.
- Do some coverage analysis to check if we're missing any code we would benefit from fuzzing (once it's
  [integrated into cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz/pull/248))
- Check in a generated corpus, and run regression over it in CI.
- Tighten the `parse_from_slice_total` fuzz target to check the total lines exactly matches the number of lines in the
  file.
- Potentially integrate with OSS-Fuzz? Not sure if they would be interested as this doesn't seem likely to identify
  serious security bugs.
