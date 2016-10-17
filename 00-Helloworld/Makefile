.PHONY: clean
CROSS_COMPILE ?= arm-none-eabi-

CC := $(CROSS_COMPILE)gcc
LD := $(CROSS_COMPILE)ld

CFLAGS = -fno-common -O0 -mcpu=cortex-m3 -mthumb -nostartfiles

rust_target = armv7-unknown-linux-gnueabihf
rust_os := ./target/$(rust_target)/debug/libmini_arm.a
linker_script = hello.ld
assembly_object_files = startup.o

TARGET = hello.elf

all: $(TARGET)

$(TARGET): cargo $(rust_os) $(assembly_object_files) $(LINKER_SCRIPT)
	$(LD) -n -T $(linker_script) -o $@ $(assembly_object_files) $(rust_os)
	$(CROSS_COMPILE)objcopy -Obinary hello.elf hello.bin
	$(CROSS_COMPILE)objdump -S hello.elf > hello.list

cargo:
	@cargo build --target=$(rust_target)

%.o: %.S
	$(CC) -c $^ -o $@

qemu: $(TARGET)
	@qemu-system-arm -M ? | grep stm32-p103 >/dev/null || exit
	@echo "Press Ctrl-A and then X to exit QEMU"
	@echo
	qemu-system-arm -M stm32-p103 -nographic -kernel hello.bin

# clean:
# 	rm -f *.o *.bin *.elf *.list DAC*
