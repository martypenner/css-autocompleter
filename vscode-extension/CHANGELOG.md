# Change Log

## 1.1.2

### Patch Changes

- refactor(parser): center everything around files instead of classes. ([`9a556d4`](https://github.com/martypenner/css-to-go/commit/9a556d4b25fd09114fb814d253cd6b3b00ccc404))

  This allows invalidation per file. I also added many tests. Woot!

## 1.1.1

### Patch Changes

- Perf: reserve some space for the final completions list ([`8587024`](https://github.com/martypenner/css-to-go/commit/8587024c882f315d16c089b5243d2cac91dd7db7))

## 1.1.0

### Minor Changes

- Feat: format autocompleted code using prettier ([`4272124`](https://github.com/martypenner/css-to-go/commit/4272124e8efa9025b11edb9f6d373b75e6fb7d07))

### Patch Changes

- Fix: don't trigger autocomplete unless the cursor is in the class attribute ([`93f8440`](https://github.com/martypenner/css-to-go/commit/93f84405130983eb639e20eaba41cf18cd600237))

## 1.0.2

### Patch Changes

- Chore: make icon background transparent ([`be191b6`](https://github.com/martypenner/css-to-go/commit/be191b6455f6fcd71f23b67a815df53c696ea719))

## 1.0.1

### Patch Changes

- Docs: add instructions for adding files in the readme ([`383fc35`](https://github.com/martypenner/css-to-go/commit/383fc3576cb3ec5c7567f6953b8550ac478d10cc))

## 1.0.0

### Major Changes

- Feat: Initial release! ([`49c2397`](https://github.com/martypenner/css-to-go/commit/49c2397f16cd01fc961feb347ed32373ae454d15))

## 0.0.13

### Patch Changes

- Feat: add a cute logo! ([`28ab2aa`](https://github.com/martypenner/css-to-go/commit/28ab2aaf4b3e329992e4600defcac1eb84b79847))

## 0.0.12

### Patch Changes

- ci: fix packaging and publishing dynamic params generation ([`6f672ae`](https://github.com/martypenner/css-to-go/commit/6f672ae978485ca5b43dc737c68380078318f139))

## 0.0.11

### Patch Changes

- Fix: filter files to parse AFTER updating config ([`61f2eec`](https://github.com/martypenner/css-to-go/commit/61f2eec855749d5c2f5a4abb77cc924c9ed94b3c))

## 0.0.10

### Patch Changes

- Fix: remove extraneous config getter ([`7a41431`](https://github.com/martypenner/css-to-go/commit/7a4143118ce1e70cb03cc2759698230c2bf4aa62))

## 0.0.9

### Patch Changes

- Perf: delay extension activation until onLanguage ([`55598a1`](https://github.com/martypenner/css-to-go/commit/55598a132eb696220d951eddf727cdefa95ab907))

## 0.0.8

### Patch Changes

- Fix: use an esbuild plugin instead of sed to more reliably rewrite the engine import path ([`cf2fcc9`](https://github.com/martypenner/css-to-go/commit/cf2fcc952a3c5e21e40ea12b33dbaa3a652da5c0))

## 0.0.7

### Patch Changes

- Fix: ensure config values get updated correctly ([`23a7446`](https://github.com/martypenner/css-to-go/commit/23a744693e4ea4292d5e99ab13a84110f050f014))

## 0.0.6

### Patch Changes

- Docs: try fixing media path again ([`b90f9b8`](https://github.com/martypenner/css-to-go/commit/b90f9b81260a257830195639d0486b5ed477cefe))

## 0.0.5

### Patch Changes

- Docs: fix incorrect image path ([`6fc2e1b`](https://github.com/martypenner/css-to-go/commit/6fc2e1bd55b7eb47c7c7028183b0d7a43a93e3f4))

- Feat: allow removing files from the autocomplete list ([`212d087`](https://github.com/martypenner/css-to-go/commit/212d087c20443818cd9819f61f86df1a1878b658))

## 0.0.4

### Patch Changes

- Docs: correct broken image ([`f64aaff`](https://github.com/martypenner/css-to-go/commit/f64aaffc9d6e2640902dc3122ec21d5ab77874c3))

## 0.0.3

### Patch Changes

- Docs: tidy up readme ([`eab9ae7`](https://github.com/martypenner/css-to-go/commit/eab9ae70ee7cd9afbe1f61cec98522716b1e4553))

## 0.0.2

### Patch Changes

- Fix: package the extension correctly

## 0.0.1

### Patch Changes

- Initial release

## Notes

All notable changes to the "css-to-go" extension will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.
