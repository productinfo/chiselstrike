// SPDX-FileCopyrightText: © 2021 ChiselStrike <info@chiselstrike.com>

mod common;

mod linters {
    use crate::common::{cargo, cargo_install, get_deno_version, nightly, run};

    #[test]
    fn eslint() {
        run("npm", ["install"]);
        run("npx", ["eslint", ".", "--ext", ".ts"]);
    }

    #[test]
    fn deno_checks() {
        // Find our deno version and install that. We don't use --path
        // because that always reinstall the binary.
        let version = get_deno_version();
        cargo_install(&version, "deno", "deno");
        run("deno", ["lint", "--config", "deno.json"]);
        run("deno", ["fmt", "--config", "deno.json", "--check"]);
    }

    #[test]
    fn sorted_dependencies() {
        cargo_install("1.0.5", "cargo-sort", "cargo-sort");
        cargo(["sort", "-w", "-c"]);
    }

    #[test]
    fn unused_dependencies() {
        cargo_install("0.1.29", "cargo-udeps", "cargo-udeps");
        nightly(["udeps"]);
    }

    #[test]
    fn check_formating() {
        cargo(["fmt", "--", "--check"]);
    }

    #[test]
    fn check_clippy() {
        cargo(["clippy", "--all-targets", "--", "-D", "warnings"]);
    }
}
