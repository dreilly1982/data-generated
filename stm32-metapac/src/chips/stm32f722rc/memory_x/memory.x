MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH =  256K /* BANK_1_REGION_1 + BANK_1_REGION_2 + BANK_1_REGION_3 */
    RAM   : ORIGIN = 0x20010000, LENGTH =  192K
    OTP   : ORIGIN = 0x1ff07800, LENGTH =  512
}