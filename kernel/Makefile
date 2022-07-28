export TARGET_DIR = $(shell pwd)

TARGETS = arch.a core.a mm.a process.a lib.a
SUBDIRS = $(basename $(TARGETS))
KERNEL = silicium

export CFLAGS += -I $(shell pwd)/include

# LD flags
LDFLAGS += -flto -nostdlib -T link.ld 
LDFLAGS += -Wl,--whole-archive -Wl,--gc-sections

.PHONY: all clean install
all: $(KERNEL)
	make -C module all

$(KERNEL): $(TARGETS)
	$(LD) $(LDFLAGS) $^ -o $@ -lgcc

%.a:
	make -C $(basename $@) all
	make -C $(basename $@) install

clean:
	for dir in $(SUBDIRS); do 	\
        make -C $$dir clean; 	\
    done
	make -C module clean
	rm -f $(KERNEL) $(TARGETS) 

$(SUBDIRS):
	make -C $@ clean

install:
	make -C module install
	cp $(KERNEL) $(INSTALL_DIR)/$(KERNEL)
