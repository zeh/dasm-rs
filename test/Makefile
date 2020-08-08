# demo.asm doesn't compile!
# no useful reference output for suitef8.asm right now

DASM=../bin/dasm
FTOHEX=../bin/ftohex

test:
	./run_tests.sh $(DASM)

%.bin: %.asm
	$(DASM) $< -o$@ -f1 -DINEEPROM

%.hex: %.bin
	$(FTOHEX) 1 $< $@

clean:
	rm -rf *.bin *.hex *.txt
	rm -rf ./atari2600/*.bin ./atari2600/*.hex ./atari2600/*.txt
	rm -rf ./atari7800/*.bin ./atari7800/*.hex ./atari7800/*.txt
	rm -rf ./channel-f/*.bin ./channel-f/*.hex ./channel-f/*.txt
