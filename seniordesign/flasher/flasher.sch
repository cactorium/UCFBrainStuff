EESchema Schematic File Version 2
LIBS:flasher-rescue
LIBS:power
LIBS:device
LIBS:transistors
LIBS:conn
LIBS:linear
LIBS:regul
LIBS:74xx
LIBS:cmos4000
LIBS:adc-dac
LIBS:memory
LIBS:xilinx
LIBS:special
LIBS:microcontrollers
LIBS:dsp
LIBS:microchip
LIBS:analog_switches
LIBS:motorola
LIBS:texas
LIBS:intel
LIBS:audio
LIBS:interface
LIBS:digital-audio
LIBS:philips
LIBS:display
LIBS:cypress
LIBS:siliconi
LIBS:opto
LIBS:atmel
LIBS:contrib
LIBS:valves
LIBS:custom-parts
LIBS:tivafpgacore-cache
LIBS:flasher-cache
EELAYER 25 0
EELAYER END
$Descr User 5906 7890
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L +5V #PWR01
U 1 1 553787AB
P 1500 850
F 0 "#PWR01" H 1500 700 50  0001 C CNN
F 1 "+5V" H 1500 990 50  0000 C CNN
F 2 "" H 1500 850 60  0000 C CNN
F 3 "" H 1500 850 60  0000 C CNN
	1    1500 850 
	1    0    0    -1  
$EndComp
$Comp
L ATTINY24-SS IC1
U 1 1 55379930
P 2650 1550
F 0 "IC1" H 1800 2300 40  0000 C CNN
F 1 "ATTINY24-SS" H 3350 800 40  0000 C CNN
F 2 "Housings_SOIC:SOIC-14_3.9x8.7mm_Pitch1.27mm" H 2650 1350 35  0000 C CIN
F 3 "" H 2650 1550 60  0000 C CNN
	1    2650 1550
	1    0    0    -1  
$EndComp
$Comp
L CONN_01X04 P2
U 1 1 5537A0D3
P 5150 2150
F 0 "P2" H 5150 2400 50  0000 C CNN
F 1 "GPIO" V 5250 2150 50  0000 C CNN
F 2 "Wire_Pads:SolderWirePad_4xInline_1-2mmDrill" H 5150 2150 60  0001 C CNN
F 3 "" H 5150 2150 60  0000 C CNN
	1    5150 2150
	1    0    0    -1  
$EndComp
$Comp
L R-RESCUE-flasher R3
U 1 1 5537B349
P 2000 2850
F 0 "R3" V 2080 2850 50  0000 C CNN
F 1 "90" V 2000 2850 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 1930 2850 30  0001 C CNN
F 3 "" H 2000 2850 30  0000 C CNN
	1    2000 2850
	1    0    0    -1  
$EndComp
$Comp
L R-RESCUE-flasher R1
U 1 1 5537B59C
P 1400 3350
F 0 "R1" V 1480 3350 50  0000 C CNN
F 1 "0" V 1400 3350 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 1330 3350 30  0001 C CNN
F 3 "" H 1400 3350 30  0000 C CNN
	1    1400 3350
	0    1    1    0   
$EndComp
$Comp
L R-RESCUE-flasher R7
U 1 1 5537BF03
P 4000 2850
F 0 "R7" V 4080 2850 50  0000 C CNN
F 1 "90" V 4000 2850 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 3930 2850 30  0001 C CNN
F 3 "" H 4000 2850 30  0000 C CNN
	1    4000 2850
	1    0    0    -1  
$EndComp
$Comp
L R-RESCUE-flasher R4
U 1 1 5537BF69
P 2000 4250
F 0 "R4" V 2080 4250 50  0000 C CNN
F 1 "90" V 2000 4250 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 1930 4250 30  0001 C CNN
F 3 "" H 2000 4250 30  0000 C CNN
	1    2000 4250
	1    0    0    -1  
$EndComp
$Comp
L R-RESCUE-flasher R8
U 1 1 5537BFC1
P 4000 4250
F 0 "R8" V 4080 4250 50  0000 C CNN
F 1 "90" V 4000 4250 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 3930 4250 30  0001 C CNN
F 3 "" H 4000 4250 30  0000 C CNN
	1    4000 4250
	1    0    0    -1  
$EndComp
$Comp
L +5V #PWR02
U 1 1 5537C027
P 4800 1850
F 0 "#PWR02" H 4800 1700 50  0001 C CNN
F 1 "+5V" H 4800 1990 50  0000 C CNN
F 2 "" H 4800 1850 60  0000 C CNN
F 3 "" H 4800 1850 60  0000 C CNN
	1    4800 1850
	1    0    0    -1  
$EndComp
$Comp
L CONN_01X06 P1
U 1 1 5537C856
P 5150 1050
F 0 "P1" H 5150 1400 50  0000 C CNN
F 1 "PROG_PINS" V 5250 1050 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_1x06" H 5150 1050 60  0001 C CNN
F 3 "" H 5150 1050 60  0000 C CNN
	1    5150 1050
	1    0    0    -1  
$EndComp
$Comp
L +5V #PWR03
U 1 1 5537CB19
P 4800 750
F 0 "#PWR03" H 4800 600 50  0001 C CNN
F 1 "+5V" H 4800 890 50  0000 C CNN
F 2 "" H 4800 750 60  0000 C CNN
F 3 "" H 4800 750 60  0000 C CNN
	1    4800 750 
	1    0    0    -1  
$EndComp
$Comp
L +5V #PWR04
U 1 1 5537CC95
P 4300 700
F 0 "#PWR04" H 4300 550 50  0001 C CNN
F 1 "+5V" H 4300 840 50  0000 C CNN
F 2 "" H 4300 700 60  0000 C CNN
F 3 "" H 4300 700 60  0000 C CNN
	1    4300 700 
	1    0    0    -1  
$EndComp
$Comp
L R-RESCUE-flasher R9
U 1 1 5537CCE6
P 4300 1000
F 0 "R9" V 4380 1000 50  0000 C CNN
F 1 "33k" V 4300 1000 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 4230 1000 30  0001 C CNN
F 3 "" H 4300 1000 30  0000 C CNN
	1    4300 1000
	1    0    0    -1  
$EndComp
Entry Wire Line
	3750 950  3850 1050
Entry Wire Line
	3750 1050 3850 1150
Entry Wire Line
	3750 1150 3850 1250
Entry Wire Line
	3750 1250 3850 1350
Entry Wire Line
	1000 3250 1100 3350
Entry Wire Line
	1000 4600 1100 4700
Text Label 1100 3350 0    60   ~ 0
LED0
Text Label 3700 950  0    60   ~ 0
LED0
Text Label 3700 1050 0    60   ~ 0
LED1
Text Label 3700 1150 0    60   ~ 0
LED2
Text Label 3700 1250 0    60   ~ 0
LED3
Entry Wire Line
	2900 3250 3000 3350
Entry Wire Line
	2900 4600 3000 4700
Entry Wire Line
	1900 2600 2000 2700
Entry Wire Line
	3900 2600 4000 2700
Entry Wire Line
	1900 4000 2000 4100
Entry Wire Line
	3900 4000 4000 4100
Text Label 3000 3350 0    60   ~ 0
LED1
Text Label 1100 4700 0    60   ~ 0
LED2
Text Label 3050 4700 0    60   ~ 0
LED3
Text Label 2000 2700 0    60   ~ 0
OUT0
Text Label 4000 2700 0    60   ~ 0
OUT1
Text Label 2000 4100 0    60   ~ 0
OUT2
Text Label 4000 4100 0    60   ~ 0
OUT3
$Comp
L BC847S Q1
U 1 1 55380003
P 1900 3350
F 0 "Q1" H 2100 3425 50  0000 L CNN
F 1 "BC847S" H 2100 3350 50  0000 L CNN
F 2 "custom_footprints:SOT-363" H 2100 3275 50  0000 L CIN
F 3 "" H 1900 3350 50  0000 L CNN
	1    1900 3350
	1    0    0    -1  
$EndComp
$Comp
L BC847S Q2
U 1 1 5538008D
P 3900 3350
F 0 "Q2" H 4100 3425 50  0000 L CNN
F 1 "BC847S" H 4100 3350 50  0000 L CNN
F 2 "custom_footprints:SOT-363" H 4100 3275 50  0000 L CIN
F 3 "" H 3900 3350 50  0000 L CNN
	1    3900 3350
	1    0    0    -1  
$EndComp
$Comp
L BC847S Q2
U 2 1 55380108
P 3900 4700
F 0 "Q2" H 4100 4775 50  0000 L CNN
F 1 "BC847S" H 4100 4700 50  0000 L CNN
F 2 "custom_footprints:SOT-363" H 4100 4625 50  0000 L CIN
F 3 "" H 3900 4700 50  0000 L CNN
	2    3900 4700
	1    0    0    -1  
$EndComp
$Comp
L BC847S Q1
U 2 1 55380171
P 1900 4700
F 0 "Q1" H 2100 4775 50  0000 L CNN
F 1 "BC847S" H 2100 4700 50  0000 L CNN
F 2 "custom_footprints:SOT-363" H 2100 4625 50  0000 L CIN
F 3 "" H 1900 4700 50  0000 L CNN
	2    1900 4700
	1    0    0    -1  
$EndComp
$Comp
L CONN_01X04 P3
U 1 1 553803F0
P 5200 3350
F 0 "P3" H 5200 3600 50  0000 C CNN
F 1 "OUT" V 5300 3350 50  0000 C CNN
F 2 "Wire_Pads:SolderWirePad_4xInline_2mmDrill" H 5200 3350 60  0001 C CNN
F 3 "" H 5200 3350 60  0000 C CNN
	1    5200 3350
	1    0    0    -1  
$EndComp
Entry Wire Line
	4700 3200 4800 3300
Entry Wire Line
	4700 3300 4800 3400
Entry Wire Line
	4700 3400 4800 3500
Entry Wire Line
	4700 3100 4800 3200
Text Label 4800 3200 0    60   ~ 0
OUT0
Text Label 4800 3300 0    60   ~ 0
OUT1
Text Label 4800 3400 0    60   ~ 0
OUT2
Text Label 4800 3500 0    60   ~ 0
OUT3
$Comp
L R-RESCUE-flasher R2
U 1 1 55381214
P 1400 4700
F 0 "R2" V 1480 4700 50  0000 C CNN
F 1 "0" V 1400 4700 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 1330 4700 30  0001 C CNN
F 3 "" H 1400 4700 30  0000 C CNN
	1    1400 4700
	0    1    1    0   
$EndComp
$Comp
L R-RESCUE-flasher R6
U 1 1 5538127C
P 3400 4700
F 0 "R6" V 3480 4700 50  0000 C CNN
F 1 "0" V 3400 4700 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 3330 4700 30  0001 C CNN
F 3 "" H 3400 4700 30  0000 C CNN
	1    3400 4700
	0    1    1    0   
$EndComp
$Comp
L R-RESCUE-flasher R5
U 1 1 553812FA
P 3400 3350
F 0 "R5" V 3480 3350 50  0000 C CNN
F 1 "0" V 3400 3350 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 3330 3350 30  0001 C CNN
F 3 "" H 3400 3350 30  0000 C CNN
	1    3400 3350
	0    1    1    0   
$EndComp
$Comp
L C_Small C2
U 1 1 55382179
P 1500 1100
F 0 "C2" H 1510 1170 50  0000 L CNN
F 1 "0.1u" H 1510 1020 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805" H 1500 1100 60  0001 C CNN
F 3 "" H 1500 1100 60  0000 C CNN
	1    1500 1100
	1    0    0    -1  
$EndComp
$Comp
L C_Small C1
U 1 1 55382206
P 900 1100
F 0 "C1" H 910 1170 50  0000 L CNN
F 1 "10u" H 910 1020 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805" H 900 1100 60  0001 C CNN
F 3 "" H 900 1100 60  0000 C CNN
	1    900  1100
	1    0    0    -1  
$EndComp
$Comp
L +5V #PWR05
U 1 1 553829BA
P 900 850
F 0 "#PWR05" H 900 700 50  0001 C CNN
F 1 "+5V" H 900 990 50  0000 C CNN
F 2 "" H 900 850 60  0000 C CNN
F 3 "" H 900 850 60  0000 C CNN
	1    900  850 
	1    0    0    -1  
$EndComp
Wire Wire Line
	1500 850  1500 1000
Wire Wire Line
	1500 950  1600 950 
Wire Wire Line
	1600 2150 1500 2150
Wire Wire Line
	1500 2150 1500 2250
Wire Wire Line
	2000 3000 2000 3150
Wire Wire Line
	1700 3350 1550 3350
Wire Wire Line
	2000 3550 2000 3650
Wire Wire Line
	2000 4400 2000 4500
Wire Wire Line
	2000 4900 2000 5000
Wire Wire Line
	1700 4700 1550 4700
Wire Wire Line
	3550 4700 3700 4700
Wire Wire Line
	4000 4400 4000 4500
Wire Wire Line
	4000 4900 4000 5000
Wire Wire Line
	4000 3550 4000 3650
Wire Wire Line
	3700 3350 3550 3350
Wire Wire Line
	4000 3150 4000 3000
Wire Wire Line
	4950 2000 4800 2000
Wire Wire Line
	4800 2000 4800 1850
Wire Wire Line
	4950 2300 4800 2300
Wire Wire Line
	4800 2300 4800 2450
Wire Wire Line
	4950 2100 4550 2100
Wire Wire Line
	4550 2100 4550 1850
Wire Wire Line
	4550 1850 3700 1850
Wire Wire Line
	3700 1950 4500 1950
Wire Wire Line
	4500 1950 4500 2200
Wire Wire Line
	4500 2200 4950 2200
Wire Wire Line
	4800 750  4800 800 
Wire Wire Line
	4800 800  4950 800 
Wire Wire Line
	4800 1400 4800 1300
Wire Wire Line
	4800 1300 4950 1300
Wire Wire Line
	4950 900  4450 900 
Wire Wire Line
	4450 900  4450 1350
Wire Wire Line
	4450 1350 3700 1350
Wire Wire Line
	3700 1450 4500 1450
Wire Wire Line
	4500 1450 4500 1000
Wire Wire Line
	4500 1000 4950 1000
Wire Wire Line
	4950 1100 4550 1100
Wire Wire Line
	4550 1100 4550 1550
Wire Wire Line
	4550 1550 3700 1550
Wire Wire Line
	3700 2150 4600 2150
Wire Wire Line
	4600 2150 4600 1200
Wire Wire Line
	4300 1200 4950 1200
Wire Wire Line
	4300 850  4300 700 
Wire Wire Line
	4300 1150 4300 1200
Connection ~ 4600 1200
Wire Bus Line
	3850 1000 3850 2500
Wire Bus Line
	3850 2500 1000 2500
Wire Bus Line
	1000 2500 1000 5400
Wire Bus Line
	1000 5400 2900 5400
Wire Bus Line
	2900 3100 2900 3350
Wire Wire Line
	3700 950  3750 950 
Wire Wire Line
	3700 1050 3750 1050
Wire Wire Line
	3700 1150 3750 1150
Wire Wire Line
	3700 1250 3750 1250
Wire Wire Line
	1250 4700 1100 4700
Wire Wire Line
	1250 3350 1100 3350
Wire Wire Line
	3000 3350 3250 3350
Wire Bus Line
	2900 5400 2900 3050
Wire Wire Line
	3250 4700 3000 4700
Wire Bus Line
	1900 2600 4700 2600
Wire Bus Line
	4700 2600 4700 4000
Wire Bus Line
	4700 4000 1900 4000
Wire Bus Line
	1900 4000 1900 3950
Wire Wire Line
	5000 3500 4800 3500
Wire Wire Line
	4800 3400 5000 3400
Wire Wire Line
	5000 3300 4800 3300
Wire Wire Line
	4800 3200 5000 3200
Wire Wire Line
	900  1200 900  1300
Wire Wire Line
	1500 1200 1500 1300
Connection ~ 1500 950 
Wire Wire Line
	900  1000 900  850 
$Comp
L GND #PWR06
U 1 1 55D8E016
P 4000 5000
F 0 "#PWR06" H 4000 5000 30  0001 C CNN
F 1 "GND" H 4000 4930 30  0001 C CNN
F 2 "" H 4000 5000 60  0000 C CNN
F 3 "" H 4000 5000 60  0000 C CNN
	1    4000 5000
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR07
U 1 1 55D8E14F
P 2000 5000
F 0 "#PWR07" H 2000 5000 30  0001 C CNN
F 1 "GND" H 2000 4930 30  0001 C CNN
F 2 "" H 2000 5000 60  0000 C CNN
F 3 "" H 2000 5000 60  0000 C CNN
	1    2000 5000
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR08
U 1 1 55D8E1EA
P 2000 3650
F 0 "#PWR08" H 2000 3650 30  0001 C CNN
F 1 "GND" H 2000 3580 30  0001 C CNN
F 2 "" H 2000 3650 60  0000 C CNN
F 3 "" H 2000 3650 60  0000 C CNN
	1    2000 3650
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR09
U 1 1 55D8EF4D
P 4000 3650
F 0 "#PWR09" H 4000 3650 30  0001 C CNN
F 1 "GND" H 4000 3580 30  0001 C CNN
F 2 "" H 4000 3650 60  0000 C CNN
F 3 "" H 4000 3650 60  0000 C CNN
	1    4000 3650
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR010
U 1 1 55D8FA33
P 900 1300
F 0 "#PWR010" H 900 1300 30  0001 C CNN
F 1 "GND" H 900 1230 30  0001 C CNN
F 2 "" H 900 1300 60  0000 C CNN
F 3 "" H 900 1300 60  0000 C CNN
	1    900  1300
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR011
U 1 1 55D8FA77
P 1500 1300
F 0 "#PWR011" H 1500 1300 30  0001 C CNN
F 1 "GND" H 1500 1230 30  0001 C CNN
F 2 "" H 1500 1300 60  0000 C CNN
F 3 "" H 1500 1300 60  0000 C CNN
	1    1500 1300
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR012
U 1 1 55D8FABB
P 1500 2250
F 0 "#PWR012" H 1500 2250 30  0001 C CNN
F 1 "GND" H 1500 2180 30  0001 C CNN
F 2 "" H 1500 2250 60  0000 C CNN
F 3 "" H 1500 2250 60  0000 C CNN
	1    1500 2250
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR013
U 1 1 55D8FAFF
P 4800 2450
F 0 "#PWR013" H 4800 2450 30  0001 C CNN
F 1 "GND" H 4800 2380 30  0001 C CNN
F 2 "" H 4800 2450 60  0000 C CNN
F 3 "" H 4800 2450 60  0000 C CNN
	1    4800 2450
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR014
U 1 1 55D8FB43
P 4800 1400
F 0 "#PWR014" H 4800 1400 30  0001 C CNN
F 1 "GND" H 4800 1330 30  0001 C CNN
F 2 "" H 4800 1400 60  0000 C CNN
F 3 "" H 4800 1400 60  0000 C CNN
	1    4800 1400
	1    0    0    -1  
$EndComp
$EndSCHEMATC
