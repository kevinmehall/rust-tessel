// Copyright 2014 Technical Machine, Inc. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


.equ    Sign_Value, 0x5A5A5A5A


/* Vector Table */

    .section ".vectors"
    .globl  _vector_table
    .type   _vector_table, %object

_vector_table:
    .long   _estack                     /* Top of Stack                 */
    .long   _reset_handler               /* Reset Handler                */
    .long   NMI_Handler                 /* NMI Handler                  */
    .long   HardFault_Handler           /* Hard Fault Handler           */
    .long   MemManage_Handler           /* MPU Fault Handler            */
    .long   BusFault_Handler            /* Bus Fault Handler            */
    .long   UsageFault_Handler          /* Usage Fault Handler          */
    .long   Sign_Value                  /* Reserved                     */
    .long   0                           /* Reserved                     */
    .long   0                           /* Reserved                     */
    .long   0                           /* Reserved                     */
    .long   SVC_Handler                 /* SVCall Handler               */
    .long   DebugMon_Handler            /* Debug Monitor Handler        */
    .long   0                           /* Reserved                     */
    .long   PendSV_Handler              /* PendSV Handler               */
    .long   SysTick_Handler             /* SysTick Handler              */

    /* External Interrupts */
    .long	DAC_IRQHandler	 			/* 16 D/A Converter */
	.long	0				/* 17 Event Router */
	.long	DMA_IRQHandler				/* 18 General Purpose DMA */
	.long	0					/* 19 Reserved */
	.long	0					/* 20 Reserved */
	.long	ETH_IRQHandler				/* 21 Ethernet */
	.long	SDIO_IRQHandler				/* 22 SD/MMC */
	.long	LCD_IRQHandler				/* 23 LCD */
	.long	USB0_IRQHandler				/* 24 USB0*/
	.long	USB1_IRQHandler				/* 25 USB1*/
	.long	SCT_IRQHandler				/* 26 State Configurable Timer*/
	.long	RIT_IRQHandler				/* 27 Repetitive Interrupt Timer*/
	.long	TIMER0_IRQHandler			/* 28 Timer0*/
	.long	TIMER1_IRQHandler			/* 29 Timer1*/
	.long	TIMER2_IRQHandler			/* 30 Timer2*/
	.long	TIMER3_IRQHandler			/* 31 Timer3*/
	.long	MCPWM_IRQHandler			/* 32 Motor Control PWM*/
	.long	ADC0_IRQHandler				/* 33 A/D Converter 0*/
	.long	I2C0_IRQHandler				/* 34 I2C0*/
	.long	I2C1_IRQHandler				/* 35 I2C1*/
	.long	0					/* 36 Reserved*/
	.long	ADC1_IRQHandler				/* 37 A/D Converter 1*/
	.long	SSP0_IRQHandler				/* 38 SSP0*/
	.long	SSP1_IRQHandler				/* 39 SSP1*/
	.long	UART0_IRQHandler			/* 40 UART0*/
	.long	UART1_IRQHandler			/* 41 UART1*/
	.long	UART2_IRQHandler			/* 42 UART2*/
	.long	UART3_IRQHandler			/* 43 UART3*/
	.long	I2S0_IRQHandler				/* 44 I2S*/
	.long 	I2S1_IRQHandler				/* 45 AES Engine*/
	.long 	SPIFI_IRQHandler			/* 46 SPI Flash Interface*/
	.long	SGPIO_IRQHandler			/* 47 SGPIO*/
	.long	GPIO0_IRQHandler			/* 48 GPIO0*/
	.long	GPIO1_IRQHandler			/* 49 GPIO1*/
	.long	GPIO2_IRQHandler			/* 50 GPIO2*/
	.long	GPIO3_IRQHandler			/* 51 GPIO3*/
	.long	GPIO4_IRQHandler			/* 52 GPIO4*/
	.long	GPIO5_IRQHandler			/* 53 GPIO5*/
	.long	GPIO6_IRQHandler			/* 54 GPIO6*/
	.long	GPIO7_IRQHandler			/* 55 GPIO7*/
	.long	GINT0_IRQHandler			/* 56 GINT0*/
	.long	GINT1_IRQHandler			/* 57 GINT1*/
	.long	EVRT_IRQHandler				/* 58 Event Router*/
	.long	CAN1_IRQHandler				/* 59 C_CAN1*/
	.long	0							/* 60 Reserved*/
	.long	VADC_IRQHandler				/* 61 VADC*/
	.long	ATIMER_IRQHandler			/* 62 ATIMER*/
	.long	RTC_IRQHandler				/* 63 RTC*/
	.long	0							/* 64 Reserved*/
	.long	WDT_IRQHandler				/* 65 WDT*/
	.long	0							/* 66 M0s*/
	.long	CAN0_IRQHandler				/* 67 C_CAN0*/
	.long 	QEI_IRQHandler				/* 68 QEI*/
    .long   0
    .long   0
    .long   0
    .byte   0
    .byte   0
    .byte   0

    .size   _vector_table, . - _vector_table


    .thumb

.section ".text"

/* Exception Handlers */

    .weak   NMI_Handler
    .type   NMI_Handler, %function
NMI_Handler:
    B       .
    .size   NMI_Handler, . - NMI_Handler

    .weak   HardFault_Handler
    .type   HardFault_Handler, %function
HardFault_Handler:
    B       .
    .size   HardFault_Handler, . - HardFault_Handler

    .weak   MemManage_Handler
    .type   MemManage_Handler, %function
MemManage_Handler:
    B       .
    .size   MemManage_Handler, . - MemManage_Handler

    .weak   BusFault_Handler
    .type   BusFault_Handler, %function
BusFault_Handler:
    B       .
    .size   BusFault_Handler, . - BusFault_Handler

    .weak   UsageFault_Handler
    .type   UsageFault_Handler, %function
UsageFault_Handler:
    B       .
    .size   UsageFault_Handler, . - UsageFault_Handler

    .weak   SVC_Handler
    .type   SVC_Handler, %function
SVC_Handler:
    B       .
    .size   SVC_Handler, . - SVC_Handler

    .weak   DebugMon_Handler
    .type   DebugMon_Handler, %function
DebugMon_Handler:
    B       .
    .size   DebugMon_Handler, . - DebugMon_Handler

    .weak   PendSV_Handler
    .type   PendSV_Handler, %function
PendSV_Handler:
    B       .
    .size   PendSV_Handler, . - PendSV_Handler

    .weak   SysTick_Handler
    .type   SysTick_Handler, %function
SysTick_Handler:
    B       .
    .size   SysTick_Handler, . - SysTick_Handler


/* IRQ Handlers */

    .globl  Default_Handler
    .type   Default_Handler, %function
Default_Handler:
    B       .
    .size   Default_Handler, . - Default_Handler

    .macro  IRQ handler
    .weak   \handler
    .set    \handler, Default_Handler
    .endm

    IRQ DAC_IRQHandler
	IRQ DMA_IRQHandler
	IRQ ETH_IRQHandler
	IRQ SDIO_IRQHandler
	IRQ LCD_IRQHandler
	IRQ USB0_IRQHandler
	IRQ USB1_IRQHandler
	IRQ SCT_IRQHandler
	IRQ RIT_IRQHandler
	IRQ TIMER0_IRQHandler
	IRQ TIMER1_IRQHandler
	IRQ TIMER2_IRQHandler
	IRQ TIMER3_IRQHandler
	IRQ MCPWM_IRQHandler
	IRQ ADC0_IRQHandler
	IRQ I2C0_IRQHandler
	IRQ I2C1_IRQHandler
	IRQ ADC1_IRQHandler
	IRQ SSP0_IRQHandler
	IRQ SSP1_IRQHandler
	IRQ UART0_IRQHandler
	IRQ UART1_IRQHandler
	IRQ UART2_IRQHandler
	IRQ UART3_IRQHandler
	IRQ I2S0_IRQHandler
	IRQ I2S1_IRQHandler
	IRQ SPIFI_IRQHandler
	IRQ	SGPIO_IRQHandler
	IRQ	GPIO0_IRQHandler
	IRQ	GPIO1_IRQHandler
	IRQ	GPIO2_IRQHandler
	IRQ	GPIO3_IRQHandler
	IRQ	GPIO4_IRQHandler
	IRQ	GPIO5_IRQHandler
	IRQ	GPIO6_IRQHandler
	IRQ	GPIO7_IRQHandler
	IRQ	GINT0_IRQHandler
	IRQ	GINT1_IRQHandler
	IRQ	EVRT_IRQHandler
	IRQ	CAN1_IRQHandler
	IRQ	VADC_IRQHandler
	IRQ	ATIMER_IRQHandler
	IRQ	RTC_IRQHandler
	IRQ	WDT_IRQHandler
	IRQ	CAN0_IRQHandler
	IRQ	QEI_IRQHandler

	.globl  getPC
    .type   getPC, %function
    .thumb_func
getPC:
	MOV     R0,LR
	BX		LR
	.size   getPC,.-getPC

	.globl  __morestack
	.type   __morestack, %function
__morestack:
	B       .;
    .end
