# Display Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version              # Rust compiler
	cargo --version              # Rust package manager
	rustfmt --version            # Rust code formatter
	rustup --version             # Rust toolchain manager
	clippy-driver --version      # Rust linter

# Format code using rustfmt
format:
	cargo fmt --quiet

# Run clippy for linting
lint:
	cargo clippy --quiet

# Run tests
test:
	cargo test --quiet

# Build and run the project
run:
	cargo run

# Build release version
release:
	cargo build --release

check:
	cargo check

# Install Rust toolchain if needed
install:
	# Install if needed
	# @echo "Updating rust toolchain"
	# rustup update stable
	# rustup default stable 

# Run all formatting, linting, and testing tasks
all: format lint test run

# Custom tasks

# Example: Extract data
extract: 
	cargo run extract

# Example: Transform and Load data
transform_load:
	cargo run transform_load

# Example: Create a database entry
create:
	cargo run query "INSERT INTO nfl_Receivers (pfr_player_id, player_name, career_try, career_ranypa, career_wowy, bcs_rating) VALUES ('SeweSt00','Steve Sewell',2247.446578,0.095783529,-0.729682089,NULL);"

# Example: Read from the database
read:
	cargo run query "SELECT * FROM nfl_Receivers ORDER BY career_try DESC LIMIT 5;"

# Example: Update a database entry
update:
	cargo run query "UPDATE nfl_Receivers SET player_name = 'Jenny' WHERE id = 1;"

# Example: Delete a database entry
delete:
	cargo run query "DELETE from nfl_Receivers WHERE id = 1;"