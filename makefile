build:
	{ \
	cargo build --release; \
	cp -f $(PWD)/target/release/$(bin) $(PWD); \
	sudo cp -f $(PWD)/$(bin) /usr/local/bin/$(bin); \
	}

