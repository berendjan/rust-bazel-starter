# Rules rust things to know

## Generate VSCode launch.json for debugging all Rust targets in the current workspace:

```bash
bazel run @rules_rust//tools/vscode:gen_launch_json
```

## Rust Analyzer

After any changes in dependencies run the following command to update `rust-project.json`

```bash
bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
```


## Rustfmt

```bash
bazel run @rules_rust//:rustfmt
```
