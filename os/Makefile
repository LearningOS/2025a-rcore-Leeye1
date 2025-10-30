# Makefile for rCore-like project (WSL friendly)
# - ensures qemu prints to current terminal (nographic + serial mon:stdio)
# - default run uses -bios default -kernel $(KERNEL_ELF)
# - keep loader mode as run-loader for experiments

# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm
OFFLINE :=

# BOARD / SBI
BOARD := qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

# Building mode argument
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# Kernel entry (when using loader)
KERNEL_ENTRY_PA := 0x80200000

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

CHAPTER ?= $(shell git rev-parse --abbrev-ref HEAD | sed -E 's/ch([0-9])/\1/')
TEST ?= $(CHAPTER)
BASE ?= 1

# Disassembly
DISASM ?= -x

# QEMU common
QEMU := qemu-system-riscv64
QEMU_MACHINE ?= virt
QEMU_MEM ?= 256M
QEMU_SMP ?= 1
QEMU_SERIAL ?= mon:stdio
QEMU_COMMON_ARGS := -machine $(QEMU_MACHINE) -m $(QEMU_MEM) -smp $(QEMU_SMP) -nographic -serial $(QEMU_SERIAL)

# Default target
.PHONY: all
all: build

build: env $(KERNEL_BIN)

env:
ifeq ($(OFFLINE),)
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils || true
	rustup component add rust-src || true
	rustup component add llvm-tools-preview || true
endif

$(KERNEL_BIN): kernel
	@echo "[OBJCOPY] $(KERNEL_ELF) -> $@"
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

kernel:
	@make -C ../user build TEST=$(TEST) CHAPTER=$(CHAPTER) BASE=$(BASE)
	@echo "Platform: $(BOARD)"
	@cargo build $(MODE_ARG)

clean:
	@cargo clean

disasm: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less

disasm-vim: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@vim $(DISASM_TMP)
	@rm $(DISASM_TMP)

# ---------------------------
# RUN / DEBUG targets
# ---------------------------

# Primary run: use default BIOS (OpenSBI) + kernel ELF (recommended & stable)
.PHONY: run run-default run-loader run-help
run: run-default

run-default: build
	@echo "[QEMU run-default] $(QEMU) $(QEMU_COMMON_ARGS) -bios default -kernel $(KERNEL_ELF)"
	@$(QEMU) $(QEMU_COMMON_ARGS) -bios default -kernel $(KERNEL_ELF)

# Loader-style run: inject binary at physical address (legacy / experimental)
run-loader: build
	@echo "[QEMU run-loader] $(QEMU) $(QEMU_COMMON_ARGS) -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)"
	@$(QEMU) $(QEMU_COMMON_ARGS) -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

# Debug (tmux-based): qemu waits for gdb, then open gdb in split pane
.PHONY: debug
debug: build
	@echo "[QEMU debug] starting qemu (waiting for gdb) and tmux session"
	@tmux new-session -d \
		"$(QEMU) $(QEMU_COMMON_ARGS) -bios default -kernel $(KERNEL_ELF) -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

# gdbserver: qemu waits for remote gdb on :1234
.PHONY: gdbserver
gdbserver: build
	@echo "[QEMU gdbserver] $(QEMU) $(QEMU_COMMON_ARGS) -bios default -kernel $(KERNEL_ELF) -s -S"
	@$(QEMU) $(QEMU_COMMON_ARGS) -bios default -kernel $(KERNEL_ELF) -s -S

# gdb client helper
.PHONY: gdbclient
gdbclient:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

# quick kill helper (useful when QEMU takes the terminal)
.PHONY: qkill
qkill:
	@pkill -f qemu-system-riscv64 || true

# help
.PHONY: help
help:
	@echo "Makefile targets:"
	@echo "  make build        # build kernel and kernel.bin"
	@echo "  make run          # recommended: run with -bios default -kernel (nographic + serial->stdio)"
	@echo "  make run-loader   # experimental: use -device loader,file=... (inject binary at addr)"
	@echo "  make debug        # start qemu waiting for gdb and open gdb in tmux"
	@echo "  make gdbserver    # start qemu waiting for gdb (no tmux)"
	@echo "  make gdbclient    # connect gdb client to localhost:1234"
	@echo "  make disasm       # disassemble kernel ELF"
	@echo "  make clean"
