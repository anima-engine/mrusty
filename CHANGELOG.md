# Change Log

## [v1.0.0](https://github.com/anima-engine/mrusty/tree/v1.0.0) (2016-12-19)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.5.1...v1.0.0)

**Implemented enhancements:**

- Don't require glob import to use mrfn! et al [\#92](https://github.com/anima-engine/mrusty/issues/92)
- Call Rust functions with Ruby blocks [\#89](https://github.com/anima-engine/mrusty/issues/89)
- Avoid glob imports from top-level of crate [\#85](https://github.com/anima-engine/mrusty/issues/85)
- Remove borrow rules from mrfn. [\#83](https://github.com/anima-engine/mrusty/issues/83)

**Fixed bugs:**

- mrbc\_context is leaked. [\#94](https://github.com/anima-engine/mrusty/issues/94)
- Nested mruby Rust calls cause memory leaks in the case of uncaught exceptions. [\#90](https://github.com/anima-engine/mrusty/issues/90)
- Change no-run to no\_run in docs. [\#87](https://github.com/anima-engine/mrusty/issues/87)

**Closed issues:**

- "error: no rules expected the token ..." when using mrfn! with Value parameters. [\#93](https://github.com/anima-engine/mrusty/issues/93)
- Is there a way to put class macro definitions in separate modules? [\#88](https://github.com/anima-engine/mrusty/issues/88)

**Merged pull requests:**

- Sanitize public macros [\#95](https://github.com/anima-engine/mrusty/pull/95) ([AndyBarron](https://github.com/AndyBarron))
- Added mruby blocks to methods. [\#91](https://github.com/anima-engine/mrusty/pull/91) ([dragostis](https://github.com/dragostis))

## [v0.5.1](https://github.com/anima-engine/mrusty/tree/v0.5.1) (2016-04-29)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.5.0...v0.5.1)

## [v0.5.0](https://github.com/anima-engine/mrusty/tree/v0.5.0) (2016-04-29)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.4.3...v0.5.0)

**Implemented enhancements:**

- Improve macros to handle &mut T case. [\#82](https://github.com/anima-engine/mrusty/issues/82)

## [v0.4.3](https://github.com/anima-engine/mrusty/tree/v0.4.3) (2016-04-26)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.4.2...v0.4.3)

## [v0.4.2](https://github.com/anima-engine/mrusty/tree/v0.4.2) (2016-04-26)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.4.1...v0.4.2)

## [v0.4.1](https://github.com/anima-engine/mrusty/tree/v0.4.1) (2016-04-20)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.4.0...v0.4.1)

## [v0.4.0](https://github.com/anima-engine/mrusty/tree/v0.4.0) (2016-04-20)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.3.2...v0.4.0)

**Implemented enhancements:**

- Add Class to mrfn. [\#80](https://github.com/anima-engine/mrusty/issues/80)
- Add missing Value::to\_module. [\#79](https://github.com/anima-engine/mrusty/issues/79)
- Expand macro functionality. [\#77](https://github.com/anima-engine/mrusty/issues/77)
- Find a way to differentiate classes that need Rust reflection and those that don't. [\#76](https://github.com/anima-engine/mrusty/issues/76)
- Clean mruby API calls. Some are probably unused. [\#75](https://github.com/anima-engine/mrusty/issues/75)
- Implement ability to define methods on Ruby classes. [\#73](https://github.com/anima-engine/mrusty/issues/73)
- Support Ruby file loading, class extraction and method call [\#70](https://github.com/anima-engine/mrusty/issues/70)

**Fixed bugs:**

- require should only return true/false, not result of execute\(\). [\#71](https://github.com/anima-engine/mrusty/issues/71)

**Merged pull requests:**

- Implemented Rust-less class and method definition. [\#78](https://github.com/anima-engine/mrusty/pull/78) ([dragostis](https://github.com/dragostis))
- Added Class & Module structs and handling. Fixes \#70. [\#74](https://github.com/anima-engine/mrusty/pull/74) ([dragostis](https://github.com/dragostis))
- small doc fixes for Repl [\#72](https://github.com/anima-engine/mrusty/pull/72) ([steveklabnik](https://github.com/steveklabnik))

## [v0.3.2](https://github.com/anima-engine/mrusty/tree/v0.3.2) (2016-04-07)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.3.1...v0.3.2)

**Implemented enhancements:**

- Fix error scheme. [\#62](https://github.com/anima-engine/mrusty/issues/62)

**Fixed bugs:**

- Fix coverage. [\#69](https://github.com/anima-engine/mrusty/issues/69)

## [v0.3.1](https://github.com/anima-engine/mrusty/tree/v0.3.1) (2016-03-26)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.3.0...v0.3.1)

**Implemented enhancements:**

- Update doc examples to use mrclass. [\#67](https://github.com/anima-engine/mrusty/issues/67)
- Added more examples and use cases. [\#66](https://github.com/anima-engine/mrusty/issues/66)
- Add mruby.sym\(\) and make sure value.to\_str\(\) works with Symbols. [\#64](https://github.com/anima-engine/mrusty/issues/64)

**Fixed bugs:**

- args in initialize should be |; args|, not |args|. [\#65](https://github.com/anima-engine/mrusty/issues/65)

**Merged pull requests:**

- Added mruby build script. [\#68](https://github.com/anima-engine/mrusty/pull/68) ([dragostis](https://github.com/dragostis))

## [v0.3.0](https://github.com/anima-engine/mrusty/tree/v0.3.0) (2016-03-21)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.2.8...v0.3.0)

**Implemented enhancements:**

- Rename MR\* to Mr\*. [\#60](https://github.com/anima-engine/mrusty/issues/60)

**Fixed bugs:**

- Panics end process with 4 SIGILL when panic is called from mruby. [\#58](https://github.com/anima-engine/mrusty/issues/58)

## [v0.2.8](https://github.com/anima-engine/mrusty/tree/v0.2.8) (2016-03-19)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.2.7...v0.2.8)

## [v0.2.7](https://github.com/anima-engine/mrusty/tree/v0.2.7) (2016-03-18)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.2.6...v0.2.7)

**Implemented enhancements:**

- Add args: Args option to match all arguments in mrfn. [\#57](https://github.com/anima-engine/mrusty/issues/57)
- Implement Value.type\(\) with Rust's Any to aid fast type pattern matching. [\#55](https://github.com/anima-engine/mrusty/issues/55)

## [v0.2.6](https://github.com/anima-engine/mrusty/tree/v0.2.6) (2016-03-15)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.2.5...v0.2.6)

## [v0.2.5](https://github.com/anima-engine/mrusty/tree/v0.2.5) (2016-03-15)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.2.4...v0.2.5)

**Implemented enhancements:**

- Add Exception type argument to raise\(\) and make it return nil. [\#54](https://github.com/anima-engine/mrusty/issues/54)
- Add run\_unchecked. [\#53](https://github.com/anima-engine/mrusty/issues/53)

## [v0.2.4](https://github.com/anima-engine/mrusty/tree/v0.2.4) (2016-03-15)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.2.3...v0.2.4)

## [v0.2.3](https://github.com/anima-engine/mrusty/tree/v0.2.3) (2016-03-12)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.2.2...v0.2.3)

**Implemented enhancements:**

- Repl should probably be abstracted to work with any kind of readline. \(a trait would be nice\) [\#49](https://github.com/anima-engine/mrusty/issues/49)

**Merged pull requests:**

- Abstract repl. Fixes \#49. [\#50](https://github.com/anima-engine/mrusty/pull/50) ([dragostis](https://github.com/dragostis))

## [v0.2.2](https://github.com/anima-engine/mrusty/tree/v0.2.2) (2016-03-10)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.2.1...v0.2.2)

**Implemented enhancements:**

- Fix mruby build documentation in README.md and lib.rs. [\#48](https://github.com/anima-engine/mrusty/issues/48)
- Add runnable example. [\#47](https://github.com/anima-engine/mrusty/issues/47)

## [v0.2.1](https://github.com/anima-engine/mrusty/tree/v0.2.1) (2016-03-04)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.2.0...v0.2.1)

## [v0.2.0](https://github.com/anima-engine/mrusty/tree/v0.2.0) (2016-03-03)
[Full Changelog](https://github.com/anima-engine/mrusty/compare/v0.1.1...v0.2.0)

**Implemented enhancements:**

- Rename project in all headers. [\#46](https://github.com/anima-engine/mrusty/issues/46)
- Try to replace null ending `C` string functions with known-size counterparts. [\#45](https://github.com/anima-engine/mrusty/issues/45)
- Add RSpec-style specification tests. [\#43](https://github.com/anima-engine/mrusty/issues/43)
- Host documentation. [\#25](https://github.com/anima-engine/mrusty/issues/25)

**Merged pull requests:**

- Added basic spec testing. Fixes \#43. [\#44](https://github.com/anima-engine/mrusty/pull/44) ([dragostis](https://github.com/dragostis))

## [v0.1.1](https://github.com/anima-engine/mrusty/tree/v0.1.1) (2016-02-27)
**Implemented enhancements:**

- Inline C functions. [\#41](https://github.com/anima-engine/mrusty/issues/41)
- Add Vec & Option to mrfn!. [\#38](https://github.com/anima-engine/mrusty/issues/38)
- Add proper errors. [\#36](https://github.com/anima-engine/mrusty/issues/36)
- Add Repl rename. [\#33](https://github.com/anima-engine/mrusty/issues/33)
- Implement .mrb execution. [\#31](https://github.com/anima-engine/mrusty/issues/31)
- Implement require. [\#30](https://github.com/anima-engine/mrusty/issues/30)
- Find a way to connect Result with mruby Exception. [\#27](https://github.com/anima-engine/mrusty/issues/27)
- Add Repl struct. [\#23](https://github.com/anima-engine/mrusty/issues/23)
- Add Data types to mrfn. [\#21](https://github.com/anima-engine/mrusty/issues/21)
- \#\[inline\] casts. [\#20](https://github.com/anima-engine/mrusty/issues/20)
- Complete README.md. [\#18](https://github.com/anima-engine/mrusty/issues/18)
- Improve documentation & talk about the little things. [\#17](https://github.com/anima-engine/mrusty/issues/17)
- Remove extra API. [\#16](https://github.com/anima-engine/mrusty/issues/16)
- Add more integration tests. [\#15](https://github.com/anima-engine/mrusty/issues/15)
- Add mruby build instructions to README.md. [\#10](https://github.com/anima-engine/mrusty/issues/10)
- Add an OSX build in Travis. [\#9](https://github.com/anima-engine/mrusty/issues/9)
- Consider using travis-cargo for travis ci. [\#7](https://github.com/anima-engine/mrusty/issues/7)
- Do a little code cleanup & refactoring. [\#5](https://github.com/anima-engine/mrusty/issues/5)

**Fixed bugs:**

- Wrap mruby macros in C function for mrb\_value and keep MRValue opaque. [\#40](https://github.com/anima-engine/mrusty/issues/40)
- Rename all unused closure variables in docs. [\#34](https://github.com/anima-engine/mrusty/issues/34)
- call should return Result\<Value\>, not Value. [\#28](https://github.com/anima-engine/mrusty/issues/28)
- `to\_obj::\<T\>` is unsafe. [\#22](https://github.com/anima-engine/mrusty/issues/22)
- Coveralls does not see mruby.rs for some reason. [\#12](https://github.com/anima-engine/mrusty/issues/12)
- Sources are not visible in Coveralls. [\#11](https://github.com/anima-engine/mrusty/issues/11)

**Merged pull requests:**

- Integration tests. Fixes \#15. [\#39](https://github.com/anima-engine/mrusty/pull/39) ([dragostis](https://github.com/dragostis))
- Moved scripts to travis folder. Fixes \#12. [\#37](https://github.com/anima-engine/mrusty/pull/37) ([krodyrobi](https://github.com/krodyrobi))
- Added require. [\#35](https://github.com/anima-engine/mrusty/pull/35) ([dragostis](https://github.com/dragostis))
- Added Repl. Fixes \#23 & \#28. [\#29](https://github.com/anima-engine/mrusty/pull/29) ([dragostis](https://github.com/dragostis))
- Coverage paths revised, closes \#11. [\#13](https://github.com/anima-engine/mrusty/pull/13) ([krodyrobi](https://github.com/krodyrobi))
- Code coverage with coveralls.io [\#6](https://github.com/anima-engine/mrusty/pull/6) ([krodyrobi](https://github.com/krodyrobi))
- Added a safe wrapper for mruby. [\#4](https://github.com/anima-engine/mrusty/pull/4) ([dragostis](https://github.com/dragostis))
- Added arrays. [\#3](https://github.com/anima-engine/mrusty/pull/3) ([dragostis](https://github.com/dragostis))
- Move tests to separate file [\#2](https://github.com/anima-engine/mrusty/pull/2) ([krodyrobi](https://github.com/krodyrobi))
- Fixed Travis build. [\#1](https://github.com/anima-engine/mrusty/pull/1) ([dragostis](https://github.com/dragostis))



\* *This Change Log was automatically generated by [github_changelog_generator](https://github.com/skywinder/Github-Changelog-Generator)*