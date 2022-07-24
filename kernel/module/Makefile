# GCC flags
CFLAGS += -c -fno-lto -W -Wall -Wextra -Wshadow
CFLAGS += -Wredundant-decls -Wstrict-prototypes
CFLAGS += -Wno-unused-function -Wno-unused-parameter -Wno-unused-variable
CFLAGS += -ffreestanding -Wl,-r

CFILES = $(shell find . -type f -name "*.c")
MODFILES = $(CFILES:.c=.kmd)

.PHONY: all clean

all: $(MODFILES)

%.kmd: %.c
	$(CC) $(CFLAGS) $^ -o $@

clean:
	rm -f $(MODFILES)

install:
	cp $(MODFILES) $(INITRD_DIR)/
