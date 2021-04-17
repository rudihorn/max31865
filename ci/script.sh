set -euxo pipefail

main() {
    cargo check --target $TARGET

    if [ $TRAVIS_BRANCH = extra_examples ]; then
        if [ $TARGET = x86_64-unknown-linux-gnu ]; then
            # cargo check --example --target $TARGET --example rpi
            echo 
        elif [ $TARGET = thumbv7m-none-eabi ]; then
            cargo check --example --target $TARGET --example stm32
            # cargo check --example --target $TARGET --example stm32_ssd1306
        fi
    fi

    if [ $TARGET = x86_64-unknown-linux-gnu ]; then 
        # the --tests is required to ignore the examples
        # which will not compile under x86
        cargo test --target $TARGET
    fi
}

main
