.section .data:

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
