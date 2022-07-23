export TARGET_DIR = $(shell pwd)

TARGETS = arch.a core.a mm.a lib.a
SUBDIRS = $(basename $(TARGETS))
KERNEL = silicium

export CFLAGS += -I $(shell pwd)/include

# LD flags
LDFLAGS += -flto -nostdlib -T link.ld -Wl,--gc-sections

.PHONY: $(SUBDIRS) all clean install

all: $(KERNEL)

$(KERNEL): $(TARGETS)
	$(LD) $(LDFLAGS) $^ -o $@ -lgcc

%.a:
	make -C $(basename $@) all
	make -C $(basename $@) install

clean:
	for dir in $(SUBDIRS); do 	\
        make -C $$dir clean; 	\
    done
	rm -f $(KERNEL) $(TARGETS) 

$(SUBDIRS):
	make -C $@ clean

install:
	cp $(KERNEL) $(INSTALL_DIR)/$(KERNEL)
