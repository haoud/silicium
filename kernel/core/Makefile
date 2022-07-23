TARGET = core.a

# GCC flags
CFLAGS += -c -W -Wall -Wextra -Wshadow
CFLAGS += -Wredundant-decls -Wstrict-prototypes
CFLAGS += -Wno-unused-function -Wno-unused-parameter -Wno-unused-variable
CFLAGS += -ffreestanding -I include -fdata-sections -ffunction-sections

# AS flags
ASFLAGS += -c

# Source files
CFILES = $(shell find . -type f -name "*.c")
ASFILES = $(shell find . -type f -name "*.asm")

# Objects files
OBJFILES = $(ASFILES:.asm=.o)
OBJFILES += $(CFILES:.c=.o)

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(OBJFILES)
	ar rcs $@ $^

%.o: %.c
	$(CC) $(CFLAGS) $^ -o $@

%.o: %.asm
	$(AS) $(ASFLAGS) $^ -o $@

clean:
	rm -f $(OBJFILES) $(TARGET)

install:
	cp $(TARGET) $(TARGET_DIR)/$(TARGET)
