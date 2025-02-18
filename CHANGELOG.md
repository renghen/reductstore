# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- reductstore: Cargo feature `web-console` to build without Web Console, [PR-365](https://github.com/reductstore/reductstore/pull/365)
- reductstore: Web Console v1.4.0

### Fixed

- reductstore: Stop downloading Web Console at docs.rs build, [PR-366](https://github.com/reductstore/reductstore/pull/366)

## [1.7.0] - 2023-10-06

### Added

- reduct-cli: `alias` and `server` commands, [PR-343](https://github.com/reductstore/reductstore/pull/343)
- reduct-rs: `ReductClient.url`, `ReductClient.token`, `ReductCientBuilder.try_build` [PR-350](https://github.com/reductstore/reductstore/pull/350)
- reductstore: `healthcheck` to buildx.Dockerfile, [PR-350](https://github.com/reductstore/reductstore/pull/350)
- reductstore: provisioning with environment variables, [PR-352](https://github.com/reductstore/reductstore/pull/352)
- reductstore,reduct-rs: batched write API, [PR-355](https://github.com/reductstore/reductstore/pull/355)

### Changed

- reductstore: Update dependencies, min. rust v1.67.0, [PR-341](https://github.com/reductstore/reductstore/pull/341)
- reductstore: Use Web Console v1.3.0, [PR-345](https://github.com/reductstore/reductstore/pull/342)
- reductstore: Move some Python API tests in Rust part, [PR-351](https://github.com/reductstore/reductstore/pull/351)
- reduct-base: Rename `HttpError` -> `ReductError`, [PR-350](https://github.com/reductstore/reductstore/pull/350)
- docs: update Getting Started, [PR-358](https://github.com/reductstore/reductstore/pull/358)

### Fixed

- reduct-rs: Normalize instance URL in `ClientBuilder.url`, [PR-343](https://github.com/reductstore/reductstore/pull/343)

# [1.6.2] - 2023-09-20

### Fixed

- reductstore: Panic for a bad time interval in `GET /b/:bucket/:entry/q`, [PR-357](https://github.com/reductstore/reductstore/pull/357)

## [1.6.1] - 2023-08-28

### Fixed

- reductstore: README and LICENSE in reductstore crate, [PR-347](https://github.com/reductstore/reductstore/pull/347)

### Security

- reductstore: Update `rustls` with
  patched `rustls-webpki`, [PR-349](https://github.com/reductstore/reductstore/pull/349)

## [1.6.0] - 2023-08-14

### Added

- reductstore: Build docker image for ARM32 platform, [PR-328](https://github.com/reductstore/reductstore/pull/328)
- reductstore,reduct-rs: Removing entries from bucket, [PR-334](https://github.com/reductstore/reductstore/pull/334)
- reductstore,reduct-rs: Limit parameter in query request, [PR-335](https://github.com/reductstore/reductstore/pull/335)
- reduct-rs: Server API for Client SDK, [PR-321](https://github.com/reductstore/reductstore/pull/321)
- reduct-rs: Token API for Client SDK, [PR-322](https://github.com/reductstore/reductstore/pull/322)
- reduct-rs: Bucket API for Client SDK, [PR-323](https://github.com/reductstore/reductstore/pull/323)
- reduct-rs: Entry API for Client SDK, [PR-326](https://github.com/reductstore/reductstore/pull/326)
- reduct-rs: Examples and docs, [PR-333](https://github.com/reductstore/reductstore/pull/333)

### Changed

- reductstore: Refactor `http_frontend` module, [PR-306](https://github.com/reductstore/reductstore/pull/306)
- reductstore: Cache last block to reduce read operations, [PR-318](https://github.com/reductstore/reductstore/pull/318)
- reductstore: Grained HTTP components, [PR-319](https://github.com/reductstore/reductstore/pull/319)
- reductstore: Default maximum records in block 256, [PR-320](https://github.com/reductstore/reductstore/pull/320)
- reductstore: BUSL-1.1 license, [PR-337](https://github.com/reductstore/reductstore/pull/337)
- reductstore: Update README.md, [PR-338](https://github.com/reductstore/reductstore/pull/338)
- all: Organize workspaces, [PR-310](https://github.com/reductstore/reductstore/pull/310)

### Removed

- reductstore: `native-tls` dependency (only `rustls`), [PR-315](https://github.com/reductstore/reductstore/pull/315)

### Fixed

- reductstore: Partial bucket settings, [PR-325](https://github.com/reductstore/reductstore/pull/325)

## [1.5.1] - 2023-07-17

### Fixed

- Handle empty or broken block descriptor, [PR-317](https://github.com/reductstore/reductstore/pull/317)

## [1.5.0] - 2023-06-30

### Added

- `x-reduct-api` header to get quick version number in Major.Minor
  format, [PR-291](https://github.com/reductstore/reductstore/pull/291)
- `GET /api/v1/:bucket/:entry/batch` endpoint to read a bunch of
  records, [PR-294](https://github.com/reductstore/reductstore/pull/294)
- `HEAD /api/v1/b/:bucket_name/:entry_name` and `HEAD /api/v1/b/:bucket_name/:entry_name/batch`
  endpoints, [PR-296]https://github.com/reductstore/reductstore/pull/296)

### Changed

- Concise format for headers in `GET /api/v1/:bucket/:entry/batch`
  response, [PR-298](https://github.com/reductstore/reductstore/pull/298)

## [1.4.1] - 2023-06-27

### Fixed

- Stuck empty entries, [PR-302](https://github.com/reductstore/reductstore/pull/302)

## [1.4.0] - 2023-06-09

### Fixed

- Panic when an invalid utf-8 received as a label value, [PR-290](https://github.com/reductstore/reductstore/pull/290)
- Writing record for clients which don't support for Expect
  header, [PR-293](https://github.com/reductstore/reductstore/pull/293)

## [1.4.0-beta.1] - 2023-06-03

### Changed

- Update web console to 1.2.2, [PR-287](https://github.com/reductstore/reductstore/pull/287)

### Fixed

- Parsing non-string quota type, [PR-286](https://github.com/reductstore/reductstore/pull/286)
- Show file paths in logs without registry path, [PR-288](https://github.com/reductstore/reductstore/pull/288)

## [1.4.0-alpha.3] - 2023-05-29

### Fixed

- Return `init-token` to token list, [PR-280](https://github.com/reductstore/reductstore/pull/280)
- First query ID is 1, [PR-281](https://github.com/reductstore/reductstore/pull/281)
- Showing permissions in `GET /api/v1/tokens`, [PR-282](https://github.com/reductstore/reductstore/pull/282)
- Remove removed bucket from token permissions, [PR-283](https://github.com/reductstore/reductstore/pull/283)
- Build on Windows and macOS, [PR-284](https://github.com/reductstore/reductstore/pull/284)

## [1.4.0-alpha.2] - 2023-05-26

### Fixed

- Cargo package, [PR-278](https://github.com/reductstore/reductstore/pull/278)

## [1.4.0-alpha.1] - 2023-05-22

### Added

- Continuous query `GET /api/v1/:bucket/:entry/q?continuous=true|false`,
  [PR-248](https://github.com/reductstore/reductstore/pull/248)
- Build ARM64 Docker image
- Integration of Rust, [PR-251](https://github.com/reductstore/reductstore/pull/251)
- Print build commit and date in logs, [PR-271](https://github.com/reductstore/reductstore/pull/271)
- Re-build ARM64 Docker image for Rust, [PR-274](https://github.com/reductstore/reductstore/pull/274)
- Publish crate to crates.io, [PR-275](https://github.com/reductstore/reductstore/pull/275)

### Changed

- New public Docker repository `reduct/store`, [PR-246](https://github.com/reductstore/reductstore/pull/246)
- Speed up loading entries at start, [PR-250](https://github.com/reductstore/reductstore/pull/250)
- Rewrite static asset management in Rust, [PR-252](https://github.com/reductstore/reductstore/pull/252)
- Rewrite token authentication module in Rust, [PR-255](https://github.com/reductstore/reductstore/pull/255)
- Rewrite storage module in Rust, [PR-257](https://github.com/reductstore/reductstore/pull/257)
- Rewrite HTTP layer in Rust, [PR-259](https://github.com/reductstore/reductstore/pull/259)

### Removed

- Disable Windows and Macos builds because of migration on
  Rust, [PR-251](https://github.com/reductstore/reductstore/pull/251)

### Fixed

- GET /api/v1/me endpoint for disabled authentication, [PR-245](https://github.com/reductstore/reductstore/pull/245)
- Error handling when a client closes a connection, [PR-263](https://github.com/reductstore/reductstore/pull/263)
- Allow null quota type in bucket settings, [PR-264](https://github.com/reductstore/reductstore/pull/264)
- Writing belated data, [PR-265](https://github.com/reductstore/reductstore/pull/265)
- Fix searching record by timestamp, [PR-266](https://github.com/reductstore/reductstore/pull/266)
- Graceful shutdown, [PR-267](https://github.com/reductstore/reductstore/pull/267)
- Access to a block descriptor from several threads, [PR-268](https://github.com/reductstore/reductstore/pull/268)
- Handling unix SIGTERM signal, [PR-269](https://github.com/reductstore/reductstore/pull/269)
- Encoding non-text assets of Web Console, [PR-270](https://github.com/reductstore/reductstore/pull/270)
- Pass hash commit into docker image, [PR-272](https://github.com/reductstore/reductstore/pull/272)
- Build snap package in CI, [PR-273](https://github.com/reductstore/reductstore/pull/273)

## [1.3.2] - 2023-03-10

### Added

- Build and publish snap, [PR-241](https://github.com/reductstore/reductstore/pull/241)

### Changed

- Fetch Web Console from cmake, [PR-239](https://github.com/reductstore/reductstore/pull/239)
- Install snap as a daemon, [PR-240](https://github.com/reductstore/reductstore/pull/240)

### Fixed

- Begin time 0 is valid for a block, [PR-242](https://github.com/reductstore/reductstore/pull/242)

## [1.3.1] - 2023-02-03

### Fixed

- Querying when a block doesn't have records for certain
  labels, [PR-235](https://github.com/reductstore/reductstore/pull/235)

## [1.3.0] - 2023-01-26

### Added

- Labels for  `POST|GET /api/v1/:bucket/:entry` as headers with
  prefix `x-reduct-label-`, [PR-224](https://github.com/reductstore/reductstore/pull/224)
- `include-<label>` and `exclude-<label>` query parameters for query endpoint
  `GET /api/v1/:bucket/:entry/q`, [PR-226](https://github.com/reductstore/reductstore/pull/226)
- Store the `Content-Type` header received for a record while writing it, so that the record may be returned with the
  same header, [PR-231](https://github.com/reductstore/reductstore/pull/231)

### Changed

- Project license AGPLv3 to MPL-2.0, [PR-221](https://github.com/reductstore/reductstore/pull/221)
- Rename error header `-x-reduct-error`
  to `x-reduct-error`, [PR-230](https://github.com/reductstore/reductstore/pull/230)
- Update Web Console to v1.2.0, [PR-232](https://github.com/reductstore/reductstore/pull/232)

## [1.2.3] - 2023-01-02

### Fixed

- Crashing when post request is aborted by client, [PR-223](https://github.com/reductstore/reductstore/pull/223)

## [1.2.2] - 2022-12-20

### Fixed

- Token validation for anonymous access, [PR-217](https://github.com/reductstore/reductstore/pull/217)

## [1.2.1] - 2022-12-19

### Fixed

- Docker image command

## [1.2.0] - 2022-12-18

### Added:

- `GET /api/v1/me` endpoint to get current
  permissions, [PR-202](https://github.com/reductstore/reductstore/pull/208)
- Send error message in `-x-reduct-error`header [PR-213](https://github.com/reductstore/reductstore/pull/213)

### Changed:

- Consistent token and bucket management, [PR-208](https://github.com/reductstore/reductstore/pull/208)
- Rebranding: update project name, CMakeTargets and docs, [PR-215](https://github.com/reductstore/reductstore/pull/215)

### Fixed:

- HTTP statuses for `GET /api/v1/:bucket/:entry/q` and `POST /api/v1/:bucket/:entry`
  , [PR-212](https://github.com/reductstore/reductstore/pull/212)

## [1.1.1] - 2022-12-08

### Fixed:

- A crush when we handle input chunks after an HTTP
  error, [PR-206](https://github.com/reductstore/reductstore/pull/206)

## [1.1.0] - 2022-11-27

### Added:

- Implement Token API, [PR-199](https://github.com/reductstore/reductstore/pull/199)

### Fixed:

- Link to Entry API in documentation, [PR-194](https://github.com/reductstore/reductstore/pull/194)
- No error body for HEAD `/b/:bucket_name`, [PR-196](https://github.com/reductstore/reductstore/pull/196)
- Use `GET /tokens` instead of `/tokens/list`, [PR-200](https://github.com/reductstore/reductstore/pull/200)

### Changed:

- Always override init-token, [PR-201](https://github.com/reductstore/reductstore/pull/201)
- Update Web Console to v1.1.0, [PR-204](https://github.com/reductstore/reductstore/pull/204)

## [1.0.1] - 2022-10-09

### Added:

- Print 404 error to logs in debug mode, [PR-187](https://github.com/reductstore/reductstore/pull/187)

### Fixed:

- Build MacOs in pipeline, [PR-188](https://github.com/reductstore/reductstore/pull/188)
- Parsing endpoint url to print it in logs, [PR-190](https://github.com/reductstore/reductstore/pull/190)
- Handling negative timestamps, [PR-191](https://github.com/reductstore/reductstore/pull/191)

## [1.0.0] - 2022-10-03

### Added:

- Web Console v1.0.0. [PR-184](https://github.com/reductstore/reductstore/pull/184)

### Removed:

- GET `/bucketname/entryname/list` endpoint, [PR-164](https://github.com/reductstore/reductstore/pull/164)
- POST `/auth/refresh` endpoint, [PR-177](https://github.com/reductstore/reductstore/pull/177)

### Changed:

- Refactor HTTP API layer, [PR-179](https://github.com/reductstore/reductstore/pull/179)
- Prefix `/api/v1/` for all endpoints, [PR-182](https://github.com/reductstore/reductstore/pull/182)

### Fixed:

- Segfault during overriding a record, [PR-183](https://github.com/reductstore/reductstore/pull/183)
- Access to Web Console when authentication is
  enabled, [PR-185](https://github.com/reductstore/reductstore/pull/185)

### Security:

- Check bucket and entry name with regex, [PR-181](https://github.com/reductstore/reductstore/pull/181)

## [0.9.0] - 2022-09-18

### Added:

- Build a static executable for AMD64 and upload it to release from
  CI, [PR-171](https://github.com/reductstore/reductstore/pull/171)
- Build on MacOS, [PR-173](https://github.com/reductstore/reductstore/pull/173)
- Build on Windows, [PR-174](https://github.com/reductstore/reductstore/pull/174)

### Changed:

- Web Console v0.5.0, [PR-175](https://github.com/reductstore/reductstore/pull/175)

## [0.8.0] - 2022-08-26

### Added:

- Web Console v0.4.0
- `Connection` header, [PR-154](https://github.com/reductstore/reductstore/pull/154)
- Publish image to DockerHub, [PR-162](https://github.com/reductstore/reductstore/pull/162)
- Use API token as an access token, [PR-167](https://github.com/reductstore/reductstore/pull/167)

### Fixed:

- Ignoring error code after a failed bucket update, [PR-161](https://github.com/reductstore/reductstore/pull/161)
- Infinite loop in Bucket::KeepQuota, [PR-146](https://github.com/reductstore/reductstore/pull/146)
- Waiting data from HTTP client if it aborts
  connection, [PR-151](https://github.com/reductstore/reductstore/pull/151)
- Writing record when current block is broken, [PR-15-](https://github.com/reductstore/reductstore/pull/153)
- Closing uSocket, [PR-154](https://github.com/reductstore/reductstore/pull/154)
- Removing broken block when it keeps quota, [PR-155](https://github.com/reductstore/reductstore/pull/155)
- Sending headers twice, [PR-156](https://github.com/reductstore/reductstore/pull/156)
- Direction to `cd` into the `build/` directory while building the server
  locally, [PR-159](https://github.com/reductstore/reductstore/pull/159)

### Changed:

- Duplication of timestamps is not allowed, [PR-147](https://github.com/reductstore/reductstore/pull/147)
- Update dependencies, [PR-163](https://github.com/reductstore/reductstore/pull/163)

### Deprecated:

- GET `/auth/refersh` endpoint and access token, [PR-167](https://github.com/reductstore/reductstore/pull/167)

## [0.7.1] - 2022-07-30

### Fixed:

- Opening Web Console, [PR-143](https://github.com/reductstore/reductstore/pull/143)

## [0.7.0] - 2022-07-29

### Added:

- Web Console v0.3.0, [PR-133](https://github.com/reductstore/reductstore/pull/133)
- GET `/b/:bucket/:entry/q?` endpoint for iterating
  data, [PR-141](https://github.com/reductstore/reductstore/pull/141)

### Changed:

- Use `Keep a log` format for CHANGELOG, [PR-136](https://github.com/reductstore/reductstore/pull/136)
- SI for max block and read chunk sizes, [PR-137](https://github.com/reductstore/reductstore/pull/137)
- SHA256 hash for API token is optional, [PR-139](https://github.com/reductstore/reductstore/pull/139)

### Fixed:

- Typo in API documentation, [PR-124](https://github.com/reductstore/reductstore/pull/124)
- Style in documentation, [PR-129](https://github.com/reductstore/reductstore/pull/129)

## [0.6.1] - 2022-06-25

### Added:

- Use Web Console v0.2.1, [PR-120](https://github.com/reductstore/reductstore/pull/120)

## [0.6.0] - 2022-06-25

### Added:

- `Content-Type` header to responses, [PR-107](https://github.com/reductstore/reductstore/pull/107)
- `max_block_records` to bucket settings, [PR-108](https://github.com/reductstore/reductstore/pull/108)
- HEAD `/alive` method for health check, [PR-114](https://github.com/reductstore/reductstore/pull/114)

### Changed:

- Filter unfinished records in GET /b/:bucket/:entry/list
  endpoint, [PR-106](https://github.com/reductstore/reductstore/pull/106)

### Fixed:

- Web Console for RS_API_BASE_PATH, [PR-92](https://github.com/reductstore/reductstore/pull/92)
- Wasting disk space in XFS filesystem, [PR-100](https://github.com/reductstore/reductstore/pull/100)
- Base path in server url, [PR-105](https://github.com/reductstore/reductstore/pull/105)
- Updating record state in asynchronous write
  operation, [PR-109](https://github.com/reductstore/reductstore/pull/109)
- SEGFAULT when entry removed but async writer is
  alive, [PR-110](https://github.com/reductstore/reductstore/pull/110)
- Removing a block with active readers or
  writers, [PR-111](https://github.com/reductstore/reductstore/pull/111)
- Loading bucket settings from disk, [PR-112](https://github.com/reductstore/reductstore/pull/112)
- 404 error for react routes, [PR-116](https://github.com/reductstore/reductstore/pull/116)
- No token error message, [PR-118](https://github.com/reductstore/reductstore/pull/118)
- Updating bucket settings, [PR-119](https://github.com/reductstore/reductstore/pull/119)
- Benchmarks and refactor block management [PR-99](https://github.com/reductstore/reductstore/pull/99)
- CURL to deploy image [PR-104](https://github.com/reductstore/reductstore/pull/104)

### Changed:

- Optimise write operation, [PR-96](https://github.com/reductstore/reductstore/pull/96)
- Disable SSL verification in API tests, [PR-113](https://github.com/reductstore/reductstore/pull/113)

## [0.5.1] - 2022-05-24

### Fixed:

- GET `/b/:bucket/:entry` to avoid creating an empty
  entry, [PR-95](https://github.com/reductstore/reductstore/pull/95)
- Update of bucket settings, [PR-138](https://github.com/reductstore/reductstore/pull/138)

## [0.5.0] - 2022-05-15

### Added:

- Web Console, [PR-77](https://github.com/reductstore/reductstore/pull/77)
- Add default settings for a new bucket in GET /info, [PR-87](https://github.com/reductstore/reductstore/pull/87)
- Link to JS SDK to documentation, [PR-88](https://github.com/reductstore/reductstore/pull/88)

### Changed:

- Only HTTP errors 50x in the logs, [PR-84](https://github.com/reductstore/reductstore/issues/84)

### Fixed:

- CORS functionality, [PR-72](https://github.com/reductstore/reductstore/pull/72)
- Quota policy, [PR-83](https://github.com/reductstore/reductstore/pull/83)

## [0.4.3] - 2022-05-01

### Fixed:

- Sending big blobs [PR-80](https://github.com/reductstore/reductstore/pull/80)
- Handling offset in tryEnd [PR-81](https://github.com/reductstore/reductstore/pull/81)

## [0.4.2] - 2022-04-30

### Fixed:

- Deadlock during sending data, [PR-78](https://github.com/reductstore/reductstore/pull/78)

## [0.4.1] - 2022-04-04

### Fixed:

- Timestamp for oldest record, [PR-68](https://github.com/reductstore/reductstore/pull/68)

## [0.4.0] - 2022-04-01

### Added:

- Asynchronous write/read operations with data blocks, [PR-62](https://github.com/reductstore/reductstore/pull/62)

### Fixed:

- Searching start block in Entry List request, [PR-61](https://github.com/reductstore/reductstore/pull/61)
- Aborting GET requests, [PR-64](https://github.com/reductstore/reductstore/pull/64)

### Changed:

- Block structure in entry, [PR-58](https://github.com/reductstore/reductstore/pull/58)

## [0.3.0]  - 2022-03-14

### Added

- Secure HTTP, [PR-49](https://github.com/reductstore/reductstore/pull/49)
- Stats and list entries to GET /b/:bucket method with
  , [PR-51](https://github.com/reductstore/reductstore/pull/51)
- Access to the latest record, [PR-53](https://github.com/reductstore/reductstore/pull/53)

### Fixed:

- Sending two responses for HTTP error, [PR-48](https://github.com/reductstore/reductstore/pull/48)

### Changed:

- Replace nholmann/json with Protobuf, [PR-47](https://github.com/reductstore/reductstore/pull/47)

## [0.2.1] - 2022-03-07

### Fixed:

* Crushing when API token is wrong, [PR-42](https://github.com/reductstore/reductstore/pull/42)
* Order of authentication checks, [PR-43](https://github.com/reductstore/reductstore/pull/43)

## [0.2.0] - 2022-02-26

### Added:

- HEAD method to Bucket API, [PR-30](https://github.com/reductstore/reductstore/pull/30)
- Extends information from GET method of Server API, [PR-33](https://github.com/reductstore/reductstore/pull/33)
- GET /list end point to browse buckets, [PR-34](https://github.com/reductstore/reductstore/pull/34)
- Bearer token authentication, [PR-36](https://github.com/reductstore/reductstore/pull/36)

### Changed:

- PUT method of Bucket API has optional parameters, [PR-32](https://github.com/reductstore/reductstore/pull/32)

### Fixed:

- Docker build on ARM32, [PR-29](https://github.com/reductstore/reductstore/pull/29)
- IBucket::List error 500 for timestamps between
  blocks, [PR-31](https://github.com/reductstore/reductstore/pull/31)
- Wrong parameters in Entry API documentation, [PR-38](https://github.com/reductstore/reductstore/pull/38)

## [0.1.1] - 2022-02-13

### Fixed:

- Default folder for data in Docker image, [PR-23](https://github.com/reductstore/reductstore/pull/23)

## [0.1.0] - 2022-01-24

- Initial release with basic HTTP API and FIFO bucket quota

[Unreleased]: https://github.com/reductstore/reductstore/compare/v1.7.0...HEAD

[1.7.0]: https://github.com/reductstore/reductstore/compare/v1.6.2...v1.7.0

[1.6.2]: https://github.com/reductstore/reductstore/compare/v1.6.1...v1.6.2

[1.6.1]: https://github.com/reductstore/reductstore/compare/v1.6.0...v1.6.1

[1.6.0]: https://github.com/reductstore/reductstore/compare/v1.5.1...v1.6.0

[1.5.1]: https://github.com/reductstore/reductstore/compare/v1.5.0...v1.5.1

[1.5.0]: https://github.com/reductstore/reductstore/compare/v1.4.1...v1.5.0

[1.4.1]: https://github.com/reductstore/reductstore/compare/v1.4.0...v1.4.1

[1.4.0]: https://github.com/reductstore/reductstore/compare/v1.4.0-beta.1...v1.4.0

[1.4.0-beta.1]: https://github.com/reductstore/reductstore/compare/v1.4.0-alpha.2...v1.4.0-beta.1

[1.4.0-alpha.2]: https://github.com/reductstore/reductstore/compare/v1.4.0-alpha.1...v1.4.0-alpha.2

[1.4.0-alpha.1]: https://github.com/reductstore/reductstore/compare/v1.3.2...v1.4.0-alpha.1

[1.3.2]: https://github.com/reductstore/reductstore/compare/v1.3.1...v1.3.2

[1.3.1]: https://github.com/reductstore/reductstore/compare/v1.3.0...v1.3.1

[1.3.0]: https://github.com/reductstore/reductstore/compare/v1.2.3...v1.3.0

[1.2.3]: https://github.com/reductstore/reductstore/compare/v1.2.2...v1.2.3

[1.2.2]: https://github.com/reductstore/reductstore/compare/v1.2.1...v1.2.2

[1.2.1]: https://github.com/reductstore/reductstore/compare/v1.2.0...v1.2.1

[1.2.0]: https://github.com/reductstore/reductstore/compare/v1.1.1...v1.2.0

[1.1.1]: https://github.com/reductstore/reductstore/compare/v1.1.0...v1.1.1

[1.1.0]: https://github.com/reductstore/reductstore/compare/v1.0.0...v1.1.0

[1.0.1]: https://github.com/reductstore/reductstore/compare/v1.0.0...v1.0.1

[1.0.0]: https://github.com/reductstore/reductstore/compare/v0.9.0...v1.0.0

[0.9.0]: https://github.com/reductstore/reductstore/compare/v0.8.0...v0.9.0

[0.8.0]: https://github.com/reductstore/reductstore/compare/v0.7.1...v0.8.0

[0.7.1]: https://github.com/reductstore/reductstore/compare/v0.7.0...v0.7.1

[0.7.0]: https://github.com/reductstore/reductstore/compare/v0.6.1...v0.7.0

[0.6.1]: https://github.com/reductstore/reductstore/compare/v0.6.0...v0.6.1

[0.6.0]: https://github.com/reductstore/reductstore/compare/v0.5.1...v0.6.0

[0.5.1]: https://github.com/reductstore/reductstore/compare/v0.5.0...v0.5.1

[0.5.0]: https://github.com/reductstore/reductstore/compare/v0.4.3...v0.5.0

[0.4.3]: https://github.com/reductstore/reductstore/compare/v0.4.2...v0.4.3

[0.4.2]: https://github.com/reductstore/reductstore/compare/v0.4.1...v0.4.2

[0.4.1]: https://github.com/reductstore/reductstore/compare/v0.4.0...v0.4.1

[0.4.0]: https://github.com/reductstore/reductstore/compare/v0.3.0...v0.4.0

[0.3.0]: https://github.com/reductstore/reductstore/compare/v0.2.1...v0.3.0

[0.2.1]: https://github.com/reductstore/reductstore/compare/v0.2.0...v0.2.1

[0.2.0]: https://github.com/reductstore/reductstore/compare/v0.1.1...v0.2.0

[0.1.1]: https://github.com/reductstore/reductstore/compare/v0.1.0...v0.1.1

[0.1.0]: https://github.com/reductstore/reductstore/releases/tag/v0.1.0
