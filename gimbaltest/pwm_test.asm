.include "msp430x2xx.inc"

CALBC1_16MHZ equ 0x10F9 ; address where the clock calibration data's stored
CALDCO_16MHZ equ 0x10F8 ; address where the clock calibration data's stored

GIE equ 0x08

TA0CTL equ 0x0160
TA0CCTL0 equ 0x162
TA0CCR0 equ 0x0172

TA0CCTL1 equ 0x164
TA0CCR1 equ 0x0174

TA0CCTL2 equ 0x166
TA0CCR2 equ 0x0176

pwm_a equ 0x40
pwm_a_reg equ P1OUT
pwm_b equ 0x80
pwm_b_reg equ P1OUT

  org 0x2000
  ; allocate space for variables here

  org 0xf800
start:
  ; turn off the watchdog timer
  mov.w #WDTPW|WDTHOLD, &WDTCTL

  ; set the clock to 16 MHz
  ; SMCLK, used by the timer modules, SHOULD be pointing at DCO as well
  clr.b &DCOCTL
  mov.b &CALBC1_16MHZ, &BCSCTL1
  mov.b &CALDCO_16MHZ, &DCOCTL

  ; TODO: configure the pins for PWM and UART
  mov.b #0xff, &P1DIR
  mov.b #0xff, &P2DIR
  mov.w #0x01, r8

  ; turn off interrupts while setting up the timers
  bic.w #GIE, SR
  ; set up timer
  mov.w #TASSEL_2|ID_3|MC_1|TAIE, &TA0CTL ; 8x clock divider, 2 MHz tick rate
  mov.w #CCIE, &TA0CCTL0 ; 8x clock divider, 2 MHz tick rate
  mov.w #CM_0|CCIE, &TA0CCTL1
  mov.w #3000, &TA0CCR1 ; 1.5 ms * 2 MHz = 3000
  mov.w #CM_0|CCIE, &TA0CCTL2
  ; mov.w #2400, &TA0CCR2 ; 1.2 ms * 2 MHz = 2400
  mov.w #2900, &TA0CCR2 ; 1.2 ms * 2 MHz = 2400

  ; start the timer
  mov.w #40000, &TA0CCR0  ; 20 ms * 2 MHz = 40000

  ; turn interrupts back on
  bis.w #GIE, SR
  ; TODO: set the MCU to sleep instead of infinite looping
repeat:
  jmp repeat

timer: ; 6 clocks to enter the interrupt
  bis.b #pwm_a, &pwm_a_reg
  bis.b #pwm_b, &pwm_b_reg
  reti ; +5

timer2: ; 6 clocks to enter the interrupt
  mov.w &TA0CCTL1, r4
  and.w #CCIFG, r4
  jz timer2_L0
  bic.w #CCIFG, &TA0CCTL1
  bic.b #pwm_a, &pwm_a_reg
timer2_L0:
  mov.w &TA0CCTL2, r4
  and.w #CCIFG, r4
  jz timer2_L1
  bic.w #CCIFG, &TA0CCTL2
  bic.b #pwm_b, &pwm_b_reg
timer2_L1:
  reti ; + 5

  org 0xffe8
vectors:
  dw 0
  dw 0
  dw 0
  dw 0
  dw timer2                 ; Timer0_A3: TA0CCR2, TA0CCR1
  dw timer                  ; Timer0_A3: TA0CCR0
  dw 0
  dw 0
  dw 0
  dw 0
  dw 0
  dw start                 ; Reset
