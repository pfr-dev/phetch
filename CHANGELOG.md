## v0.1.13

This release fixes some longstanding display bugs and introduces Tor
support to help you easily browse Gopher more anonymously.

The next release will be `v0.9.0`, the first release candidate for
`phetch v1.0`. We do not anticipate adding any more large features
before the 1.0 release.

### Added

- phetch now supports [Tor][tor]!
- phetch now supports a `~/.config/phetch/phetch.conf` config file! 
- Specify your own config file with `--config FILE`. Or disable with
  `-C`/`--no-config`. 
- Emoji can be used as status indicators. Put `emoji yes` in your
  config file. 🧅🔐 
- `phetch --print URL` will just print a rendered version of the page.
- `phetch -p URL | cat` works now. A simplified, plaintext version of
  the page will be rendered. 
- Tor and TLS can be disabled with `-O` and `-S`, opposites of their
  `-o` and `-s` flags. 
- On macOS, phetch is now available through [Homebrew](brew.sh):
  > brew install xvxx/code/phetch

### Changed

- Wide mode (`ctrl-w`/`w`) is now session-wide, not per-page.
- Many rendering bugs fixed. Pages with UTF8 errors are now displayed.
- Sites that don't prefix their selectors with `/` now work.

[tor]: (https://www.torproject.org/)