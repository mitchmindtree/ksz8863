//! MII Management (MIIM) Interface.
//!
//! The IEEE 802.3 MII Management Interface, also known as the Management Data Input/Output
//! (MDIO) Interface. Allows for upper-layer devices to monitor and control the states of the
//! switch.
//!
//! Each of the 8 16-bit registers are indexed via a 5-bit address, preceded by a 5-bit PHY address.

use mdio::miim::{Read, Write};

/// Implemented for all 16-bit MIIM registers.
pub trait Register: Default + From<u16> + Into<u16> {
    /// The address at which the register can be located via the MIIM interface.
    const ADDRESS: Address;
}

/// A higher-level wrapper around an `miim::Read` and/or `miim::Write` implementation.
pub struct Miim<T>(pub T);

/// A wrapper around an `miim::Read` and/or `miim::Write` implementation for a particular PHY.
pub struct Phy<'miim, T> {
    pub miim: &'miim mut Miim<T>,
    pub addr: u8,
}

/// A wrapper around an `miim::Read` and/or `miim::Write` implementation for a particular register
/// on a particular PHY.
pub struct PhyReg<'phy, 'miim, T, R> {
    pub phy: &'phy mut Phy<'miim, T>,
    reg: core::marker::PhantomData<R>,
}

/// A type wrapper that allows to read the individual fields of a register.
pub struct R<T>(T);

/// A type wrapper that allows to write to the individual fields of a register.
pub struct W<T>(T);

/// The default PHY addresses of the two PHYs on the KSZ8863.
pub const DEFAULT_PHY_ADDRS: [u8; 2] = [0x01, 0x02];

impl_registers! {
    size_bits 16;
    data_type u16;
    miim_phy_register_methods Phy PhyReg;
    0x0 Bcr bcr [
        [R 15; 0] SoftReset soft_reset,
        [RW 14; 0] Loopback loopback,
        [RW 13; 0] Force100 force_100,
        [RW 12; 1] EnableAutoneg enable_autoneg,
        [RW 11; 0] PowerDown power_down,
        [R 10; 0] Isolate isolate,
        [RW 9; 0] RestartAutoneg restart_autoneg,
        [RW 8; 0] ForceFd force_fd,
        [R 7; 0] CollisionTest collision_test,
        [RW 5; 1] HpMdix hp_mdix,
        [RW 4; 0] ForceMdi force_mdi,
        [RW 3; 0] DisableMdix disable_mdix,
        [RW 2; 0] DisableFarEndFault disable_far_end_fault,
        [RW 1; 0] DisableTransmit disable_transmit,
        [RW 0; 0] DisableLeds disable_leds,
    ],
    0x1 Bsr bsr [
        [R 15; 0] CapableT4 capable_t4,
        [R 14; 1] Capable100Fd capable_100_fd,
        [R 13; 1] Capable100Hd capable_100_hd,
        [R 12; 1] Capable10Fd capable_10_fd,
        [R 11; 1] Capable10Hd capable_10_hd,
        [R 6; 0] PreambleSuppressed preamble_suppressed,
        [R 5; 0] AnComplete an_complete,
        [R 4; 0] RemoteFault remote_fault,
        [R 3; 1] AnCapable an_capable,
        [R 2; 0] LinkStatus link_status,
        [R 1; 0] JabberTest jabber_test,
        [R 0; 0] ExtendedCapable extended_capable,
    ],
    0x2 PhyIdR1 phyidr1 [
        [R 0..=15; u16; 0x0022] PhyIdHigh phy_id_high,
    ],
    0x3 PhyIdR2 phyidr2 [
        [RW 0..=15; u16; 0x1430] PhyIdLow phy_id_low,
    ],
    0x4 Anar anar [
        [R 15; 0] NextPage next_page,
        [R 13; 0] RemoteFault remote_fault,
        [RW 10; 1] AdvPause adv_pause,
        [RW 8; 1] Adv100Fd adv_100_fd,
        [RW 7; 1] Adv100Hd adv_100_hd,
        [RW 6; 1] Adv10Fd adv_10_fd,
        [RW 5; 1] Adv10Hd adv_10_hd,
    ],
    0x5 Anlpar anlpar [
        [R 15; 0] NextPage next_page,
        [R 10; 0] LpPause lp_pause,
        [R 8; 0] Lp100Fd lp_100_fd,
        [R 7; 0] Lp100Hd lp_100_hd,
        [R 6; 0] Lp10Fd lp_10_fd,
        [R 5; 0] Lp10Hd lp_10_hd,
    ],
    0x1D LinkMd link_md [
        [RW 15; 0] VctEnable vct_enable,
        [R 13..=14; 0] VctResult vct_result,
        [R 12; 0] Vct10mShort vct_10m_short,
        [R 0..=8; u16; 0] VctFaultCount vct_fault_count,
    ],
    0x1F PhySpecial phy_special [
        [R 5; 0] PolarityReversed polarity_reversed,
        [R 4; 0] MdixStatus mdix_status,
        [RW 3; 0] ForceLink force_link,
        [RW 2; 1] PowerSave power_save,
        [RW 1; 0] RemoteLoopback remote_loopback,
    ],
}

impl<T> Miim<T> {
    /// Address a particular PHY over MIIM.
    pub fn phy(&mut self, addr: u8) -> Phy<T> {
        Phy { miim: self, addr }
    }
}

impl<'miim, T> Phy<'miim, T> {
    /// Access a particular register associated with this PHY.
    pub fn reg<'phy, R>(&'phy mut self) -> PhyReg<'phy, 'miim, T, R> {
        PhyReg {
            phy: self,
            reg: core::marker::PhantomData,
        }
    }

    /// Read the register with the given address.
    pub fn read(&mut self, addr: Address) -> Result<State, T::Error>
    where
        T: Read,
    {
        let bits = self.miim.0.read(self.addr, addr.into())?;
        Ok(State::from_addr_and_data(addr, bits))
    }

    /// Write the given register state to the register with the associated address.
    pub fn write(&mut self, state: State) -> Result<(), T::Error>
    where
        T: Write,
    {
        self.miim
            .0
            .write(self.addr, state.addr().into(), state.into())
    }
}

impl<'phy, 'miim, T, R> PhyReg<'phy, 'miim, T, R>
where
    R: Register,
{
    /// Read from the register `R` associated with the specified PHY.
    pub fn read(&mut self) -> Result<R, T::Error>
    where
        T: Read,
    {
        let bits = self.phy.miim.0.read(self.phy.addr, R::ADDRESS.into())?;
        Ok(R::from(bits))
    }

    /// Write to the register `R` associated with the specified PHY.
    pub fn write<F>(&mut self, write: F) -> Result<(), T::Error>
    where
        T: Write,
        F: for<'a, 'b> FnOnce(&'a mut W<&'b mut R>) -> &'a mut W<&'b mut R>,
    {
        let mut reg = R::default();
        write(&mut W(&mut reg));
        self.phy
            .miim
            .0
            .write(self.phy.addr, R::ADDRESS.into(), reg.into())
    }

    /// Modify the register `R` associated with the specified PHY.
    ///
    /// This first reads the value from the register, delivers it to the user via the `modify`
    /// function, and then writes the result.
    pub fn modify<F, E>(&mut self, modify: F) -> Result<(), E>
    where
        T: Read<Error = E> + Write<Error = E>,
        F: for<'a, 'b> FnOnce(&'a mut W<&'b mut R>) -> &'a mut W<&'b mut R>,
    {
        let mut reg: R = self.read()?;
        modify(&mut W(&mut reg));
        self.phy
            .miim
            .0
            .write(self.phy.addr, R::ADDRESS.into(), reg.into())
    }
}

impl Read for Map {
    type Error = crate::InvalidAddress;
    fn read(&mut self, _phy_addr: u8, reg_addr: u8) -> Result<u16, Self::Error> {
        let addr: Address = core::convert::TryFrom::try_from(reg_addr)?;
        Ok((*self.state(addr)).into())
    }
}

impl Write for Map {
    type Error = crate::InvalidAddress;
    fn write(&mut self, _phy_addr: u8, reg_addr: u8, data: u16) -> Result<(), Self::Error> {
        let addr: Address = core::convert::TryFrom::try_from(reg_addr)?;
        self[addr] = State::from_addr_and_data(addr, data);
        Ok(())
    }
}
