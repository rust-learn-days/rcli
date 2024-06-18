init:
	git init
	git add .
	git commit -m "init"
	git branch -M main
	git remote add origin git@github.com:rust-learn-days/rcli.git
	pip install pre-commit
	cargo install --locked cargo-deny
	cargo install typos-cli
	cargo install git-cliff
	cargo install cargo-nextest --locked
	cargo install cross --git https://github.com/cross-rs/cross
	pre-commit install
	sed -i '' 's#rust-learn-days/template#rust-learn-days/rcli#g' cliff.toml


build:
    rustup target add ${target}
    cargo build --target  ${target}

cross-build:
    rustup target add ${target}
    cross build --target  ${target}

cross-build-release:
    rustup target add ${target}
    cross build --release --target  ${target}
    mkdir -p output/${target}
    cp target/${target}/release/rcli output/${target}/rcli
    cd output/${target} && tar -zcvf rcli-${target}.tar.gz rcli

build-release:
    rustup target add ${target}
    cargo build --release --target  ${target}
    mkdir -p output/${target}
    cp target/${target}/release/rcli output/${target}/rcli
    cd output/${target} && tar -zcvf rcli-${target}.tar.gz rcli

changelog:
    git-cliff --config cliff.toml > CHANGELOG.md
