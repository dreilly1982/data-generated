MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 2048K /* BANK_1 + BANK_2 */
    RAM   : ORIGIN = 0x20000000, LENGTH =  256K
    OTP   : ORIGIN = 0x08fff000, LENGTH = 2048
}