// 67 lines 46 code 10 comments 11 blanks
#include "config.h"

/* BIG FAT WARNING GOES HERE */
#define dbg(char) \
	mov \char, %ax ;\
	out %ax, $0xe9 ;\
#endif

.align 16
.code16
.section .resettext, "xa", @progbits
reset_vector:
	cli
	jmp switch_modes

.section .text
switch_modes:
	/*
	 * The developer manual builds up the GDT, but since it should get
	 * cached by the CPU, we can just have it in the flash.
	 */
	mov %cs, %ax
	mov %ax, %ds

	/* Enable protected mode (PE) */
	mov %cr0, %eax
	or $1, %al
	mov %eax, %cr0

	ljmpl $0x8,$protected_mode

.code32
protected_mode:
	mov $0x10, %ax
	mov %ax, %ds
	mov %ax, %es
	mov %ax, %fs
	mov %ax, %gs
	mov %ax, %ss


gdt_desc:
	.word egdt - gdt - 1
	.long gdt

.align 8
gdt:
// 0 segment
.long 0, 0

// code (0x8)
.word 0xffff	// limit 15:0
.word 0x0	// base 15:0
.byte 0x0	// base 23:16
.byte 0x9b	// present, ring 0, executable, !conforming, readable, accessed
.byte 0xcf	// granularity size, limit[19:16] = f
.byte 0x0	// base 31:24

// data (0x10)
.word 0xffff	// limit 15:0
.word 0x0	// base 15:0
.byte 0x0	// base 23:16
.byte 0x93	// present, priv=0, !executable, stack down, writable, accessed
.byte 0xcf	// granularity=1 size=1, limit 19:16 = f
.byte 0x0	// base 31:24
egdt:
