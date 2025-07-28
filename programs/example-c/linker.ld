OUTPUT_FORMAT(elf64-x86-64)

ENTRY(_start)

PHDRS
{
    text_data PT_LOAD FLAGS(5);
}

SECTIONS
{
  . = 0x400000;
  .text : {
    *(.text .text.*)
  } :text_data

  .data : {
    *(.data .data.*)
  } :text_data
  
  .rodata : {
        *(.rodata .rodata.*)
    } :text_data
     

  .bss : {
    *(.bss .bss.*)
  } :text_data
  
  /DISCARD/ : {
    *(.note .note*)
    *(.note.GNU-stack)
  }
}
