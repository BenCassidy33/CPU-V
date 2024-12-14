.section .data:
    dword big_number 340_282_366_920_938_463_463_374_607_431_768_211_455
    byte small_number 1
    str8 lang "rust"
    str16 name "assembly"

.section .program:
_start:
    LDA #10
    STA $0200
    LDA #20
    STA $0201
    JMP _jump

_jump:
    LDA $0200
    ADC $0201
    TAX

    LDA #0
    TAY

    JMP .start
