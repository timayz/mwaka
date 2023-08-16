test:
	cargo test

fmt:
	cargo fmt -- --emit files

clippy:
	cargo clippy --fix --all-features -- -D warnings

deny:
	cargo deny check

udeps:
	cargo udeps -p mwaka -p mwaka-aria

udeps.leptos:
	cargo udeps -p demo -p docs 

advisory.clean:
	rm -rf ~/.cargo/advisory-db

pants: advisory.clean
	cargo pants

audit: advisory.clean
	cargo audit

outdated:
	cargo outdated
