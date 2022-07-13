KERNEL = silicium

# GCC flags
CFLAGS += -c -W -Wall -Wextra -Wshadow
CFLAGS += -Wredundant-decls -Wstrict-prototypes
CFLAGS += -Wno-unused-function -Wno-unused-parameter -Wno-unused-variable
CFLAGS += -ffreestanding -I include -fdata-sections -ffunction-sections

# AS flags
ASFLAGS += -c

# LD flags
LDFLAGS += -flto -nostdlib -T link.ld -Wl,--gc-sections

# Source files
CFILES = $(shell find . -type f -name "*.c")
ASFILES = $(shell find . -type f -name "*.asm")

# Objects files
OBJFILES = $(ASFILES:.asm=.o)
OBJFILES += $(CFILES:.c=.o)

.PHONY: all clean

all: $(KERNEL) 

$(KERNEL): $(OBJFILES)
	$(LD) $(LDFLAGS) $^ -o $@ -lgcc
	# $(STRIP) --strip-all $@		# Remove all symbols

%.o: %.c
	$(CC) $(CFLAGS) $^ -o $@

%.o: %.asm
	$(AS) $(ASFLAGS) $^ -o $@

clean:
	rm -f $(OBJFILES) $(OUTPUT)

install:
	cp $(KERNEL) $(INSTALL_DIR)/$(KERNEL)
