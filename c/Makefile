KDIR = /lib/modules/$(shell uname -r)/build

.DEFAULT_GOAL := build

build:
	make -C $(KDIR) M=$$PWD

clean:
	make -C $(KDIR) M=$$PWD clean
