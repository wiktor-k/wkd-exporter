#!/usr/bin/env -S just --working-directory . --justfile
# Since this is a first recipe it's being run by default.
# Faster checks need to be executed first for better UX.  For example
# codespell is very fast. cargo fmt does not need to download crates etc.

clippy := "cargo clippy --quiet --workspace --no-deps --all-targets"
clippy_args := "-D warnings"
nextest_args := "--locked --workspace"
udeps_args := "--quiet --workspace --all-features --all-targets"

# Perform all checks
[parallel]
check: spell fmt doc lints deps unused-deps test integration-test

# Check spelling
[group('ci')]
[metadata('pacman', 'codespell')]
spell:
    codespell

# Check source code formatting
[group('ci')]
[metadata('pacman', 'rustup')]
fmt:
    just --unstable --fmt --check
    # We're using nightly to properly group imports, see .rustfmt.toml
    rustup component add --toolchain nightly rustfmt
    cargo +nightly fmt --quiet --all -- --check

# Lint the source code
lints:
    {{ clippy }} -- {{ clippy_args }}

# Report lints in the source code
[group('ci')]
[metadata('gitlabci-job', '{"artifacts":{"reports":{"codequality":"codeclimate.json"}}}')]
[metadata('pacman', 'cargo-sonar', 'rust')]
report-lints:
    #!/usr/bin/bash
    set -euo pipefail

    readonly CLIPPY_JSON=$(mktemp)
    {{ clippy }} --message-format=json -- {{ clippy_args }} > "$CLIPPY_JSON" || EC=$?
    cargo codeclimate --clippy --clippy-path "$CLIPPY_JSON" --codeclimate-path codeclimate.json
    exit "${EC:-0}"

# Check for issues with dependencies
deps:
    cargo deny check

# Report issues with dependencies
[group('ci')]
[metadata('gitlabci-job', '{"artifacts":{"reports":{"codequality":"codeclimate.json"}}}')]
[metadata('pacman', 'cargo-deny', 'cargo-sonar')]
report-deps:
    #!/usr/bin/bash
    set -euo pipefail

    readonly DENY_JSON=$(mktemp)
    cargo deny --format json check 2> "$DENY_JSON" || EC=$?
    cargo codeclimate --deny --deny-path "$DENY_JSON" --codeclimate-path codeclimate.json
    exit "${EC:-0}"

# Check for unused dependencies
unused-deps:
    cargo +nightly udeps {{ udeps_args }}

# Report unused dependencies
[group('ci')]
[metadata('gitlabci-job', '{"artifacts":{"reports":{"codequality":"codeclimate.json"}}}')]
[metadata('pacman', 'cargo-sonar', 'cargo-udeps', 'rust')]
report-unused-deps:
    #!/usr/bin/bash
    set -euxo pipefail

    readonly UDEPS_JSON=$(mktemp)
    cargo +nightly udeps {{ udeps_args }} --output json > "$UDEPS_JSON" || EC=$?
    cargo +nightly codeclimate --udeps --udeps-path "$UDEPS_JSON" --codeclimate-path codeclimate.json
    exit "${EC:-0}"

# Run unit tests
test:
    cargo nextest run {{ nextest_args }}
    # doctests are not currently run by cargo-nextest
    # CHECK https://github.com/nextest-rs/nextest/issues/16
    cargo test --doc

# Run integration tests
integration-test:
    #!/usr/bin/bash
    set -euo pipefail
    cargo +nightly build --locked
    readonly target=$(cargo +nightly metadata --format-version 1 | jq --raw-output '.target_directory')
    tangler sh < README.md | sed --quiet --regexp-extended 's/^\$ (.*)/\1/p' | PATH="$target/debug:$PATH" bash -euxo pipefail -

# Report on all tests
[group('ci')]
[metadata('gitlabci-job', '{"coverage":"/Line coverage: ([0-9.]*)%/","artifacts":{"when":"always","reports":{"junit":"target/nextest/ci/junit.xml","metrics":"target/metrics.txt","coverage_report":{"coverage_format":"cobertura","path":"target/coverage.xml"}}}}')]
[metadata('pacman', 'rust', 'cargo-llvm-cov', 'cargo-nextest', 'rustup', 'tangler', 'tree')]
report-test:
    #!/usr/bin/bash
    # enabling "x" here will garble text output that's parsed by GitLab for code coverage
    set -euo pipefail

    rustup component add --toolchain nightly llvm-tools-preview

    source <(cargo +nightly llvm-cov show-env --export-prefix --doctests --branch)
    cargo +nightly llvm-cov clean

    just test integration-test

    # explicitly use "target" (even if CARGO_TARGET_DIR is somewhere else) so that
    # local tools (such as https://github.com/ryanluker/vscode-coverage-gutters) can find the file
    cargo +nightly llvm-cov --quiet report --cobertura --output-path target/coverage.xml > /dev/null 2>&1

    LINE_RATE=$(head target/coverage.xml | sed -nE 's/(.*coverage.*line-rate=")([^"]*)".*/\2/p')
    LINE_PERCENT=$(echo "$LINE_RATE" | awk '{print $1 * 100}')
    printf 'Line coverage: %s%%\n' "$LINE_PERCENT"

    BRANCH_RATE=$(head target/coverage.xml | sed -nE 's/(.*coverage.*branch-rate=")([^"]*)".*/\2/p')
    printf 'line_coverage_ratio %s\nbranch_coverage_ratio %s\n' "$LINE_RATE" "$BRANCH_RATE" > target/metrics.txt

# Generate HTML report for the coverage
coverage-html-report: report-test
    #!/usr/bin/bash
    set -euo pipefail
    source <(cargo +nightly llvm-cov show-env --export-prefix)
    cargo +nightly llvm-cov --quiet report --html > /dev/null 2>&1

# Build docs
[group('ci')]
[metadata('pacman', 'rust')]
doc:
    RUSTDOCFLAGS='-D warnings' cargo doc --quiet --no-deps --document-private-items

# Check commit messages
[group('ci')]
[metadata('pacman', 'codespell', 'git')]
commits:
    #!/usr/bin/env bash
    set -Eeuo pipefail

    # fetch default branch if it's set
    if [[ -v CI_DEFAULT_BRANCH ]]; then
        git fetch origin $CI_DEFAULT_BRANCH
        refs="origin/$CI_DEFAULT_BRANCH"
    else
        refs="main"
    fi

    commits=$(git rev-list "${refs}..")
    for commit in $commits; do
      MSG="$(git show -s --format=%B "$commit")"
      CODESPELL_RC="$(mktemp)"
      git show "$commit:.codespellrc" > "$CODESPELL_RC"
      if ! grep -q "Signed-off-by: " <<< "$MSG"; then
        printf "Commit %s lacks \"Signed-off-by\" line.\n" "$commit"
        printf "%s\n" \
            "  Please use:" \
            "    git rebase --signoff main && git push --force-with-lease" \
            "  See https://developercertificate.org/ for more details."
        exit 1;
      elif ! codespell --config "$CODESPELL_RC" - <<< "$MSG"; then
        printf "The spelling in commit %s needs improvement.\n" "$commit"
        exit 1;
      else
        printf "Commit %s is good.\n" "$commit"
      fi
    done

# Fixes common issues. Files need to be git add'ed
fix:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! git diff-files --quiet ; then
        echo "Working tree has changes. Please stage them: git add ."
        exit 1
    fi

    codespell --write-changes
    just --unstable --fmt
    # try to fix rustc issues
    cargo fix --allow-staged
    # try to fix clippy issues
    cargo clippy --fix --allow-staged

    # fmt must be last as clippy's changes may break formatting
    cargo +nightly fmt --all

render-script := '''
    //! ```cargo
    //! [dependencies]
    //! pkg = { path = "PATH", package = "wkd-exporter", features = ["cli"] }
    //! clap_allgen = "0.2.1"
    //! ```

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        clap_allgen::render_KIND::<pkg::cli::Cli>(
            &std::env::args().collect::<Vec<_>>()[1],
        )?;
        Ok(())
    }
'''

# Render `manpages` or `shell_completions` (`kind`) for the package.
generate kind:
    #!/usr/bin/bash

    set -Eeuo pipefail

    readonly output_dir="${CARGO_TARGET_DIR:-$PWD/output}"
    mkdir --parents "$output_dir"

    readonly kind="{{ kind }}"

    case "$kind" in
      manpages|shell_completions)
          ;;
      *)
          printf 'Only "manpages" and "shell_completions" are supported.\n'
          exit 1
    esac

    script="$(mktemp --suffix=.rs)"
    sed "s#PATH#$PWD#g;s/KIND/{{ kind }}/g" > "$script" <<< '{{ render-script }}'
    rust-script "$script" "$output_dir/{{ kind }}"
    rm --force "$script"
