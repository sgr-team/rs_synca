# Development

## Tools

- [git](https://git-scm.com/) - no comments
- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) - no comments
- [mdbook](https://rust-lang.github.io/mdBook/index.html) - used for [SyncA Book](https://synca.sgr-team.dev)
- [docker](https://www.docker.com) - used for run tests environment (postgres & other)

## Tests

To run all tests, use the script ["./tests/run.sh"](https://github.com/sgr-team/rs_synca/blob/develop/tests/run.sh).

```bash
./tests/run.sh
```

## Branches & features

- [main](https://github.com/sgr-team/rs_synca/tree/main) - branch with releases published in [crates.io](https://crates.io)
- [develop](https://github.com/sgr-team/rs_synca/tree/develop) - branch with features of the next release
- *Feature branches* - branches that contain atomic features (code gets into the [develop branch](https://github.com/sgr-team/rs_synca/tree/develop) through [pull requests](https://github.com/sgr-team/rs_synca/pulls))

## Release version

- Create a commit with the version in the development branch (don't forget to increment version in synca/Cargo.toml)
- Create a [pull request](https://github.com/sgr-team/rs_synca/pulls) into [main branch](https://github.com/sgr-team/rs_synca/tree/main)
- Create a git tag that points into the [main branch](https://github.com/sgr-team/rs_synca/tree/main)
- Publish

```bash
cargo publish -p synca --allow-dirty
```