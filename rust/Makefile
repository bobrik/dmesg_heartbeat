KDIR = /lib/modules/$(shell uname -r)/build

default:
	make -C $(KDIR) M=$$PWD

clean:
	make -C $(KDIR) M=$$PWD clean
