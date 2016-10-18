#![allow(dead_code)]

/* RCC Memory Map */
pub const RCC: u32 = 0x40021000;
pub const RCC_APB2ENR: u32 = RCC + 0x18;
pub const RCC_APB1ENR: u32 = RCC + 0x1C;

/* GPIO Memory Map */
pub const GPIOA: u32 = 0x40010800;
pub const GPIOA_CRL: u32 = GPIOA + 0x00;
pub const GPIOA_CRH: u32 = GPIOA + 0x04;

/* USART2 Memory Map */
pub const USART2: u32 = 0x40004400;
pub const USART2_SR: u32 = USART2 + 0x00;
pub const USART2_DR: u32 = USART2 + 0x04;
pub const USART2_CR1: u32 = USART2 + 0x0C;

