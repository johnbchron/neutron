
# run the project
run:
	MTL_HUD_ENABLED=1 cargo run

run-release:
	MTL_HUD_ENABLED=1 cargo run --release
	
# run the pre-commit checklist
pre-c:
	cargo fmt
	cargo clippy
	cargo test	
	
