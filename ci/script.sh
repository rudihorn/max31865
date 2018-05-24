set -euxo pipefail

main() {
    cargo check --target $TARGET
    cargo check --target $TARGET --example rpi
    cargo test --target $TARGET
}

main
