#!/usr/bin/env -S just --working-directory . --justfile
# Since this is a first recipe it's being run by default.
# Faster checks need to be executed first for better UX.  For example
# codespell is very fast. cargo fmt does not need to download crates etc.

# Perform all checks
check: spell fmt doc lints deps test

# Check spelling
[group('ci')]
spell:
    # pacman -S codespell
    codespell

# Check source code formatting
[group('ci')]
fmt:
    # pacman -S rustup
    just --unstable --fmt --check
    # We're using nightly to properly group imports, see .rustfmt.toml
    rustup component add --toolchain nightly rustfmt
    cargo +nightly fmt --all -- --check

# Lint the source code
[group('ci')]
lints:
    # pacman -S rust
    cargo clippy --tests --all --workspace --no-deps --all-targets -- -D warnings

# Check for issues with dependencies
[group('ci')]
deps:
    # pacman -S cargo-deny
    cargo deny check

# Run unit tests
[group('ci')]
test:
    #!/usr/bin/bash
    set -euo pipefail

    # pacman -S rust cargo-llvm-cov cargo-nextest rustup
    rustup component add --toolchain nightly llvm-tools-preview
    cargo +nightly llvm-cov --branch --no-report nextest --locked --all
    # doctests are not currently run by cargo-nextest
    # see: https://github.com/nextest-rs/nextest/issues/16
    cargo +nightly llvm-cov --branch --no-report --doc --locked

    # gitlabci-job {"coverage":"/Line coverage: ([0-9.]*)%/","artifacts":{"reports":{"metrics":"target/metrics.txt","coverage_report":{"coverage_format":"cobertura","path":"target/coverage.xml"}}}}
    cargo +nightly llvm-cov --quiet --branch report --cobertura --output-path target/coverage.xml > /dev/null 2>&1

    LINE_RATE=$(head target/coverage.xml | sed -nE 's/(.*coverage.*line-rate=")([^"]*)".*/\2/p')
    LINE_PERCENT=$(echo "$LINE_RATE" | awk '{print $1 * 100}')
    printf 'Line coverage: %s%%\n' "$LINE_PERCENT"

    BRANCH_RATE=$(head target/coverage.xml | sed -nE 's/(.*coverage.*branch-rate=")([^"]*)".*/\2/p')
    printf 'line_coverage_ratio %s\nbranch_coverage_ratio %s\n' "$LINE_RATE" "$BRANCH_RATE" > target/metrics.txt

# Build docs
[group('ci')]
doc:
    # pacman -S rust
    RUSTDOCFLAGS='-D warnings' cargo doc --no-deps

# Check commit messages
[group('ci')]
commits:
    #!/usr/bin/env bash
    set -Eeuo pipefail
    # pacman -S codespell git

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
