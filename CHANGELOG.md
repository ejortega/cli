# Changelog

All notable changes are documented in this file.
The sections should follow the order `Packaging`, `Added`, `Changed`, `Fixed` and `Removed`.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [5.0.1] - 2023-04-20

### Fixed
- Handle null job labels in `phylum history --project`

## [5.0.0] - 2023-04-13

### Added
- Add extension changelog by @cd-work (#1019)
- Add base option to `phylum analyze` by @cd-work (#1008)

### Changed
- Switch to policy endpoint for job results by @cd-work (#1006)
- Reformat `phylum history` output by @kylewillmon (#1010)

### Fixed
- Allow `phylum pip install -e .` on macOS by @kylewillmon (#1017)
- Skip analysis with empty package list by @cd-work (#1007)

### Removed
- Remove `phylum project set-thresholds` subcommand by @cd-work (#1004)
- Remove request type from global config by @kylewillmon (#1001)

## [4.8.0] - 2023-04-04

### Added
- Add SPDX SBOM parser by @ejortega (#963)
- Add `pip` extension as official extension by @kylewillmon (#980)
- Use recursive lockfile search for `phylum init` by @cd-work (#979)

### Fixed
- Use `--dry-run` output for `poetry` extension by @cd-work (#957)
- Switch default subcommand from `list` to `help` by @cd-work (#959)
- Fix inconsistent `phylum init` whitespace by @cd-work (#964)
- Remove `--force` option from `phylum analyze` by @kylewillmon (#966)
- Move extension API source to extension directory by @cd-work (#969)
- Improve sandboxed process failure message by @cd-work (#972)
- Allow calling `phylum` from subdirectories by @matt-phylum (#974)
- Improve lockfile parsing errors by @cd-work (#992)
- Fix `phylum project link` overwriting project file by @cd-work (#995)
- Add SPDX tag:value parser by @ejortega (#978)
- Make `phylum package` type argument mandatory by @cd-work (#997)

## [4.7.0] - 2023-02-27

### Added
- Add automatic lockfile detection by @cd-work (#950)

### Fixed
- Fix project history endpoint by @cd-work (#947)

## [4.6.1] - 2023-02-14

### Fixed
- Fix Go parser ignoring dependencies by @cd-work (#944)

## [4.6.0] - 2023-02-03

### Added
- Improve `phylum init` UX by @cd-work (#936)

## [4.5.0] - 2023-02-01

### Added
- Add multi-lockfile ecosystems to analysis summary by @cd-work (#925)
- Add option to specify multiple lockfiles on CLI by @cd-work (#927)

### Fixed
- Fix poetry extension by @cd-work (#926)
- Fix install with poetry extension by @cd-work (#930)

### Upgrading
There are no breaking changes in this release. Projects may like to take
advantage of the new `.phylum_project` file format which accounts for multiple
lockfiles. To do so, simply run the `phylum init` command from the root of the
project directory. As long as the project and group names used are the same as
before, the existing project ID will be re-linked.

## [4.4.0] - 2023-01-20

### Added
- Add `phylum group delete` subcommand by @cd-work (#916)
- Add multi-lockfile support to `phylum init` by @cd-work (#910)

### Fixed
- Abort on unknown extension subcommands by @cd-work (#915)
- Fix gem parser for dependencies without version by @cd-work (#919)

## [4.3.0] - 2023-01-17

### Added
- Add multi-lockfile support to `.phylum_project` by @kylewillmon (#902)
- Make config file write atomic by @cd-work (#892)

### Fixed
- Fix sandbox executable path resolution by @cd-work (#905)

## [4.2.0] - 2023-01-05

### Added
- Submit single package with `phylum package` by @kylewillmon (#880)

### Fixed
- Fix parser lockfile consistency by @cd-work (#882)
- Add deno.window lib reference to extension\_api.ts by @kylewillmon (#890)

## [4.1.0] - 2022-12-20

### Added
- Add `phylum group transfer` subcommand by @cd-work (#833)
- Add extension helpers for direct API requests by @cd-work (#868)
- Add `--reauth` flag to `phylum auth login` by @kylewillmon (#879)

### Fixed
- Fix subdir analysis without lockfile parameter by @cd-work (#845)
- Add possible values to `phylum init -t` by @cd-work (#849)
- Reorder project initialization by @cd-work (#848)
- Ignore parent directory projects for `phylum init` by @cd-work (#840)
- Skip backup for non-intercepted ecosystem commands by @cd-work (#859)
- Traverse directories to find ecosystem root by @cd-work (#861)
- Restore files on ecosystem extension API failure by @cd-work (#866)
- Fix group prompt during `phylum init` by @cd-work (#869)
- Don't warn about config search if we didn't recurse by @kylewillmon (#870)

## [4.0.1] - 2022-11-30

### Fixed
- Downgrade linux builder to 20.04 by @kylewillmon (#835)

## [4.0.0] - 2022-11-30

### Added
- Add poetry lockfile v2 support by @cd-work (#780)
- `phylum auth set-token` by @kylewillmon (#786)
- Add `--lockfile-type` option to `phylum analyze` by @cd-work (#798)
- Add `phylum init` subcommand by @cd-work (#801)
- Add lockfile path and type to .phylum\_project by @cd-work (#806)
- Add `unsandboxed_run` manifest permission by @cd-work (#777)
- Add group member management subcommands by @cd-work (#809)

### Fixed
- Add ignore scripts when updating package-lock.json by @louislang (#791)
- Require "selfmanage" feature flag for `phylum update` by @kylewillmon (#797)
- Remove $PATH exception for `run` permission by @cd-work (#784)
- Clarify connection between read and run permissions by @kylewillmon (#802)
- Fix `phylum batch` command by @kylewillmon (#813)
- Remove minisign artifacts by @kylewillmon (#815)
- Fix regressions in #816 by @kylewillmon (#817)
- Fix package-lock parsing with 3rd-party registries by @cd-work (#828)

## [3.12.1] - 2022-10-28

### Fixed
- Avoid stdout when run with `--json` by @maxrake (#787)

## [3.12.0] - 2022-10-27

### Added
- Permissions extensions API by @andreaphylum (#767)

### Fixed
- Fix environment variable permission prompting by @cd-work (#766)
- Add default sandbox exception for $PATH by @cd-work (#772)
- Fix --package-type option by @kylewillmon (#774)
- Improve strictness of Gradle parser by @cd-work (#771)
- Avoid stdout when run with `--json` by @kylewillmon (#773)
- Re-execute phylum for sandboxing extensions by @cd-work (#765)
- Added ignore certs flag by @andreaphylum (#779)
- Clean up options by @maxrake (#768)

## [3.11.0] - 2022-10-19

### Added
- Add sandbox to extensions API by @cd-work (#673)
- Allow upgrade in phylum extension install by @kylewillmon (#693)
- Include pre-installed extensions by @kylewillmon (#702)
- Add CLI flags for log level control by @cd-work (#731)
- Create project extensions API by @andreaphylum (#709)
- Sign archives with openssl by @kylewillmon (#724)
- Add support for parsing golang lockfiles by @ein-tier (#720)
- Add support for parsing cargo lockfiles by @JosephPhylum (#743)

### Fixed
- Fix local yarn filesystem dependencies by @cd-work (#691)
- Add `./` prefix to extension install suggestions by @cd-work (#713)
- Add extension description to help output by @cd-work (#730)
- Improve extension subcommand conflict resolution by @cd-work (#740)
- Fix NPM dependency bundling by @cd-work (#750)
- Fix verbosity errors by @cd-work (#749)
- Improve `phylum history` UUID error message by @cd-work (#753)
- Fix CLI certificate override modifying config by @cd-work (#747)

## [3.10.1] - 2022-10-13

### Fixed
- Fix NPM dependency bundling by @cd-work (#752)

## [3.10.0] - 2022-09-16

### Fixed
- Handle `legacy` poetry source type by @louislang (#681)
- Fix extension name regex by @cd-work (#684)

### Added
- Send an appropriate User-Agent header by @kylewillmon (#666)

### Deprecated
- Remove XDG migration code by @kylewillmon (#677)

## [3.9.1] - 2022-08-31

### Fixed
- NPM and Yarn extensions do not properly exit on threshold violation by @cd-work (#660)
- Duplicate dependencies in `package-lock.json` aren't handled properly by @cd-work (#661)

## [3.9.0] - 2022-08-29

### Added
- Add support for native certificate store by @cd-work (#652)
- Add project extension APIs by @cd-work (#647)

### Fixed
- Update shim for musl to gnu is broken by @maxrake (#650)

## [3.8.0] - 2022-08-22

### Added
- CLI Extensions by @cd-work @kylewillmon and @andreaphylum

### Fixed
- Restore error trace output by @kylewillmon (#595)
- Use POST for job submission instead of PUT by @kylewillmon (#533)
- Switch to new project thresholds endpoint by @cd-work (#626)

## [3.7.4] - 2022-08-17

### Fixed

- Fix PHYLUM\_API\_KEY overwriting config token by @cd-work in #631
- Fix parsing gradle lockfile without classpath by @cd-work in #627
- Fix link dependencies in yarn parser by @cd-work in #621

### Added

- Add git dependency support to package-lock.json by @cd-work in #623

## [3.7.3] - 2022-08-09

### Fixed

- Fix `phylum update` zip decompression errors by @cd-work (#613)

## [3.7.2] - 2022-08-03

### Fixed
- Remove warnings from generic lockfile parser by @cd-work (#558)
- Remove deprecated `phylum history project` by @cd-work (#563)
- Refactor CLI output formatting by @cd-work (#564)
- Ignore empty refresh token from environment by @matt-phylum (#584)
- Better error messages by @kylewillmon (#588)

## [3.7.1] - 2022-07-14

### Fixed
- Support effective-pom files with site information by @ejortega (#550)
- Fix CI release readme release process by @cd-work (#553)

## [3.7.0] - 2022-07-13

### Added
- Add support for effective-pom.xml workspaces by @cd-work (#493)
- Add `phylum project delete` command by @kylewillmon (#527)
- Add aarch64-unknown-linux-musl builds to release by @kylewillmon (#528)

### Fixed
- Add detailed messages for HTTP conflicts by @cd-work (#491)
- Show a spinner while waiting for API by @samtay (#476)
- Don't require Job ID for `phylum history` command by @kylewillmon (#525)
- Remove user ID from analysis output by @cd-work (#545)

## [3.6.0] - 2022-06-20

### Added
- Add support for `gradle.lockfile` by @cd-work (#405)
- Add CONTRIBUTING.md documentation by @cd-work (#436)

### Fixed
- Fix stack overflow on Windows by @cd-work (#425)
- Fix error when parsing otherArchives pom.xml field by @cd-work (#458)
- Added build script as workaround for Window debug builds by @andreaphylum (#462)
- Fix messed up spinner output by @samtay (#464)
- Fix SHELL env var assumed to exist during install by @maxrake (#471)

## [3.5.0] - 2022-05-23

### Added
- Use new API endpoint for OIDC redirect by @cd-work (#399)
- Emit unique exit code when failing thresholds by @cd-work (#406)

### Fixed
- Ignore certs everywhere when requested by @kylewillmon (#389)
- Remove Web UI link from analyze output by @cd-work (#397)
- Don't use streaming parsers by @kylewillmon (#401)
- Bump phylum\_types version by @kylewillmon (#409)

## [3.4.0] - 2022-05-19

### Added
- Add group support by @cd-work (#381)

### Fixed
- Fix yarn v1 parser with quoted version key by @cd-work (#383)
- Use new format for package analysis endpoint by @cd-work (#384)

## [3.3.0] - 2022-05-16

### Added
- Create `phylum parse` command by @kylewillmon (#362)
- Improve handling of HTTP JSON error responses by @cd-work (#365)
- Improve error messages with HTTP failures by @cd-work (#358)

### Fixed
- Fix non-frozen Pipfile suffix by @cd-work (#366)
- Use new endpoint for ping  by @kylewillmon (#369)

## [3.2.0] - 2022-05-06

### Added
- Add support for patched deps in yarn lockfile by @cd-work (#343)
- Add support for http(s) and ssh resolvers in yarn lockfiles by @cd-work (#345)
- Add explicit option to disable thresholds from CLI by @cd-work (#329)

### Fixed
- Don't panic in the javascript lockfile parser by @kylewillmon (#340)
- Use better error for missing lockfiles by @cd-work (#352)

## [3.1.0] - 2022-04-29

### Added
- Add `--bearer` parameter to `phylum auth token` by @cd-work (#320)

### Fixed
- Resolve project create errors by @kylewillmon (#332)

## [3.0.0] - 2022-04-28

### BREAKING CHANGES
- Follow XDG directories spec by @cd-work (#251)
  * Existing installs will have config file moved automatically

### Added
- Add `uninstall` subcommand to phylum by @cd-work (#239)
- Add `--project` parameter to `phylum analyze` by @cd-work (#280)
- Improve tab completion in ZSH for file path arguments by @kylewillmon (#300)

### Fixed
- Create app directories with mode 700 by default by @cd-work (#289)
- Remove header from `phylum history --json` output by @cd-work (#290)
- Fix formatting of `phylum history` project scores by @cd-work (#297)
- Add newline to shell rc files before Phylum entries by @cd-work (#291)
- Filter non-PyPI dependencies from poetry lockfile by @cd-work (#273)

### Deprecated
- Hide the `--prerelease` arg in `phylum update` by @kylewillmon (#302)
- Deprecate `phylum history project` by @cd-work (#290)
- Remove PyO3 bindings by @eeclfrei (#295)

## [2.2.0] - 2022-04-21

### Added
- Add yarn v2 lockfile support by @cd-work (#247)
- Parse package extras in Python requirements.txt files by @kylewillmon (#271)
- Rename projects subcommand to project by @kylewillmon (#282)
- Improved scripting support
  * Remove checkmark from `auth token` command by @cd-work (#261)
  * Set appropriate exit codes on failure by @cd-work (#260)

### Fixed
- Format "Last updated" field with ISO 8601 by @cd-work (#257)
- Truncate excessive project names by @cd-work (#262)
- Remove table header from projects list json by @cd-work (#264)
- Document the name argument for projects subcommand by @kylewillmon (#283)

## [2.1.0] - 2022-04-13

### What's Changed
- Continue install/upgrade even if quarantine flag isn't found by @kylewillmon (#249)
- Replace Language/Type with Ecosystem by @cd-work (#248)
- Use git\_version for version numbers by @kylewillmon (#243)
- Use Ecosystem in `phylum package` output by @cd-work (#255)
- Add support for new npm package-lock format by @cd-work (#242)

## [2.0.1] - 2022-04-12

### What's Changed
- Create phylum auth token command by @mdx97 (#217)
- Add Python poetry.lock support by @cd-work (#238)

## [2.0.0] - 2022-04-11

### What's Changed
- Add maven support by @ejortega (#178)
- Fix pypi parsing by @ejortega (#182)
- Standardize package type names / add nuget package type by @eeclfrei (#181)
- Add lockfile parsing for C# by @eeclfrei (#189)
- Allow binary to be run without config file by @kylewillmon (#196)
- Restrict settings.yaml file permissions by @kylewillmon (#219)
- Add email to `phylum auth status` by @cd-work (#227)
- Fix cryptic errors with invalid auth token by @cd-work (#233)
- Migrate install script to POSIX sh by @cd-work (#235)

## [1.2.0] - 2022-01-22

### What's Changed
- Bring Oauth Support to CLI by @DanielJoyce (#118)
- Better error handling by @DanielJoyce (#145)
- Swap out static\_init module for lazy\_static by @DanielJoyce (#146)
- Gather files from static builder by @louislang (#147)
- Adding release script by @eeclfrei (#150)
- Updates for recent api changes by @eeclfrei (#160)
- Update sha2 crate due to RUSTSEC-2021-0100 by @ejortega (#161)
- Adding m1/arm build by @eeclfrei (#162)
- Include the error message associated with an http error by @eeclfrei (#163)
- Readme update for v1.2.0 by @furi0us333 (#164)
- Update install script to support m1/arm by @eeclfrei (#165)
- Bump version v1.2.0 by @louislang (#168)

## 1.1.5 - 2021-12-06
- Option to ignore cert check; various bugfixes

## 1.1.4 - 2021-11-02

## 1.1.3 - 2021-10-22
- Add issues filtering; display / error codes cleanup

## 1.1.2 - 2021-09-15
- Bugfix for deserialization issue
- Tab completion support for zsh and fish
- Support for tmpfs

## 1.1.1 - 2021-09-03
- Updates to signature verification

## 1.1.0 - 2021-09-01
- Add support for submitting Python packages; signature verification on upgrade

## 1.0.2 - 2021-08-23

## 1.0.1 - 2021-08-23
- Add support for automatically building macOS release

## 1.0.0 - 2021-08-02
- Add formatted output; refactor subcommands; many other changes for improved usability

## 0.0.7 
- Adding synch submit requests

## 0.0.5 
- Add support for projects and project labels / decrease verbosity of package status

## 0.0.4 
- Minor update to API response format; add `--threshold` argument to `status` command

## 0.0.3 
- Update response format of the `status` command to match API changes.

## 0.0.2 
- Add support for listing / submitting heuristics.

## 0.0.1 
- Initial release.

[unreleased]: https://github.com/phylum-dev/cli/compare/v5.0.1...HEAD
[5.0.1]: https://github.com/phylum-dev/cli/compare/v5.0.0...v5.0.1
[5.0.0]: https://github.com/phylum-dev/cli/compare/v4.8.0...v5.0.0
[4.8.0]: https://github.com/phylum-dev/cli/compare/v4.7.0...v4.8.0
[4.7.0]: https://github.com/phylum-dev/cli/compare/v4.6.1...v4.7.0
[4.6.1]: https://github.com/phylum-dev/cli/compare/v4.6.0...v4.6.1
[4.6.0]: https://github.com/phylum-dev/cli/compare/v4.5.0...v4.6.0
[4.5.0]: https://github.com/phylum-dev/cli/compare/v4.4.0...v4.5.0
[4.4.0]: https://github.com/phylum-dev/cli/compare/v4.3.0...v4.4.0
[4.3.0]: https://github.com/phylum-dev/cli/compare/v4.2.0...v4.3.0
[4.2.0]: https://github.com/phylum-dev/cli/compare/v4.1.0...v4.2.0
[4.1.0]: https://github.com/phylum-dev/cli/compare/v4.0.1...v4.1.0
[4.0.1]: https://github.com/phylum-dev/cli/compare/v4.0.0...v4.0.1
[4.0.0]: https://github.com/phylum-dev/cli/compare/v3.12.1...v4.0.0
[3.12.1]: https://github.com/phylum-dev/cli/compare/v3.12.0...v3.12.1
[3.12.0]: https://github.com/phylum-dev/cli/compare/v3.11.0...v3.12.0
[3.11.0]: https://github.com/phylum-dev/cli/compare/v3.10.0...v3.11.0
[3.10.1]: https://github.com/phylum-dev/cli/compare/v3.10.0...v3.10.1
[3.10.0]: https://github.com/phylum-dev/cli/compare/v3.9.1...v3.10.0
[3.9.1]: https://github.com/phylum-dev/cli/compare/v3.9.0...v3.9.1
[3.9.0]: https://github.com/phylum-dev/cli/compare/v3.8.0...v3.9.0
[3.8.0]: https://github.com/phylum-dev/cli/compare/v3.7.2...v3.8.0
[3.7.4]: https://github.com/phylum-dev/cli/compare/v3.7.3...v3.7.4
[3.7.3]: https://github.com/phylum-dev/cli/compare/v3.7.2...v3.7.3
[3.7.2]: https://github.com/phylum-dev/cli/compare/v3.7.1...v3.7.2
[3.7.1]: https://github.com/phylum-dev/cli/compare/v3.7.0...v3.7.1
[3.7.0]: https://github.com/phylum-dev/cli/compare/v3.6.0...v3.7.0
[3.6.0]: https://github.com/phylum-dev/cli/compare/v3.5.0...v3.6.0
[3.5.0]: https://github.com/phylum-dev/cli/compare/v3.4.0...v3.5.0
[3.4.0]: https://github.com/phylum-dev/cli/compare/v3.3.0...v3.4.0
[3.3.0]: https://github.com/phylum-dev/cli/compare/v3.2.0...v3.3.0
[3.2.0]: https://github.com/phylum-dev/cli/compare/v3.1.0...v3.2.0
[3.1.0]: https://github.com/phylum-dev/cli/compare/v3.0.0...v3.1.0
[3.0.0]: https://github.com/phylum-dev/cli/compare/v2.2.0...v3.0.0
[2.2.0]: https://github.com/phylum-dev/cli/compare/v2.1.0...v2.2.0
[2.1.0]: https://github.com/phylum-dev/cli/compare/v2.0.1...v2.1.0
[2.0.1]: https://github.com/phylum-dev/cli/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/phylum-dev/cli/compare/v1.2.0...v2.0.0
[1.2.0]: https://github.com/phylum-dev/cli/compare/v1.1.4...v1.2.0