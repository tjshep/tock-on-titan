#[no_mangle]
pub static INTERRUPT_TABLE: [Option<unsafe extern fn()>; 203] =
    [ None, // CRYPTO0_BREAK_INT,
      None, // CRYPTO0_DMEM_PTRS_OVERFLOW_INT
      None, // CRYPTO0_DONE_WIPE_SECRETS_INT
      None, // CRYPTO0_DRF_PTRS_OVERFLOW_INT
      None, // CRYPTO0_HOST_CMD_DONE_INT
      None, // CRYPTO0_HOST_CMD_RECV_INT
      None, // CRYPTO0_LOOP_STACK_OVERFLOW_INT
      None, // CRYPTO0_LOOP_STACK_UNDERFLOW_INT
      None, // CRYPTO0_MOD_OPERAND_OUT_OF_RANGE_INT
      None, // CRYPTO0_PC_STACK_OVERFLOW_INT
      None, // CRYPTO0_PGM_FAULT_INT
      None, // CRYPTO0_TRAP_INT
      None, // DMA0_INTR_COMPLETE_CHAN_INT
      None, // DMA0_INTR_ERROR_CHAN_INT
      None, // DMA0_INTR_PROG_CHAN_INT
      None, // DMA0_INTR_TIMEOUT_CHAN_INT
      None, // FLASH0_EDONEINT
      None, // FLASH0_PDONEINT
      None, // GLOBALSEC_CAMO0_BREACH_ALERT_INT
      None, // GLOBALSEC_CRYPTO0_DMEM_PARITY_ALERT_INT
      None, // GLOBALSEC_CRYPTO0_DRF_PARITY_ALERT_INT
      None, // GLOBALSEC_CRYPTO0_IMEM_PARITY_ALERT_INT
      None, // GLOBALSEC_CRYPTO0_PGM_FAULT_ALERT_INT
      None, // GLOBALSEC_DBCTRL_CPU0_D_IF_BUS_ERR_ALERT_INT
      None, // GLOBALSEC_DBCTRL_CPU0_D_IF_UPDATE_WATCHDOG_ALERT_INT
      None, // GLOBALSEC_DBCTRL_CPU0_I_IF_BUS_ERR_ALERT_INT
      None, // GLOBALSEC_DBCTRL_CPU0_I_IF_UPDATE_WATCHDOG_ALERT_INT
      None, // GLOBALSEC_DBCTRL_CPU0_S_IF_BUS_ERR_ALERT_INT
      None, // GLOBALSEC_DBCTRL_CPU0_S_IF_UPDATE_WATCHDOG_ALERT_INT
      None, // GLOBALSEC_DBCTRL_DDMA0_IF_BUS_ERR_ALERT_INT
      None, // GLOBALSEC_DBCTRL_DDMA0_IF_UPDATE_WATCHDOG_ALERT_INT
      None, // GLOBALSEC_DBCTRL_DSPS0_IF_BUS_ERR_ALERT_INT
      None, // GLOBALSEC_DBCTRL_DSPS0_IF_UPDATE_WATCHDOG_ALERT_INT
      None, // GLOBALSEC_DBCTRL_DUSB0_IF_BUS_ERR_ALERT_INT
      None, // GLOBALSEC_DBCTRL_DUSB0_IF_UPDATE_WATCHDOG_ALERT_INT
      None, // GLOBALSEC_FUSE0_FUSE_DEFAULTS_ALERT_INT
      None, // GLOBALSEC_GLOBALSEC_ALERT_GROUPA_INT
      None, // GLOBALSEC_GLOBALSEC_ALERT_GROUPB_INT
      None, // GLOBALSEC_GLOBALSEC_ALERT_GROUPC_INT
      None, // GLOBALSEC_GLOBALSEC_DIFF_FAIL_ALERT_INT
      None, // GLOBALSEC_GLOBALSEC_FW0_ALERT_INT
      None, // GLOBALSEC_GLOBALSEC_FW1_ALERT_INT
      None, // GLOBALSEC_GLOBALSEC_FW2_ALERT_INT
      None, // GLOBALSEC_GLOBALSEC_FW3_ALERT_INT
      None, // GLOBALSEC_GLOBALSEC_HEARTBEAT_FAIL_ALERT_INT
      None, // GLOBALSEC_GLOBALSEC_PROC_OPCODE_HASH_ALERT_INT
      None, // GLOBALSEC_GLOBALSEC_SRAM_PARITY_SCRUB_ALERT_INT
      None, // GLOBALSEC_KEYMGR0_AES_EXEC_CTR_MAX_ALERT_INT
      None, // GLOBALSEC_KEYMGR0_AES_HKEY_ALERT_INT
      None, // GLOBALSEC_KEYMGR0_CERT_LOOKUP_ALERT_INT
      None, // GLOBALSEC_KEYMGR0_FLASH_ENTRY_ALERT_INT
      None, // GLOBALSEC_KEYMGR0_PW_ALERT_INT
      None, // GLOBALSEC_KEYMGR0_SHA_EXEC_CTR_MAX_ALERT_INT
      None, // GLOBALSEC_KEYMGR0_SHA_FAULT_ALERT_INT
      None, // GLOBALSEC_KEYMGR0_SHA_HKEY_ALERT_INT
      None, // GLOBALSEC_PMU_BATTERY_MON_ALERT_INT
      None, // GLOBALSEC_PMU_PMU_WDOG_ALERT_INT
      None, // GLOBALSEC_RTC0_RTC_DEAD_ALERT_INT
      None, // GLOBALSEC_TEMP0_MAX_TEMP_ALERT_INT
      None, // GLOBALSEC_TEMP0_MAX_TEMP_DIFF_ALERT_INT
      None, // GLOBALSEC_TEMP0_MIN_TEMP_ALERT_INT
      None, // GLOBALSEC_TRNG0_OUT_OF_SPEC_ALERT_INT
      None, // GLOBALSEC_TRNG0_TIMEOUT_ALERT_INT
      None, // GLOBALSEC_VOLT0_VOLT_ERR_ALERT_INT
      None, // GLOBALSEC_XO0_JITTERY_TRIM_DIS_ALERT_INT
      None, // GPIO0_GPIO0INT
      None, // GPIO0_GPIO1INT
      None, // GPIO0_GPIO2INT
      None, // GPIO0_GPIO3INT
      None, // GPIO0_GPIO4INT
      None, // GPIO0_GPIO5INT
      None, // GPIO0_GPIO6INT
      None, // GPIO0_GPIO7INT
      None, // GPIO0_GPIO8INT
      None, // GPIO0_GPIO9INT
      None, // GPIO0_GPIO10INT
      None, // GPIO0_GPIO11INT
      None, // GPIO0_GPIO12INT
      None, // GPIO0_GPIO13INT
      None, // GPIO0_GPIO14INT
      None, // GPIO0_GPIO15INT
      None, // GPIO0_GPIOCOMBINT
      None, // GPIO1_GPIO0INT
      None, // GPIO1_GPIO1INT
      None, // GPIO1_GPIO2INT
      None, // GPIO1_GPIO3INT
      None, // GPIO1_GPIO4INT
      None, // GPIO1_GPIO5INT
      None, // GPIO1_GPIO6INT
      None, // GPIO1_GPIO7INT
      None, // GPIO1_GPIO8INT
      None, // GPIO1_GPIO9INT
      None, // GPIO1_GPIO10INT
      None, // GPIO1_GPIO11INT
      None, // GPIO1_GPIO12INT
      None, // GPIO1_GPIO13INT
      None, // GPIO1_GPIO14INT
      None, // GPIO1_GPIO15INT
      None, // GPIO1_GPIOCOMBINT
      None, // I2C0_I2CINT
      None, // I2C1_I2CINT
      None, // I2CS0_INTR_READ_BEGIN_INT
      None, // I2CS0_INTR_READ_COMPLETE_INT
      None, // I2CS0_INTR_WRITE_COMPLETE_INT
      None, // KEYMGR0_AES_DONE_CIPHER_INT
      None, // KEYMGR0_AES_DONE_KEYEXPANSION_INT
      None, // KEYMGR0_AES_DONE_WIPE_SECRETS_INT
      None, // KEYMGR0_AES_RFIFO_OVERFLOW_INT
      None, // KEYMGR0_AES_RFIFO_UNDERFLOW_INT
      None, // KEYMGR0_AES_WFIFO_OVERFLOW_INT
      None, // KEYMGR0_DSHA_INT
      None, // KEYMGR0_SHA_WFIFO_FULL_INT
      None, // PMU_INTR_WAKEUP_INT
      None, // RBOX0_INTR_AC_PRESENT_FED_INT
      None, // RBOX0_INTR_AC_PRESENT_RED_INT
      None, // RBOX0_INTR_BUTTON_COMBO0_RDY_INT
      None, // RBOX0_INTR_BUTTON_COMBO1_RDY_INT
      None, // RBOX0_INTR_BUTTON_COMBO2_RDY_INT
      None, // RBOX0_INTR_EC_RST_FED_INT
      None, // RBOX0_INTR_EC_RST_RED_INT
      None, // RBOX0_INTR_KEY0_IN_FED_INT
      None, // RBOX0_INTR_KEY0_IN_RED_INT
      None, // RBOX0_INTR_KEY1_IN_FED_INT
      None, // RBOX0_INTR_KEY1_IN_RED_INT
      None, // RBOX0_INTR_PWRB_IN_FED_INT
      None, // RBOX0_INTR_PWRB_IN_RED_INT
      None, // RDD0_INTR_DEBUG_STATE_DETECTED_INT
      None, // SPI0_SPITXINT
      None, // SPI1_SPITXINT
      None, // SPS0_CS_ASSERT_INTR
      None, // SPS0_CS_DEASSERT_INTR
      None, // SPS0_INTR_CMD_ADDR_FIFO_NOT_EMPTY_INT
      None, // SPS0_INTR_CMD_ADDR_FIFO_OVFL_INT
      None, // SPS0_INTR_CMD_MEM_OVFL_INT
      None, // SPS0_INTR_RAM_PAGE0_LVL_INT
      None, // SPS0_INTR_RAM_PAGE1_LVL_INT
      None, // SPS0_INTR_RAM_PAGE2_LVL_INT
      None, // SPS0_INTR_RAM_PAGE3_LVL_INT
      None, // SPS0_RXFIFO_LVL_INTR
      None, // SPS0_RXFIFO_OVERFLOW_INTR
      None, // SPS0_SPSCTRLINT0
      None, // SPS0_SPSCTRLINT1
      None, // SPS0_SPSCTRLINT2
      None, // SPS0_SPSCTRLINT3
      None, // SPS0_SPSCTRLINT4
      None, // SPS0_SPSCTRLINT5
      None, // SPS0_SPSCTRLINT6
      None, // SPS0_SPSCTRLINT7
      None, // SPS0_TXFIFO_EMPTY_INTR
      None, // SPS0_TXFIFO_FULL_INTR
      None, // SPS0_TXFIFO_LVL_INTR
      None, // TEMP0_ADC_ICLKDV_INT
      None, // TEMP0_COMP_OVERFLOW_INT
      None, // TIMEHS0_TIMINT1
      None, // TIMEHS0_TIMINT2
      None, // TIMEHS0_TIMINTC
      None, // TIMEHS1_TIMINT1
      None, // TIMEHS1_TIMINT2
      None, // TIMEHS1_TIMINTC
      None, // TIMELS0_TIMINT0
      None, // TIMELS0_TIMINT1
      None, // TIMEUS0_INTR_MAX_COUNT_HIT0_INT
      None, // TIMEUS0_INTR_MAX_COUNT_HIT1_INT
      None, // TIMEUS0_INTR_MAX_COUNT_HIT2_INT
      None, // TIMEUS0_INTR_MAX_COUNT_HIT3_INT
      None, // TIMEUS0_INTR_PROG_COUNT_HIT0_INT
      None, // TIMEUS0_INTR_PROG_COUNT_HIT1_INT
      None, // TIMEUS0_INTR_PROG_COUNT_HIT2_INT
      None, // TIMEUS0_INTR_PROG_COUNT_HIT3_INT
      None, // TRNG0_INTR_BUFFER_FULL_INT
      None, // TRNG0_INTR_ONE_SHOT_DONE_INT
      None, // TRNG0_INTR_READ_EMPTY_INT
      None, // UART0_RXBINT
      None, // UART0_RXFINT
      None, // UART0_RXINT
      None, // UART0_RXOVINT
      None, // UART0_RXTOINT
      Some(::uart::uart0_handler), // UART0_TXINT
      None, // UART0_TXOVINT
      None, // UART1_RXBINT
      None, // UART1_RXFINT
      None, // UART1_RXINT
      None, // UART1_RXOVINT
      None, // UART1_RXTOINT
      None, // UART1_TXINT
      None, // UART1_TXOVINT
      None, // UART2_RXBINT
      None, // UART2_RXFINT
      None, // UART2_RXINT
      None, // UART2_RXOVINT
      None, // UART2_RXTOINT
      None, // UART2_TXINT
      None, // UART2_TXOVINT
      Some(::usb::usb_handler), // USB0_USBINTR
      None, // WATCHDOG0_WDOGINT
      None, // XO0_CLK_JTR_NOP_SEEN_INT
      None, // XO0_CLK_JTR_SW_TRIM_DONE_INT
      None, // XO0_CLK_TIMER_NOP_SEEN_INT
      None, // XO0_CLK_TIMER_SW_TRIM_DONE_INT
      None, // XO0_FAST_CALIB_OVERFLOW_INT
      None, // XO0_FAST_CALIB_UNDERRUN_INT
      None, // XO0_SLOW_CALIB_OVERFLOW_INT
      None, // XO0_SLOW_CALIB_UNDERRUN_INT
    ];
