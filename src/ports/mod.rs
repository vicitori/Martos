use core::time::Duration;
#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
#[cfg(feature = "network")]
use esp_wifi::esp_now::EspNow;

/// PortTrait contains all the platform specific functions.
pub trait PortTrait {
    /// Function is called when timer is created. Can be used to set configuration.
    fn setup_hardware_timer();
    /// Function is used to check the correctness of index.
    fn valid_timer_index(timer_index: u8) -> bool;
    /// Function is used to check if the timer is active.
    fn timer_is_active(timer_index: u8) -> bool;
    /// Function is called to start timer.
    fn start_hardware_timer(timer_index: u8);
    /// Function is used to change timer operating mode.
    fn set_reload_mode(timer_index: u8, auto_reload: bool);
    /// Function is used to change the period of a timer.
    fn change_period_timer(timer_index: u8, period: Duration);
    /// Function is used to get amount of time from the start of a timer.
    fn get_time(timer_index: u8) -> Duration;
    /// Function is called to stop timer.
    fn stop_hardware_timer(timer_index: u8) -> bool;

    /// Function is called when heap is created. Can be used to set configuration.
    fn init_heap();
    #[cfg(feature = "network")]
    /// Function for initializing network settings.
    fn init_network();
    #[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
    #[cfg(feature = "network")]
    /// Function for getting esp-now object for network.
    fn get_esp_now() -> EspNow<'static>;
}

/// Port is an alias of PortTrait implementation for a current platform

#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
pub mod xtensa_esp32;
#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
pub type Port = xtensa_esp32::XtensaEsp32;

#[cfg(all(
    not(any(target_arch = "riscv32", target_arch = "xtensa")),
    not(target_arch = "mips64")
))]
pub mod mok;
#[cfg(all(
    not(any(target_arch = "riscv32", target_arch = "xtensa")),
    not(target_arch = "mips64")
))]
pub type Port = mok::Mok;

#[cfg(any(target_arch = "mips64", feature = "mips64_timer_tests"))]
pub mod mips64;
#[cfg(target_arch = "mips64")]
pub type Port = mips64::Mips64;
