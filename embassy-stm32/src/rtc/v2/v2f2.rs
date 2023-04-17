use stm32_metapac::rcc::vals::Rtcsel;

pub const BACKUP_REGISTER_COUNT: usize = 20;

/// Unlock the backup domain
pub(super) unsafe fn unlock_backup_domain(clock_config: u8) {
    crate::pac::PWR.cr().modify(|w| w.set_dbp(true));
    while !crate::pac::PWR.cr().read().dbp() {}

    let reg = crate::pac::RCC.bdcr().read();

    if !reg.rtcen() || reg.rtcsel().0 != clock_config {
        crate::pac::RCC.bdcr().modify(|w| w.set_bdrst(true));

        crate::pac::RCC.bdcr().modify(|w| {
            // Reset
            w.set_bdrst(false);

            // Select RTC source
            w.set_rtcsel(Rtcsel(clock_config));
            w.set_rtcen(true);

            w.set_lseon(reg.lseon());
            w.set_lsebyp(reg.lsebyp());
        });
    }
}

pub(crate) unsafe fn enable_peripheral_clk() {
    // Nothing to do
}
