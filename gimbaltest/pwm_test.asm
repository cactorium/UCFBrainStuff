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

  ; allocate space for variables here
  org 0x0200
a_count: 
  dw 1395
b_count:
  dw 1302

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

  ; set up the transmit interrupt
  ;bis.b #UCA0RXIE, &IE2
  bis.b #UCA0RXIE, &IE2
  mov.b #'A', &UCA0TXBUF
  ; turn interrupts back on
  eint

  clr.w r4
main: ; 20 clock cycle, so each loop increments r4 in 20/16 MHz = 1.25us
      ; well, it should be. It's a little off. 16000 loops take 17.2ms,
      ; so each loop takes 17.2ms/16000 = 1.075 us
  mov.w r4, r5 ; +1
  mov.w r5, r4 ; +1
  add.w #1, r4 ; +3
  cmp.w &a_count, r4 ; +3
  jne skip_a ; +2

low_a:
  bic.b #pwm_a, &pwm_a_reg
skip_a:

  cmp.w &b_count, r4 ; +3
  jne skip_b ; +2

low_b:
  bic.b #pwm_b, &pwm_b_reg
skip_b:

  ;cmp.w #16000, r4 ; +3 ; 20ms/1.25us = 16000. 
  cmp.w #18605, r4 ; +3 ; 20ms/1.075us = 18605. 
  jl main ; +2

pwm_high:
  clr.w r4
  bis.b #pwm_a, &pwm_a_reg
  bis.b #pwm_b, &pwm_b_reg
  jmp main

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
  dw 0
  dw 0
  dw 0
  dw 0
  dw 0
  dw 0
  dw 0
  dw start                 ; Reset
