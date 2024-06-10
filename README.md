# Decision Maker - Rust & Slint

[![Rust CI](https://github.com/quattervals/decision_maker/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/quattervals/decision_maker/actions/workflows/rust.yml)

## About

Often, the discussion about a decision revolves around things that are not actually important. This tool helps to find the parameters which are really important. For each pair of parameters, the user makes a binary decision to determine which one is more important.


For example if you want to buy a bicycle you may have these parameters
- price
- color
- weight
- quality
- descent
- ascent
The tool presents you every pair (e.g. descent vs. color) on which you must say which one is more important. After all comparisons, you are presented with a ranking.


Knowing these weights helps you to fill in a [weighted decision matrix](https://en.wikipedia.org/wiki/Decision-matrix_method).

## FAQ
- *Why only binary decisions between parameters?*<br>
  Inherently, decisions are "either or". You have to make a decision between parameters and not haggle about sub-percent weights. Remember, there are $\frac{N(N-1)}{2}$ decisions to make, where $N$ is the number of parameters you enter. And yes, binary is easier to implement.
- *Why not for macos?*<br>
  Because I can't be bothered
- *Will there be an Android version?*<br>
  Yes, there might be.
- *Why slint and rust?*<br>
  I want to learn about slint and rust


## Todo

### Deployment

- Installer for Windows, Linux
- Build for Android
- Multilingual using `@tr()` [doc](https://releases.slint.dev/1.6.0/docs/slint/src/language/concepts/translations)
- random questioning as checkbox?
- add sanitizers

### Business logic
- move data types to business logic so that it is possible to test
- show consolidated only when there are appended elements
- no console output in release
- no results shown when there is no game
- save/load from file
- option to make no decision between pairs.

