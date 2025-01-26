all:
	cargo build --release
	avr-size --mcu=atmega328p --format=avr target/avr-atmega328p/release/blink-nano.elf
	du -sh target/
	echo "done"
