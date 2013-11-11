// dwarf
// strategy: bomb memory at regular intervals

loop:   *addr = 0
        addr += 4    // bomb every 4 locations
        jmp loop
addr:   .DAT 4       // start bombing immediately after ourself

