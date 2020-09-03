; Test case for double macros

	processor 6502

 MAC testmac
	lda #1
	ds	33, $a5
 ENDM
 MAC testmac
	lda #1
	ds	33, $a5
 ENDM

	ORG $f800

	testmac

