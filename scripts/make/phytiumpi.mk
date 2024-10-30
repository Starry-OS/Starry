chainboot: build
	tools/phytiumpi/yet_another_uboot_transfer.py /dev/ttyUSB0 115200 $(OUT_BIN)
	echo ' ' > minicom_output.log
	minicom -D /dev/ttyUSB0 -b 115200 -C minicom_output.log
# python tools/phytiumpi/uboot_transfer.py /dev/ttyUSB0 115200 $(OUT_BIN)
#	python tools/phytiumpi/uboot_test_send.py /dev/ttyUSB0 115200 $(OUT_BIN)
#ruby tools/phytiumpi/uboot_transfer.rb /dev/ttyUSB0 115200 $(OUT_BIN)
	
