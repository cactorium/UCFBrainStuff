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

TA1CCR2 equ 0x196
TA1CCR1 equ 0x194
TA1CCR0 equ 0x192
TA1R equ 0x190
TA1CCTL2 equ 0x186
TA1CCTL1 equ 0x184
TA1CCTL0 equ 0x182
TA1CTL equ 0x180
TA1IV equ 0x11E

pwm_a equ 0x40
pwm_a_reg equ P1OUT
pwm_b equ 0x80
pwm_b_reg equ P1OUT

  org 0x2000
  ; allocate space for variables here

  org 0xf800
start:
  mov.w #0x0280, sp
  ; turn off the watchdog timer
  mov.w #WDTPW|WDTHOLD, &WDTCTL

  ;; Set MCLK to 16 MHz with DCO 
  ;mov.b #(DCO_4), &DCOCTL
  ;mov.b #RSEL_15, &BCSCTL1
  ;mov.b #0, &BCSCTL2

  clr.b &DCOCTL
  mov.b &CALBC1_16MHZ, &BCSCTL1
  mov.b &CALDCO_16MHZ, &DCOCTL


  ; configure the pins for PWM and UART
  bis.b #0xf5, &P1DIR
  ;bis.b #0x01, &P1OUT
  mov.b #0xff, &P2DIR
  mov.b #6, &P1SEL
  mov.b #6, &P1SEL2

  ; turn off interrupts while setting up the timers and UART
  dint
  jmp new_uart_conf

old_uart_conf:
  ;; Setup UART
  mov.b #UCSSEL_2|UCSWRST, &UCA0CTL1
  mov.b #0, &UCA0CTL0
  ;mov.b #0x82, &UCA0BR0
  ;mov.b #0x06, &UCA0BR1
  mov.b #0xf8, &UCA0BR0  ; 9600 baud
  mov.b #0x05, &UCA0BR1
  bic.b #UCSWRST, &UCA0CTL1
  jmp uart_conf_end

new_uart_conf:
  mov.b #UCSSEL_2|UCSWRST, &UCA0CTL1
  bis.b #0, &UCA0CTL0
  mov.b #104, &UCA0BR0
  mov.b #0, &UCA0BR1
  mov.b #48|UCOS16, &UCA0MCTL
  bic.b #UCSWRST, &UCA0CTL1
no_uart_conf:
uart_conf_end:
  jmp new_timer_conf

old_timer_conf:
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
  jmp timer_conf_end

new_timer_conf:
  mov.w #TASSEL_2|ID_3|MC_1|TAIE, &TA1CTL ; 8x clock divider, 2 MHz tick rate
  mov.w #CCIE, &TA1CCTL0 ; 8x clock divider, 2 MHz tick rate
  mov.w #CM_0|CCIE, &TA1CCTL1
  mov.w #3000, &TA1CCR1 ; 1.5 ms * 2 MHz = 3000
  mov.w #CM_0|CCIE, &TA1CCTL2
  ; mov.w #2400, &TA1CCR2 ; 1.2 ms * 2 MHz = 2400
  mov.w #2900, &TA1CCR2 ; 1.45 ms * 2 MHz = 2900

  ; start the timer
  mov.w #40000, &TA1CCR0  ; 20 ms * 2 MHz = 40000
no_timer_conf:
timer_conf_end:

  ; set up the transmit interrupt
  ;bis.b #UCA0RXIE, &IE2
  bis.b #UCA0RXIE, &IE2
  mov.b #'A', &UCA0TXBUF

  ; turn interrupts back on
  eint
  ; TODO: set the MCU to sleep instead of infinite looping

  jmp repeat
  ; send a bunch of A's

  mov #'A', r7
  mov.b r7, &UCA0TXBUF

main:
  bit.b #UCA0TXIFG, &IFG2
  jz main
  mov.b r7, &UCA0TXBUF
  inc r7
  jmp main


  mov.b #'A', r4
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

timer_2: ; 6 clocks to enter the interrupt
  bis.b #pwm_a, &pwm_a_reg
  bis.b #pwm_b, &pwm_b_reg
  reti ; +5

timer2_2: ; 6 clocks to enter the interrupt
  mov.w &TA1CCTL1, r4
  and.w #CCIFG, r4
  jz timer2_2_L0
  bic.w #CCIFG, &TA1CCTL1
  bic.b #pwm_a, &pwm_a_reg
timer2_2_L0:
  mov.w &TA1CCTL2, r4
  and.w #CCIFG, r4
  jz timer2_2_L1
  bic.w #CCIFG, &TA1CCTL2
  bic.b #pwm_b, &pwm_b_reg
timer2_2_L1:
  reti ; + 5


uart_rcv:
  push r4
  mov.b &UCA0RXBUF, r4
uart_rcv_L0:
  bit.b #UCA0TXIFG, &IFG2
  jz uart_rcv_L0
  mov.b r4, &UCA0TXBUF
  pop r4
  reti

uart_send:
  reti

  org 0xffe8
vectors:
  dw 0
  dw 0
  dw uart_send
  dw uart_rcv
  dw timer2                 ; Timer0_A3: TA0CCR2, TA0CCR1
  dw timer                  ; Timer0_A3: TA0CCR0
  dw 0
  dw 0
  dw timer2_2
  dw timer_2
  dw 0
  dw start                 ; Reset
