//! Serial Management Interface (SMI).
//!
//! The SMI is the KSZ8863-specific, non-standard MIIM interface that provides access to all
//! available configuration registers.
//!
//! Each register is indexed via an 8-bit address.

/// Implemented for all 8-bit SMI registers.
pub trait Register: Default + From<u8> + Into<u8> {
    /// The address at which the register can be located via the SMI interface.
    const ADDRESS: Address;
}

/// A trait for reading from the KSZ8863's SMI interface.
pub trait Read {
    /// Errors that might occur on the SMI interface.
    type Error;
    /// Read the data from the given register address associated with the specified PHY.
    fn read(&mut self, reg_addr: u8) -> Result<u8, Self::Error>;
}

/// A trait for writing to the KSZ8863's SMI interface.
pub trait Write {
    /// Errors that might occur on the SMI interface.
    type Error;
    /// Write to the register at the given address associated with the specified PHY.
    fn write(&mut self, reg_addr: u8, data: u8) -> Result<(), Self::Error>;
}

/// A higher-level wrapper around an `smi::Read` and/or `smi::Write` implementation.
pub struct Smi<T>(pub T);

/// A wrapper around an `miim::Read` and/or `miim::Write` implementation for a particular SMI
/// register.
pub struct Reg<'smi, T, R> {
    pub smi: &'smi mut Smi<T>,
    reg: core::marker::PhantomData<R>,
}

/// A type wrapper that allows to read the individual fields of a register.
pub struct R<T>(T);

/// A type wrapper that allows to write to the individual fields of a register.
pub struct W<T>(T);

impl_registers! {
    size_bits 8;
    data_type u8;
    smi_register_methods Smi Reg;

    // Chip ID and Start Switch

    0x00 ChipId0 chip_id0 [
        [R 0..=7; 0x88] FamilyId family_id,
    ],
    0x01 ChipId1 chip_id1 [
        [R 4..=7; 0x3] ChipId chip_id,
        [R 1..=3] RevisionId revision_id,
        [RW 0; 1] StartSwitch start_switch,
    ],

    // Global Control

    0x02 Gc0 gc0 [
        [RW 7; 0] NewBackOff new_back_off,
        [RW 5; 0] FlushDynamicMacTable flush_dynamic_mac_table,
        [RW 4; 0] FlushStaticMacTable flush_static_mac_table,
        [RW 3; 0] PassFlowControlPacket pass_flow_control_packet,
    ],
    0x03 Gc1 gc1 [
        [RW 7; 0] PassAllFrames pass_all_frames,
        [RW 6; 0] Port3TailTag port3_tail_tag,
        [RW 5; 1] TxFlowControl tx_flow_control,
        [RW 4; 1] RxFlowControl rx_flow_control,
        [RW 3; 0] FrameLengthFieldCheck frame_length_field_check,
        [RW 2; 1] Aging aging,
        [RW 1; 0] FastAge fast_age,
        [RW 0; 0] AggressiveBackOff aggressive_back_off,
    ],
    0x04 Gc2 gc2 [
        [RW 7; 1] UnicastPortVlanMismatchDiscard unicast_port_vlan_mismatch_discard,
        [RW 6; 1] MulticastStormProtectionDisable multicast_storm_protection_disable,
        [RW 5; 1] BackPressureMode back_pressure_mode,
        [RW 4; 1] FcBpFairMode fc_bp_fair_mode,
        [RW 3; 0] NoExcessiveCollisionDrop no_excessive_collision_drop,
        [RW 2; 0] HugePacketSupport huge_packet_support,
        [RW 1; 0] LegalMaxPacketSizeCheck legal_max_packet_size_check,
    ],
    0x05 Gc3 gc3 [
        [RW 7; 0] Vlan vlan,
        [RW 6; 0] IgmpSnoop igmp_snoop,
        [RW 3; 0] WeightedFairQueue weighted_fair_queue,
        [RW 0; 0] SniffMode sniff_mode,
    ],
    0x06 Gc4 gc4 [
        [RW 6; 0] MiiHdMode mii_hd_mode,
        [RW 5; 0] MiiFlowCtrl mii_flow_ctrl,
        [RW 4; 1] Mii10Bt mii_10_bt,
        [RW 3; 0] NullVidReplacement null_vid_replacement,
        [RW 0..=2; 0] BroadcastStormProtectionRateHigh broadcast_storm_protection_rate_high,
    ],
    0x07 Gc5 gc5 [
        [RW 0..=7; 0x63] BroadcastStormProtectionRateLow broadcast_storm_protection_rate_low,
    ],
    0x0B Gc9 gc9 [
        [RW 6..=7; 0b10] CpuIfaceClk cpu_iface_clk,
        [R 2..=3; 0b10] Reserved reserved,
    ],
    0x0C Gc10 gc10 [
        [RW 6..=7; 0b01] Tag0x3 tag_0x3,
        [RW 4..=5; 0b01] Tag0x2 tag_0x2,
        [RW 2..=3; 0b00] Tag0x1 tag_0x1,
        [RW 0..=1; 0b00] Tag0x0 tag_0x0,
    ],
    0x0D Gc11 gc11 [
        [RW 6..=7; 0b11] Tag0x7 tag_0x7,
        [RW 4..=5; 0b11] Tag0x6 tag_0x6,
        [RW 2..=3; 0b10] Tag0x5 tag_0x5,
        [RW 0..=1; 0b10] Tag0x4 tag_0x4,
    ],
    0x0E Gc12 gc12 [
        [RW 7; 0] UnknownPacketDefaultPortEnable unknown_packet_default_port_enable,
        [RW 6; 1] DriveStrength drive_strength,
        [RW 0..=2; 0b111] UnknownPacketDefaultPort unknown_packet_default_port,
    ],
    0x0F Gc13 gc13 [
        [RW 3..=7; 0b00001] PhyAddr phy_addr,
    ],

    // Port Control

    // Port 1
    0x10 Port1Ctrl0 port1_ctrl0 [
        [RW 7; 0] BroadcastStormProtection broadcast_storm_protection,
        [RW 6; 0] DiffServPriorityClassification diff_serv_priority_classification,
        [RW 5; 0] IeeePriorityClassification ieee_priority_classification,
        [RW 3..=4; 0] PortBasedPriorityClassification port_based_priority_classification,
        [RW 2; 0] TagInsertion tag_insertion,
        [RW 1; 0] TagRemoval tag_removal,
        [RW 0; 0] TxqSplitEnable txq_split,
    ],
    0x11 Port1Ctrl1 port1_ctrl1 [
        [RW 7; 0] SnifferPort sniffer_port,
        [RW 6; 0] ReceiveSniff receive_sniff,
        [RW 5; 0] TransmitSniff transmit_sniff,
        [RW 4; 0] DoubleTag double_tag,
        [RW 3; 0] UserPriorityCeiling user_priority_ceiling,
        [RW 0..=2; 0b111] PortVlanMembership port_vlan_membership,
    ],
    0x12 Port1Ctrl2 port1_ctrl2 [
        [RW 7; 0] Enable2QueueSplitTx enable_2_queue_split_tx,
        [RW 6; 0] IngressVlanFiltering ingress_vlan_filtering,
        [RW 5; 0] DiscardNonPvidPackets discard_non_pvid_packets,
        [RW 4] ForceFlowControl force_flow_control,
        [RW 3; 0] BackPressure back_pressure,
        [RW 2; 1] Transmit transmit,
        [RW 1; 1] Receive receive,
        [RW 0; 0] LearningDisable learning_disable,
    ],
    0x13 Port1Ctrl3 port1_ctrl3 [
        [RW 0..=7; 0x00] DefaultTag15_8 default_tag_15_8,
    ],
    0x14 Port1Ctrl4 port1_ctrl4 [
        [RW 0..=7; 0x01] DefaultTag7_0 default_tag_7_0,
    ],
    0x15 Port1Ctrl5 port1_ctrl5 [
        [RW 7; 0] Port3MiiModeSelection port3_mii_mode_selection,
        [RW 6; 0] SelfAddrFilteringEnableMaca1 self_addr_filtering_enable_maca1,
        [RW 5; 0] SelfAddrFilteringEnableMaca2 self_addr_filtering_enable_maca2,
        [RW 4; 0] DropIngressTaggedFrame dropped_ingress_tagged_frame,
        [RW 2..=3; 0b00] LimitMode limit_mode,
        [RW 1; 0] CoungIfg count_ifg,
        [RW 0; 0] CoungPre count_pre,
    ],
    0x16 Port1Q0IngressRateLimit port1_q0_ingress_rate_limit [
        [RW 0..=6; 0] Limit limit,
    ],
    0x17 Port1Q1IngressRateLimit port1_q1_ingress_rate_limit [
        [RW 0..=6; 0] Limit limit,
    ],
    0x18 Port1Q2IngressRateLimit port1_q2_ingress_rate_limit [
        [RW 0..=6; 0] Limit limit,
    ],
    0x19 Port1Q3IngressRateLimit port1_q3_ingress_rate_limit [
        [RW 0..=6; 0] Limit limit,
    ],
    0x1A Port1PhySpecial port1_phy_special [
        [R 5..=6; 0] VctResult vct_result,
        [RW 4; 0] VctEn vct_en,
        [RW 3; 0] ForceLink force_link,
        [RW 1; 0] RemoteLoopback remote_loopback,
        [R 0; 0] VctFaultCount8 vct_fault_count8,
    ],
    0x1B Port1LinkMdResult port1_link_md_result [
        [R 0..=7; 0] VctFaultCount7_0 vct_fault_count7_0,
    ],
    0x1C Port1Ctrl12 port1_ctrl12 [
        [RW 7] AnEnable an_enable,
        [RW 6] ForceSpeed force_speed,
        [RW 5] ForceDuplex force_duplex,
        [RW 4; 1] AdvFlowCtrl adv_flow_ctrl,
        [RW 3; 1] Adv100Fd adv_100_fd,
        [RW 2; 1] Adv100Hd adv_100_hd,
        [RW 1; 1] Adv10Fd adv_10_fd,
        [RW 0; 1] Adv10Hd adv_10_hd,
    ],
    0x1D Port1Ctrl13 port1_ctrl13 [
        [RW 7; 0] LedOff led_off,
        [RW 6; 0] DisableTx disable_tx,
        [RW 5; 0] RestartAn restart_an,
        [RW 4; 0] DisableFarEndFault disable_far_end_fault,
        [RW 3; 0] PowerDown power_down,
        [RW 2; 0] DisableAutoMdix disable_auto_mdix,
        [RW 1; 0] ForceMdi force_mdi,
        [RW 0; 0] Loopback loopback,
    ],
    0x1E Port1Status0 port1_status0 [
        [R 7; 0] MdixStatus mdix_status,
        [R 6; 0] AnDone an_done,
        [R 5; 0] LinkGood link_good,
        [R 4; 0] PartnerFlowCtrl partner_flow_ctrl,
        [R 3; 0] Partner100Fd partner_100_fd,
        [R 2; 0] Partner100Hd partner_100_hd,
        [R 1; 0] Partner10Fd partner_10_fd,
        [R 0; 0] Partner10Hd partner_10_hd,
    ],
    0x1F Port1Status1 port1_status1 [
        [R 7; 1] HpMdix hp_mdix,
        [R 5; 0] PolarityReversed polarity_reversed,
        [R 4; 0] TxFlowCtrl tx_flow_ctrl,
        [R 3; 0] RxFlowCtrl rx_flow_ctrl,
        [R 2; 0] OperationSpeed operation_speed,
        [R 1; 0] OperationDuplex operation_duplex,
        [R 0; 0] FarEndFault far_end_fault,
    ],

    // Port 2
    0x20 Port2Ctrl0 port2_ctrl0 [
        [RW 7; 0] BroadcastStormProtection broadcast_storm_protection,
        [RW 6; 0] DiffServPriorityClassification diff_serv_priority_classification,
        [RW 5; 0] IeeePriorityClassification ieee_priority_classification,
        [RW 3..=4; 0] PortBasedPriorityClassification port_based_priority_classification,
        [RW 2; 0] TagInsertion tag_insertion,
        [RW 1; 0] TagRemoval tag_removal,
        [RW 0; 0] TxqSplitEnable txq_split,
    ],
    0x21 Port2Ctrl1 port2_ctrl1 [
        [RW 7; 0] SnifferPort sniffer_port,
        [RW 6; 0] ReceiveSniff receive_sniff,
        [RW 5; 0] TransmitSniff transmit_sniff,
        [RW 4; 0] DoubleTag double_tag,
        [RW 3; 0] UserPriorityCeiling user_priority_ceiling,
        [RW 0..=2; 0b111] PortVlanMembership port_vlan_membership,
    ],
    0x22 Port2Ctrl2 port2_ctrl2 [
        [RW 7; 0] Enable2QueueSplitTx enable_2_queue_split_tx,
        [RW 6; 0] IngressVlanFiltering ingress_vlan_filtering,
        [RW 5; 0] DiscardNonPvidPackets discard_non_pvid_packets,
        [RW 4] ForceFlowControl force_flow_control,
        [RW 3; 0] BackPressure back_pressure,
        [RW 2; 1] Transmit transmit,
        [RW 1; 1] Receive receive,
        [RW 0; 0] LearningDisable learning_disable,
    ],
    0x23 Port2Ctrl3 port2_ctrl3 [
        [RW 0..=7; 0x00] DefaultTag15_8 default_tag_15_8,
    ],
    0x24 Port2Ctrl4 port2_ctrl4 [
        [RW 0..=7; 0x01] DefaultTag7_0 default_tag_7_0,
    ],
    0x25 Port2Ctrl5 port2_ctrl5 [
        [RW 7; 0] Port3MiiModeSelection port3_mii_mode_selection,
        [RW 6; 0] SelfAddrFilteringEnableMaca1 self_addr_filtering_enable_maca1,
        [RW 5; 0] SelfAddrFilteringEnableMaca2 self_addr_filtering_enable_maca2,
        [RW 4; 0] DropIngressTaggedFrame dropped_ingress_tagged_frame,
        [RW 2..=3; 0b00] LimitMode limit_mode,
        [RW 1; 0] CoungIfg count_ifg,
        [RW 0; 0] CoungPre count_pre,
    ],
    0x26 Port2Q0IngressRateLimit port2_q0_ingress_rate_limit [
        [RW 0..=6; 0] Limit limit,
    ],
    0x27 Port2Q1IngressRateLimit port2_q1_ingress_rate_limit [
        [RW 0..=6; 0] Limit limit,
    ],
    0x28 Port2Q2IngressRateLimit port2_q2_ingress_rate_limit [
        [RW 0..=6; 0] Limit limit,
    ],
    0x29 Port2Q3IngressRateLimit port2_q3_ingress_rate_limit [
        [RW 0..=6; 0] Limit limit,
    ],
    0x2A Port2PhySpecial port2_phy_special [
        [R 5..=6; 0] VctResult vct_result,
        [RW 4; 0] VctEn vct_en,
        [RW 3; 0] ForceLink force_link,
        [RW 1; 0] RemoteLoopback remote_loopback,
        [R 0; 0] VctFaultCount8 vct_fault_count8,
    ],
    0x2B Port2LinkMdResult port2_link_md_result [
        [R 0..=7; 0] VctFaultCount7_0 vct_fault_count7_0,
    ],
    0x2C Port2Ctrl12 port2_ctrl12 [
        [RW 7] AnEnable an_enable,
        [RW 6] ForceSpeed force_speed,
        [RW 5] ForceDuplex force_duplex,
        [RW 4; 1] AdvFlowCtrl adv_flow_ctrl,
        [RW 3; 1] Adv100Fd adv_100_fd,
        [RW 2; 1] Adv100Hd adv_100_hd,
        [RW 1; 1] Adv10Fd adv_10_fd,
        [RW 0; 1] Adv10Hd adv_10_hd,
    ],
    0x2D Port2Ctrl13 port2_ctrl13 [
        [RW 7; 0] LedOff led_off,
        [RW 6; 0] DisableTx disable_tx,
        [RW 5; 0] RestartAn restart_an,
        [RW 4; 0] DisableFarEndFault disable_far_end_fault,
        [RW 3; 0] PowerDown power_down,
        [RW 2; 0] DisableAutoMdix disable_auto_mdix,
        [RW 1; 0] ForceMdi force_mdi,
        [RW 0; 0] Loopback loopback,
    ],
    0x2E Port2Status0 port2_status0 [
        [R 7; 0] MdixStatus mdix_status,
        [R 6; 0] AnDone an_done,
        [R 5; 0] LinkGood link_good,
        [R 4; 0] PartnerFlowCtrl partner_flow_ctrl,
        [R 3; 0] Partner100Fd partner_100_fd,
        [R 2; 0] Partner100Hd partner_100_hd,
        [R 1; 0] Partner10Fd partner_10_fd,
        [R 0; 0] Partner10Hd partner_10_hd,
    ],
    0x2F Port2Status1 port2_status1 [
        [R 7; 1] HpMdix hp_mdix,
        [R 5; 0] PolarityReversed polarity_reversed,
        [R 4; 0] TxFlowCtrl tx_flow_ctrl,
        [R 3; 0] RxFlowCtrl rx_flow_ctrl,
        [R 2; 0] OperationSpeed operation_speed,
        [R 1; 0] OperationDuplex operation_duplex,
        [R 0; 0] FarEndFault far_end_fault,
    ],

    // Port 3
    0x30 Port3Ctrl0 port3_ctrl0 [
        [RW 7; 0] BroadcastStormProtection broadcast_storm_protection,
        [RW 6; 0] DiffServPriorityClassification diff_serv_priority_classification,
        [RW 5; 0] IeeePriorityClassification ieee_priority_classification,
        [RW 3..=4; 0] PortBasedPriorityClassification port_based_priority_classification,
        [RW 2; 0] TagInsertion tag_insertion,
        [RW 1; 0] TagRemoval tag_removal,
        [RW 0; 0] TxqSplitEnable txq_split,
    ],
    0x31 Port3Ctrl1 port3_ctrl1 [
        [RW 7; 0] SnifferPort sniffer_port,
        [RW 6; 0] ReceiveSniff receive_sniff,
        [RW 5; 0] TransmitSniff transmit_sniff,
        [RW 4; 0] DoubleTag double_tag,
        [RW 3; 0] UserPriorityCeiling user_priority_ceiling,
        [RW 0..=2; 0b111] PortVlanMembership port_vlan_membership,
    ],
    0x32 Port3Ctrl2 port3_ctrl2 [
        [RW 7; 0] Enable2QueueSplitTx enable_2_queue_split_tx,
        [RW 6; 0] IngressVlanFiltering ingress_vlan_filtering,
        [RW 5; 0] DiscardNonPvidPackets discard_non_pvid_packets,
        [RW 3; 0] BackPressure back_pressure,
        [RW 2; 1] Transmit transmit,
        [RW 1; 1] Receive receive,
        [RW 0; 0] LearningDisable learning_disable,
    ],
    0x33 Port3Ctrl3 port3_ctrl3 [
        [RW 0..=7; 0x00] DefaultTag15_8 default_tag_15_8,
    ],
    0x34 Port3Ctrl4 port3_ctrl4 [
        [RW 0..=7; 0x01] DefaultTag7_0 default_tag_7_0,
    ],
    0x35 Port3Ctrl5 port3_ctrl5 [
        [RW 7; 0] Port3MiiModeSelection port3_mii_mode_selection,
        [RW 6; 0] SelfAddrFilteringEnableMaca1 self_addr_filtering_enable_maca1,
        [RW 5; 0] SelfAddrFilteringEnableMaca2 self_addr_filtering_enable_maca2,
        [RW 4; 0] DropIngressTaggedFrame dropped_ingress_tagged_frame,
        [RW 2..=3; 0b00] LimitMode limit_mode,
        [RW 1; 0] CoungIfg count_ifg,
        [RW 0; 0] CoungPre count_pre,
    ],
    0x36 Port3Q0IngressRateLimit port3_q0_ingress_rate_limit [
        [RW 7; 0] RmiiRefclkInvert rmii_refclk_invert,
        [RW 0..=6; 0] Limit limit,
    ],
    0x37 Port3Q1IngressRateLimit port3_q1_ingress_rate_limit [
        [RW 0..=6; 0] Limit limit,
    ],
    0x38 Port3Q2IngressRateLimit port3_q2_ingress_rate_limit [
        [RW 0..=6; 0] Limit limit,
    ],
    0x39 Port3Q3IngressRateLimit port3_q3_ingress_rate_limit [
        [RW 0..=6; 0] Limit limit,
    ],
    0x3F Port3Status1 port3_status1 [
        [R 4; 0] TxFlowCtrl tx_flow_ctrl,
        [R 3; 0] RxFlowCtrl rx_flow_ctrl,
        [R 2; 0] OperationSpeed operation_speed,
        [R 1; 0] OperationDuplex operation_duplex,
    ],

    // Reset

    0x43 Reset reset [
        [RW 4; 0] Software software,
        [RW 0; 0] Pcs pcs,
    ],

    // Advanced Control Registers

    0x60 TosPriorityCtrl0 tos_priority_ctrl_0 [
        [RW 0..=7; 0] Dscp0_7 dscp0_7,
    ],
    0x61 TosPriorityCtrl1 tos_priority_ctrl_1 [
        [RW 0..=7; 0] Dscp8_15 dscp8_15,
    ],
    0x62 TosPriorityCtrl2 tos_priority_ctrl_2 [
        [RW 0..=7; 0] Dscp16_23 dscp16_23,
    ],
    0x63 TosPriorityCtrl3 tos_priority_ctrl_3 [
        [RW 0..=7; 0] Dscp24_31 dscp24_31,
    ],
    0x64 TosPriorityCtrl4 tos_priority_ctrl_4 [
        [RW 0..=7; 0] Dscp32_39 dscp32_39,
    ],
    0x65 TosPriorityCtrl5 tos_priority_ctrl_5 [
        [RW 0..=7; 0] Dscp40_47 dscp40_47,
    ],
    0x66 TosPriorityCtrl6 tos_priority_ctrl_6 [
        [RW 0..=7; 0] Dscp48_55 dscp48_55,
    ],
    0x67 TosPriorityCtrl7 tos_priority_ctrl_7 [
        [RW 0..=7; 0] Dscp56_63 dscp56_63,
    ],
    0x68 TosPriorityCtrl8 tos_priority_ctrl_8 [
        [RW 0..=7; 0] Dscp64_71 dscp64_71,
    ],
    0x69 TosPriorityCtrl9 tos_priority_ctrl_9 [
        [RW 0..=7; 0] Dscp72_79 dscp72_79,
    ],
    0x6A TosPriorityCtrl10 tos_priority_ctrl_10 [
        [RW 0..=7; 0] Dscp80_87 dscp80_87,
    ],
    0x6B TosPriorityCtrl11 tos_priority_ctrl_11 [
        [RW 0..=7; 0] Dscp88_95 dscp88_95,
    ],
    0x6C TosPriorityCtrl12 tos_priority_ctrl_12 [
        [RW 0..=7; 0] Dscp96_103 dscp96_103,
    ],
    0x6D TosPriorityCtrl13 tos_priority_ctrl_13 [
        [RW 0..=7; 0] Dscp104_111 dscp104_111,
    ],
    0x6E TosPriorityCtrl14 tos_priority_ctrl_14 [
        [RW 0..=7; 0] Dscp112_119 dscp112_119,
    ],
    0x6F TosPriorityCtrl15 tos_priority_ctrl_15 [
        [RW 0..=7; 0] Dscp120_127 dscp120_127,
    ],

    0x70 MacAddr0 mac_addr_0 [
        [RW 0..=7; 0x00] Data data,
    ],
    0x71 MacAddr1 mac_addr_1 [
        [RW 0..=7; 0x10] Data data,
    ],
    0x72 MacAddr2 mac_addr_2 [
        [RW 0..=7; 0xA1] Data data,
    ],
    0x73 MacAddr3 mac_addr_3 [
        [RW 0..=7; 0xFF] Data data,
    ],
    0x74 MacAddr4 mac_addr_4 [
        [RW 0..=7; 0xFF] Data data,
    ],
    0x75 MacAddr5 mac_addr_5 [
        [RW 0..=7; 0xFF] Data data,
    ],

    0x76 UserDef1 user_def1 [
        [RW 0..=7; 0] Data data,
    ],
    0x77 UserDef2 user_def2 [
        [RW 0..=7; 0] Data data,
    ],
    0x78 UserDef3 user_def3 [
        [RW 0..=7; 0] Data data,
    ],

    0x79 IndirectAccessCtrl0 indirect_access_ctrl0 [
        [RW 4; 0] ReadHighWriteLow read_high_write_low,
        [RW 2..=3; 0] TableSelect table_select,
        [RW 0..=1; 0] IndirectAddrHigh indirect_addr_high,
    ],
    0x7A IndirectAccessCtrl1 indirect_access_ctrl1 [
        [RW 0..=7; 0] IndirectAddrLow indirect_addr_low,
    ],

    0x7B IndirectData8 indirect_data8 [
        [R 7; 0] CpuReadStatus cpu_read_status,
        [RW 0..=2; 0] Data data,
    ],
    0x7C IndirectData7 indirect_data7 [
        [RW 0..=7; 0] Data data,
    ],
    0x7D IndirectData6 indirect_data6 [
        [RW 0..=7; 0] Data data,
    ],
    0x7E IndirectData5 indirect_data5 [
        [RW 0..=7; 0] Data data,
    ],
    0x7F IndirectData4 indirect_data4 [
        [RW 0..=7; 0] Data data,
    ],
    0x80 IndirectData3 indirect_data3 [
        [RW 0..=7; 0] Data data,
    ],
    0x81 IndirectData2 indirect_data2 [
        [RW 0..=7; 0] Data data,
    ],
    0x82 IndirectData1 indirect_data1 [
        [RW 0..=7; 0] Data data,
    ],
    0x83 IndirectData0 indirect_data0 [
        [RW 0..=7; 0] Data data,
    ],

    0x8E Station1MacAddr0 station1_mac_addr0 [
        [RW 0..=7] Data data,
    ],
    0x8F Station1MacAddr1 station1_mac_addr1 [
        [RW 0..=7] Data data,
    ],
    0x90 Station1MacAddr2 station1_mac_addr2 [
        [RW 0..=7] Data data,
    ],
    0x91 Station1MacAddr3 station1_mac_addr3 [
        [RW 0..=7] Data data,
    ],
    0x92 Station1MacAddr4 station1_mac_addr4 [
        [RW 0..=7] Data data,
    ],
    0x93 Station1MacAddr5 station1_mac_addr5 [
        [RW 0..=7] Data data,
    ],

    0x94 Station2MacAddr0 station2_mac_addr0 [
        [RW 0..=7] Data data,
    ],
    0x95 Station2MacAddr1 station2_mac_addr1 [
        [RW 0..=7] Data data,
    ],
    0x96 Station2MacAddr2 station2_mac_addr2 [
        [RW 0..=7] Data data,
    ],
    0x97 Station2MacAddr3 station2_mac_addr3 [
        [RW 0..=7] Data data,
    ],
    0x98 Station2MacAddr4 station2_mac_addr4 [
        [RW 0..=7] Data data,
    ],
    0x99 Station2MacAddr5 station2_mac_addr5 [
        [RW 0..=7] Data data,
    ],

    // TODO: [0x9A ..= 0xA5] Per-Port Egress Data Rate Limit

    0xA6 Mode mode [
        [R 0..=7] Data data,
    ],

    0xA7 HighPriorityPacketBufferQ3 high_priority_packet_buffer_q3 [
        [R 0..=7; 0x45] Data data,
    ],
    0xA8 HighPriorityPacketBufferQ2 high_priority_packet_buffer_q2 [
        [R 0..=7; 0x35] Data data,
    ],
    0xA9 HighPriorityPacketBufferQ1 high_priority_packet_buffer_q1 [
        [R 0..=7; 0x25] Data data,
    ],
    0xAA HighPriorityPacketBufferQ0 high_priority_packet_buffer_q0 [
        [R 0..=7; 0x15] Data data,
    ],

    0xAB PmUsageFlowCtrlSelectMode1 pm_usage_flow_ctrl_select_mode_1 [
        [R 0..=7] Data data,
    ],
    0xAC PmUsageFlowCtrlSelectMode2 pm_usage_flow_ctrl_select_mode_2 [
        [R 0..=7] Data data,
    ],
    0xAD PmUsageFlowCtrlSelectMode3 pm_usage_flow_ctrl_select_mode_3 [
        [R 0..=7] Data data,
    ],
    0xAE PmUsageFlowCtrlSelectMode4 pm_usage_flow_ctrl_select_mode_4 [
        [R 0..=7] Data data,
    ],

    0xAF Port1TxqSplitForQ3 port1_txq_split_for_q3 [
        [RW 7; 1] PrioritySelect priority_select,
    ],
    0xB0 Port1TxqSplitForQ2 port1_txq_split_for_q2 [
        [RW 7; 1] PrioritySelect priority_select,
    ],
    0xB1 Port1TxqSplitForQ1 port1_txq_split_for_q1 [
        [RW 7; 1] PrioritySelect priority_select,
    ],
    0xB2 Port1TxqSplitForQ0 port1_txq_split_for_q0 [
        [RW 7; 1] PrioritySelect priority_select,
    ],

    0xB3 Port2TxqSplitForQ3 port2_txq_split_for_q3 [
        [RW 7; 1] PrioritySelect priority_select,
    ],
    0xB4 Port2TxqSplitForQ2 port2_txq_split_for_q2 [
        [RW 7; 1] PrioritySelect priority_select,
    ],
    0xB5 Port2TxqSplitForQ1 port2_txq_split_for_q1 [
        [RW 7; 1] PrioritySelect priority_select,
    ],
    0xB6 Port2TxqSplitForQ0 port2_txq_split_for_q0 [
        [RW 7; 1] PrioritySelect priority_select,
    ],

    0xB7 Port3TxqSplitForQ3 port3_txq_split_for_q3 [
        [RW 7; 1] PrioritySelect priority_select,
    ],
    0xB8 Port3TxqSplitForQ2 port3_txq_split_for_q2 [
        [RW 7; 1] PrioritySelect priority_select,
    ],
    0xB9 Port3TxqSplitForQ1 port3_txq_split_for_q1 [
        [RW 7; 1] PrioritySelect priority_select,
    ],
    0xBA Port3TxqSplitForQ0 port3_txq_split_for_q0 [
        [RW 7; 1] PrioritySelect priority_select,
    ],

    0xBB InterruptEnable interrupt_enable [
        [RW 0..=7; 0] Reg reg,
    ],
    0xBC LinkChangeInterrupt link_change_interrupt [
        [RW 7; 0] P1P2 p1_p2,
        [RW 2; 0] P3 p3,
        [RW 1; 0] P2 p2,
        [RW 0; 0] P1 p1,
    ],
    0xBD ForcePauseOff force_pause_off [
        [RW 0..=7; 0] IterationLimitEnable iteration_limit_enable,
    ],
    0xC0 FiberSignalThreshold fiber_signal_threshold [
        [RW 7; 0] Port2 port2,
        [RW 6; 0] Port1 port1,
    ],
    0xC1 InternalLdoCtrl internal_ldo_ctrl [
        [RW 6; 0] Disable disable,
    ],
    0xC2 InsertSrcPvid insert_src_pvid [
        [RW 5; 0] P1AtP2 p1_at_p2,
        [RW 4; 0] P1AtP3 p1_at_p3,
        [RW 3; 0] P2AtP1 p2_at_p1,
        [RW 2; 0] P2AtP3 p2_at_p3,
        [RW 1; 0] P3AtP1 p3_at_p1,
        [RW 0; 0] P3AtP2 p3_at_p2,
    ],
    0xC3 PwrMgmtAndLedMode pwr_mgmt_and_led_mode [
        [RW 7; 0] CpuIfacePowerDown cpu_iface_power_down,
        [RW 6; 0] SwitchPowerDown switch_power_down,
        [RW 4..=5; 0] LedModeSelection led_mode_selection,
        [RW 3; 0] LedOutputMode led_output_mode,
        [RW 2; 0] PllOff pll_off,
        [RW 0..=1; 0] PwrMgmtMode pwr_mgmt_mode,
    ],
    0xC4 SleepMode sleep_mode [
        [RW 0..=7; 0x50] Data data,
    ],
    0xC6 FwdInvalidVidFrameAndHostMode fwd_invalid_vid_frame_and_host_mode [
        [RW 4..=6; 0] FwdInvalidVidFrame fwd_invalid_vid_frame,
        [RW 3; 0] P3RmiiClockSelection p3_rmii_clock_selection,
        [RW 2; 0] P1RmiiClockSelection p1_rmii_clock_selection,
        [RW 0..=1] HostIfaceMode host_iface_mode,
    ],
}

impl<T> Smi<T> {
    /// Access a particular register associated with this PHY.
    pub fn reg<R>(&mut self) -> Reg<T, R> {
        Reg {
            smi: self,
            reg: core::marker::PhantomData,
        }
    }

    /// Read the register with the given address.
    pub fn read(&mut self, addr: Address) -> Result<State, T::Error>
    where
        T: Read,
    {
        let bits = self.0.read(addr.into())?;
        Ok(State::from_addr_and_data(addr, bits))
    }

    /// Write the given register state to the register with the associated address.
    pub fn write(&mut self, state: State) -> Result<(), T::Error>
    where
        T: Write,
    {
        self.0.write(state.addr().into(), state.into())
    }
}

impl<'smi, T, R> Reg<'smi, T, R>
where
    R: Register,
{
    /// Read the value from register `R` via SMI.
    pub fn read(&mut self) -> Result<R, T::Error>
    where
        T: Read,
    {
        let bits = self.smi.0.read(R::ADDRESS.into())?;
        Ok(R::from(bits))
    }

    /// Write to the register `R`, initialised with a default state.
    pub fn write<F>(&mut self, write: F) -> Result<(), T::Error>
    where
        T: Write,
        F: for<'a, 'b> FnOnce(&'a mut W<&'b mut R>) -> &'a mut W<&'b mut R>,
    {
        let mut reg = R::default();
        write(&mut W(&mut reg));
        self.smi.0.write(R::ADDRESS.into(), reg.into())
    }

    /// Modify the register `R`.
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
        self.smi.0.write(R::ADDRESS.into(), reg.into())
    }
}

impl<'a, T> Read for &'a mut T
where
    T: Read,
{
    type Error = T::Error;
    fn read(&mut self, reg_addr: u8) -> Result<u8, Self::Error> {
        (*self).read(reg_addr)
    }
}

impl<'a, T> Write for &'a mut T
where
    T: Write,
{
    type Error = T::Error;
    fn write(&mut self, reg_addr: u8, data: u8) -> Result<(), Self::Error> {
        (*self).write(reg_addr, data)
    }
}

impl Read for Map {
    type Error = crate::InvalidAddress;
    fn read(&mut self, reg_addr: u8) -> Result<u8, Self::Error> {
        let addr: Address = core::convert::TryFrom::try_from(reg_addr)?;
        Ok((*self.state(addr)).into())
    }
}

impl Write for Map {
    type Error = crate::InvalidAddress;
    fn write(&mut self, reg_addr: u8, data: u8) -> Result<(), Self::Error> {
        let addr: Address = core::convert::TryFrom::try_from(reg_addr)?;
        self[addr] = State::from_addr_and_data(addr, data);
        Ok(())
    }
}
