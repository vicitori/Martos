/*
 * SPDX-FileCopyrightText: 2021-2024 Espressif Systems (Shanghai) CO LTD
 *
 * SPDX-License-Identifier: Apache-2.0
 */
/** Simplified memory map for the bootloader.
 *  Make sure the bootloader can load into main memory without overwriting itself.
 *
 *  ESP32-C6 ROM static data usage is as follows:
 *  - 0x4086ad08 - 0x4087c610: Shared buffers, used in UART/USB/SPI download mode only
 *  - 0x4087c610 - 0x4087e610: PRO CPU stack, can be reclaimed as heap after RTOS startup
 *  - 0x4087e610 - 0x40880000: ROM .bss and .data (not easily reclaimable)
 *
 *  The 2nd stage bootloader can take space up to the end of ROM shared
 *  buffers area (0x4087c610).
 */

/* We consider 0x4087c610 to be the last usable address for 2nd stage bootloader stack overhead, dram_seg,
 * and work out iram_seg and iram_loader_seg addresses from there, backwards.
 */

/* These lengths can be adjusted, if necessary: */
bootloader_usable_dram_end = 0x4087E610;
bootloader_stack_overhead = 0x2000; /* For safety margin between bootloader data section and startup stacks */
bootloader_dram_seg_len = 0xE000;
bootloader_iram_loader_seg_len = 0x7000;
bootloader_iram_seg_len = 0x4800;

/* Start of the lower region is determined by region size and the end of the higher region */
bootloader_dram_seg_end = bootloader_usable_dram_end - bootloader_stack_overhead;
bootloader_dram_seg_start = bootloader_dram_seg_end - bootloader_dram_seg_len - 0x1000;
bootloader_iram_loader_seg_start = 0x40866BD8;
bootloader_iram_seg_start = bootloader_iram_loader_seg_start - bootloader_iram_seg_len;

MEMORY
{
  iram_seg (RWX) :                  org = bootloader_iram_seg_start, len = bootloader_iram_seg_len
  iram_loader_seg (RWX) :           org = bootloader_iram_loader_seg_start, len = bootloader_iram_loader_seg_len
  dram_seg (RW) :                   org = bootloader_dram_seg_start, len = bootloader_dram_seg_len + 0x38AC
  lp_ram_seg (RW)  :                org = 0x50000000, len = 0x4000
}

/* The app may use RAM for static allocations up to the start of iram_loader_seg.
 * If you have changed something above and this assert fails:
 * 1. Check what the new value of bootloader_iram_loader_seg start is.
 * 2. Update the value in this assert.
 * 3. Update SRAM_DRAM_END in components/esp_system/ld/esp32c6/memory.ld.in to the same value.
 */

ASSERT(bootloader_iram_loader_seg_start == 0x40866BD8, "bootloader_iram_loader_seg_start inconsistent with SRAM_DRAM_END");

/* Default entry point: */
ENTRY(call_start_cpu0);

SECTIONS
{
    .iram_loader.text :
  {
    . = ALIGN (16);
    _loader_text_start = ABSOLUTE(.);
    *(.stub .gnu.warning .gnu.linkonce.literal.* .gnu.linkonce.t.*.literal .gnu.linkonce.t.*)
    *(.iram1 .iram1.*)
    *(.fini.literal)
    *(.fini)
    *(.gnu.version)
    _loader_text_end = ABSOLUTE(.);
  } > iram_loader_seg

  .rtc.text :
  {
    . = ALIGN(4);
    _rtc_fast_start = ABSOLUTE(.);
    _rtc_text_start = ABSOLUTE(.);
    *(.rtc.entry.text)

    *rtc_wake_stub*.*(.literal .text .literal.* .text.*)
    *(.rtc_text_end_test)

    . = ALIGN(4);

    _rtc_text_end = ABSOLUTE(.);
  } > lp_ram_seg

  .rtc.force_fast :
  {
    . = ALIGN(4);
    _rtc_force_fast_start = ABSOLUTE(.);

    mapping[rtc_force_fast]

    *(.rtc.force_fast .rtc.force_fast.*)
    . = ALIGN(4) ;
    _rtc_force_fast_end = ABSOLUTE(.);
  } > lp_ram_seg

  .rtc.data :
  {
    _rtc_data_start = ABSOLUTE(.);

    mapping[rtc_data]

    *rtc_wake_stub*.*(.data .rodata .data.* .rodata.* .srodata.*)
    _rtc_data_end = ABSOLUTE(.);
  } > lp_ram_seg

  .rtc.bss (NOLOAD) :
  {
    _rtc_bss_start = ABSOLUTE(.);
    *rtc_wake_stub*.*(.bss .bss.* .sbss .sbss.*)
    *rtc_wake_stub*.*(COMMON)

    mapping[rtc_bss]

    _rtc_bss_end = ABSOLUTE(.);
  } > lp_ram_seg

  .rtc_noinit (NOLOAD):
  {
    . = ALIGN(4);
    _rtc_noinit_start = ABSOLUTE(.);
    *(.rtc_noinit .rtc_noinit.*)
    . = ALIGN(4) ;
    _rtc_noinit_end = ABSOLUTE(.);
  } > lp_ram_seg

  .rtc.force_slow :
  {
    . = ALIGN(4);
    _rtc_force_slow_start = ABSOLUTE(.);
    *(.rtc.force_slow .rtc.force_slow.*)
    . = ALIGN(4) ;
    _rtc_force_slow_end = ABSOLUTE(.);
  } > lp_ram_seg

  .iram.text :
  {
    . = ALIGN (16);
    *(.entry.text)
    *(.init.literal)
    *(.init)
  } > iram_seg


  /* Shared RAM */
  .dram0.bss (NOLOAD) :
  {
    . = ALIGN (8);
    _bss_start = ABSOLUTE(.);
    _dram_start = ABSOLUTE(.);
    *(.dynsbss)
    *(.sbss)
    *(.sbss.*)
    *(.gnu.linkonce.sb.*)
    *(.scommon)
    *(.sbss2)
    *(.sbss2.*)
    *(.gnu.linkonce.sb2.*)
    *(.dynbss)
    *(.bss)
    *(.bss.*)
    *(.gnu.linkonce.b.*)
    *(COMMON)
    . = ALIGN (8);
    _bss_end = ABSOLUTE(.);
  } > dram_seg

  .dram0.data :
  {
    *(.dram1 .dram1.*) /* catch stray DRAM_ATTR */
    *(.data)
    *(.data.*)
    *(.gnu.linkonce.d.*)
    *(.data1)
    *(.sdata)
    *(.sdata.*)
    *(.gnu.linkonce.s.*)
    *(.gnu.linkonce.s2.*)
    *(.jcr)
  } > dram_seg

  .dram0.rodata :
  {
    _rodata_start = ABSOLUTE(.);
    *(.rodata)
    *(.rodata.*)
    *(.gnu.linkonce.r.*)
    *(.rodata1)
    *(.sdata2 .sdata2.* .srodata .srodata.*)
    __XT_EXCEPTION_TABLE_ = ABSOLUTE(.);
    . = ALIGN(4);
    _dram_end = ABSOLUTE(.);
  } > dram_seg

  .iram.text :
  {
    _stext = .;
    _text_start = ABSOLUTE(.);
    *(.literal .text .literal.* .text.* .stub .gnu.warning .gnu.linkonce.literal.* .gnu.linkonce.t.*.literal .gnu.linkonce.t.*)
    *(.iram .iram.*) /* catch stray IRAM_ATTR */
    *(.fini.literal)
    *(.fini)
    *(.gnu.version)

    /** CPU will try to prefetch up to 16 bytes of
      * of instructions. This means that any configuration (e.g. MMU, PMS) must allow
      * safe access to up to 16 bytes after the last real instruction, add
      * dummy bytes to ensure this
      */
    . += 16;

    _text_end = ABSOLUTE(.);
    _etext = .;
  } > iram_seg

  .riscv.attributes 0: { *(.riscv.attributes) }
}

INCLUDE "ld/esp32c6.rom.ld"
PROVIDE (ets_update_cpu_frequency_rom = ets_update_cpu_frequency);
