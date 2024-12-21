Instructions:
    EXTERN, // For loading functions from outside of the program,
    .section,
        .data
        .program

    LOAD VALUE (into) Register,
    MOV (from) Register (to) Register, // sets inital register to 0 or nothing of its data type
    INC Register,
    DEC Register,
    SET Register,
    CAL Calls a external function that has been loaded before the program has started,

    ADD,
    SUB,
    CMP first second,

    JMP // Jumps to a label,
    JEQ, // Jumps if the both argumens passed to CMP are equal
    JLT, // Jumps if the first argument of CMP is less than the second
    JGT, // Jumps if the first argument of CMP is greater than the second

    NOP,
    BRK,
    EXT code, // exits the program with an exit code
