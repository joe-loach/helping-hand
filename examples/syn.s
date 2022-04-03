; list of all instructions and their correct syntax
label:
ADC     R0, #1
ADC     R0, R0, #1
ADC     R0, R0, R1
ADC     R0, R1, RRX
ADC     R0, R0, R1, RRX
ADC     R0, R0, R1, LSL #1
ADC     R0, R1, LSL R2
ADC     R0, R0, R1, LSL R2
ADR     R0, label
B       label
BEQ     label
BNE     label
BCS     label
BCC     label
BMI     label
BPL     label
BVS     label
BVC     label
BHI     label
BLS     label
BGE     label
BLT     label
BGT     label
BLE     label
BAL     label
BHS     label
BLO     label
BFC     R0, #1, #31
BFI     R0, R1, #1, #31
BL      label
BX      R0
CLZ     R0, R0
CMN     R0, #1
CMN     R0, R1, RRX
CMN     R0, R1, LSL #1
CMN     R0, R1, LSL R2
LDA     R0, [R1]
LDAB    R0, [R1]
LDAH    R0, [R1]
LDM     SP!, {R0, R1, R2}
LDR     R0, [R0, #1]
LDR     R0, label
LDR     R0, [PC, #1]
LDR     R0, [R0, R1, LSL #1]
LDR     R0, [R0], R1, LSL #1
LDR     R0, [R0, R1, LSL #1]!
LSL     R0, #1
LSL     R0, R0, #1
MLA     R0, R1, R2, R3
MOV     R0, #1
MOV     R0, R1, RRX
MOV     R0, R1, LSL #1
MOV     R0, R1, LSL R2
MOVT    R0, #10
NOP
POP     {R0}
POP     {R0, R1}
PUSH    {R0}
PUSH    {R0, R1}
STM     SP, {R1, R2}
STM     SP!, {R1, R2}
STR     R0, [R0]
STR     R0, [R0, #1]
STR     R0, [R0], #1
STR     R0, [R0, #1]!
STR     R0, [R0, R1, LSL #1]
STR     R0, [R0], R1
STR     R0, [R0, R1]!
STRD    R0, R1, [R2]
STRD    R0, R1, [R2, #1]
STRD    R0, R1, [R2], #1
STRD    R0, R1, [R2, #1]!
STRD    R0, R1, [R2, R3]
STRD    R0, R1, [R2], R3
STRD    R0, R1, [R2, R4]!
STRH    R0, [R1]
STRH    R0, [R1, #1]
STRH    R0, [R1], #1
STRH    R0, [R1, #1]!
STRH    R0, [R1, R2]
STRH    R0, [R1], R2
STRH    R0, [R1, R2]!