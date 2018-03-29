MEMORY
{
    RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 20K
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 64K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM); 
_heap_size = 1024; 
