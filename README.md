# thingz

cli for thingz cloud

## Dev Workflow

```bash
cargo bump patch --git-tag
git push --follow-tags
```

```bash
# cargo release
cargo publish

# build a release file
cargo build --release
```

## Todo

- config files should be passed as param or to be from $HOME/.thingz.toml
