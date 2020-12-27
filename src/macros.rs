//! Macro implementations.

/// A macro for generating the register implementations in the `miim` and `smi` modules.
macro_rules! impl_registers {
    // Declare a unique type for each field that will be used for providing accessor methods.
    (declare_fields $([$($tokens:tt)*] $Field:ident $field:ident,)*) => {
        $(
            pub struct $Field<T>(T);
        )*
    };

    // Define the methods providing read access to each field.
    (define_read_field $Field:ident $field:ident) => {
        pub fn $field(&self) -> $Field<&Self> {
            $Field(self)
        }
    };
    (define_read_field [R $($tokens:tt)*] $Field:ident $field:ident) => {
        impl_registers!(define_read_field $Field $field);
    };
    (define_read_field [RW $($tokens:tt)*] $Field:ident $field:ident) => {
        impl_registers!(define_read_field $Field $field);
    };
    (define_read_field [W $($tokens:tt)*] $Field:ident $field:ident) => {
    };
    (define_read_fields $([$($tokens:tt)*] $Field:ident $field:ident,)*) => {
        $(
            impl_registers!(define_read_field [$($tokens)*] $Field $field);
        )*
    };

    // Define the methods providing write access to each field.
    (define_write_field $Field:ident $field:ident) => {
        pub fn $field(&mut self) -> $Field<&mut Self> {
            $Field(self)
        }
    };
    (define_write_field [R $($tokens:tt)*] $Field:ident $field:ident) => {
    };
    (define_write_field [RW $($tokens:tt)*] $Field:ident $field:ident) => {
        impl_registers!(define_write_field $Field $field);
    };
    (define_write_field [W $($tokens:tt)*] $Field:ident $field:ident) => {
        impl_registers!(define_write_field $Field $field);
    };
    (define_write_fields $([$($tokens:tt)*] $Field:ident $field:ident,)*) => {
        $(
            impl_registers!(define_write_field [$($tokens)*] $Field $field);
        )*
    };

    // Define the methods for accessing the value associated with a single-bit field.
    (define_field_r_methods [$bit_index:literal]) => {
        /// Value of the field as a raw bit.
        pub fn bit(&self) -> bool {
            self.0.0.fields[$bit_index]
        }

        /// Returns `true` if the bit is set (1).
        pub fn bit_is_set(&self) -> bool {
            self.bit()
        }

        /// Returns `true` if the bit is clear (0).
        pub fn bit_is_clear(&self) -> bool {
            !self.bit()
        }
    };
    (define_field_r_methods [$bit_index:literal; $default:literal]) => {
        impl_registers!(define_field_r_methods [$bit_index]);
    };

    // Define the methods for accessing the value associated with a field spanning multiple bits.
    (define_field_r_methods [$bit_range:expr]) => {
        /// Value of the field as raw bits.
        pub fn bits(&self) -> u8 {
            self.0.0.fields[$bit_range].load()
        }
    };
    (define_field_r_methods [$bit_range:expr; $default:literal]) => {
        impl_registers!(define_field_r_methods [$bit_range]);
    };

    // Define the methods for accessing the value associated with a field spanning multiple bits.
    (define_field_r_methods [$bit_range:expr; $Ty:ty]) => {
        /// Value of the field as raw bits.
        pub fn bits(&self) -> $Ty {
            self.0.0.fields[$bit_range].load()
        }
    };
    (define_field_r_methods [$bit_range:expr; $Ty:ty; $default:literal]) => {
        impl_registers!(define_field_r_methods [$bit_range; $Ty]);
    };

    // First check that the register is readable.
    (define_field_r_methods [R $($tokens:tt)*]) => {
        impl_registers!(define_field_r_methods [$($tokens)*]);
    };
    (define_field_r_methods [RW $($tokens:tt)*]) => {
        impl_registers!(define_field_r_methods [$($tokens)*]);
    };
    (define_field_r_methods [W $($tokens:tt)*]) => {
    };

    // Create the impl that will provide methods for accessing the values for each field.
    (define_field_r_impl $Reg:ident [$($tokens:tt)*] $Field:ident $field:ident) => {
        impl<'a, 'b> $Field<&'a R<&'b $Reg>> {
            impl_registers!(define_field_r_methods [$($tokens)*]);
        }
    };
    (define_field_r_impl $Reg:ident [R $($tokens:tt)*] $Field:ident $field:ident) => {
        impl_registers!(define_field_r_impl $Reg [$($tokens)*] $Field $field);
    };
    (define_field_r_impl $Reg:ident [RW $($tokens:tt)*] $Field:ident $field:ident) => {
        impl_registers!(define_field_r_impl $Reg [$($tokens)*] $Field $field);
    };
    (define_field_r_impl $Reg:ident [W $($tokens:tt)*] $Field:ident $field:ident) => {
    };
    (define_field_r_impls $Reg:ident $([$($tokens:tt)*] $Field:ident $field:ident,)*) => {
        $(
            impl_registers!(define_field_r_impl $Reg [$($tokens)*] $Field $field);
        )*
    };

    // Define the methods for accessing the value associated with a single-bit field.
    (define_field_w_methods $Reg:ident [$bit_index:literal]) => {
        /// Set the value as a raw bit, where true is 1, false is 0.
        pub fn bit(self, bit: bool) -> &'a mut W<&'b mut $Reg> {
            unsafe {
                *self.0.0.fields.get_unchecked_mut($bit_index) = bit;
            }
            self.0
        }

        /// Set the field bit (to 1).
        pub fn set_bit(self) -> &'a mut W<&'b mut $Reg> {
            self.bit(true)
        }

        /// Clear the field bit (to 0).
        pub fn clear_bit(self) -> &'a mut W<&'b mut $Reg> {
            self.bit(false)
        }
    };
    (define_field_w_methods $Reg:ident [$bit_index:literal; $default:literal]) => {
        impl_registers!(define_field_w_methods $Reg [$bit_index]);
        /// Reset the field to its default value.
        pub fn reset(self) -> &'a mut W<&'b mut $Reg> {
            self.bit(crate::IntoBool::into_bool($default))
        }
    };

    // Define the methods for writing the value associated with a field spanning multiple bits.
    (define_field_w_methods $Reg:ident [$bit_range:expr]) => {
        /// Value of the field as raw bits.
        pub fn bits(self, bits: u8) -> &'a mut W<&'b mut $Reg> {
            self.0.0.fields[$bit_range].store(bits);
            self.0
        }
    };
    (define_field_w_methods $Reg:ident [$bit_range:expr; $default:literal]) => {
        impl_registers!(define_field_w_methods $Reg [$bit_range]);
        /// Reset the field to its default value.
        pub fn reset(self) -> &'a mut W<&'b mut $Reg> {
            self.bits($default)
        }
    };

    // Define the methods for writing the value associated with a field spanning multiple bits.
    (define_field_w_methods $Reg:ident [$bit_range:expr; $Ty:ty]) => {
        /// Value of the field as raw bits.
        pub fn bits(self, bits: $Ty) -> &'a mut W<&'b mut $Reg> {
            self.0.0.fields[$bit_range].store(bits);
            self.0
        }
    };
    (define_field_w_methods $Reg:ident [$bit_range:expr; $Ty:ty; $default:literal]) => {
        impl_registers!(define_field_w_methods $Reg [$bit_range; $Ty]);
        /// Value of the field as raw bits.
        pub fn reset(self) -> &'a mut W<&'b mut $Reg> {
            self.bits($default)
        }
    };

    // Create the impl that will provide methods for accessing the values for each field.
    (define_field_w_impl $Reg:ident [W $($tokens:tt)*] $Field:ident $field:ident) => {
        impl<'a, 'b> $Field<&'a mut W<&'b mut $Reg>> {
            impl_registers!(define_field_w_methods $Reg [$($tokens)*]);
        }
    };
    (define_field_w_impl $Reg:ident [R $($tokens:tt)*] $Field:ident $field:ident) => {};
    (define_field_w_impl $Reg:ident [RW $($tokens:tt)*] $Field:ident $field:ident) => {
        impl_registers!(define_field_w_impl $Reg [W $($tokens)*] $Field $field);
    };
    (define_field_w_impls $Reg:ident $([$($tokens:tt)*] $Field:ident $field:ident,)*) => {
        $(
            impl_registers!(define_field_w_impl $Reg [$($tokens)*] $Field $field);
        )*
    };

    // Produce the reset statements that reset a register to its default state.
    (field_reset_stmt $reg:ident $field:ident) => {
        $reg.$field().reset();
    };
    (field_reset_stmt $reg:ident [W $bit_index:literal; $default:literal] $field:ident) => {
        impl_registers!(field_reset_stmt $reg $field);
    };
    (field_reset_stmt $reg:ident [W $bit_range:expr; $default:literal] $field:ident) => {
        impl_registers!(field_reset_stmt $reg $field);
    };
    (field_reset_stmt $reg:ident [W $bit_range:expr; $Ty:ty; $default:literal] $field:ident) => {
        impl_registers!(field_reset_stmt $reg $field);
    };
    (field_reset_stmt $reg:ident [RW $($tokens:tt)*] $field:ident) => {
        impl_registers!(field_reset_stmt $reg [W $($tokens)*] $field);
    };
    (field_reset_stmt $reg:ident [$($tokens:tt)*] $field:ident) => {
    };
    (field_reset_stmts $reg:ident $([$($tokens:tt)*] $Field:ident $field:ident,)*) => {
        $(
            impl_registers!(field_reset_stmt $reg [$($tokens)*] $field);
        )*
    };

    // Produce a statement for resetting each field of the register to its default state.
    (field_default_stmt $reg:ident [R $($tokens:tt)*] $field:ident) => {
        impl_registers!(field_default_stmt $reg [$($tokens)*] $field);
    };
    (field_default_stmt $reg:ident [RW $($tokens:tt)*] $field:ident) => {
        impl_registers!(field_default_stmt $reg [$($tokens)*] $field);
    };
    (field_default_stmt $reg:ident [W $($tokens:tt)*] $field:ident) => {
        impl_registers!(field_default_stmt $reg [$($tokens)*] $field);
    };
    (field_default_stmt $reg:ident [$bit_index:literal; $default:literal] $field:ident) => {
        unsafe {
            let b: bool = crate::IntoBool::into_bool($default);
            *$reg.fields.get_unchecked_mut($bit_index) = b;
        }
    };
    (field_default_stmt $reg:ident [$bit_range:expr; $default:literal] $field:ident) => {
        $reg.fields[$bit_range].store::<u8>($default);
    };
    (field_default_stmt $reg:ident [$bit_range:expr; $Ty:ty; $default:literal] $field:ident) => {
        $reg.fields[$bit_range].store::<$Ty>($default);
    };
    (field_default_stmt $reg:ident [$($tokens:tt)*] $field:ident) => {};
    (field_default_stmts $reg:ident $([$($tokens:tt)*] $Field:ident $field:ident,)*) => {
        $(
            impl_registers!(field_default_stmt $reg [$($tokens)*] $field);
        )*
    };

    // The statements used for the register `Debug` and `uDebug` implementations.
    (field_debug_expr $reg:ident [R $($tokens:tt)*] $field:ident) => {
        impl_registers!(field_debug_expr $reg [$($tokens)*] $field)
    };
    (field_debug_expr $reg:ident [RW $($tokens:tt)*] $field:ident) => {
        impl_registers!(field_debug_expr $reg [$($tokens)*] $field)
    };
    (field_debug_expr $reg:ident [W $($tokens:tt)*] $field:ident) => {};
    (field_debug_expr $reg:ident [$bit_index:literal] $field:ident) => {
        &$reg.read().$field().bit()
    };
    (field_debug_expr $reg:ident [$bit_index:literal; $($tokens:tt)*] $field:ident) => {
        &$reg.read().$field().bit()
    };
    (field_debug_expr $reg:ident [$bit_range:expr] $field:ident) => {
        &$reg.read().$field().bits()
    };
    (field_debug_expr $reg:ident [$bit_range:expr; $($tokens:tt)*] $field:ident) => {
        &$reg.read().$field().bits()
    };
    (field_debug_expr $reg:ident [$($tokens:tt)*] $field:ident) => {};
    (field_debug_stmts $reg:ident $dbg:ident $([$($tokens:tt)*] $Field:ident $field:ident,)*) => {
        $(
            $dbg.field(stringify!($field), impl_registers!(field_debug_expr $reg [$($tokens)*] $field));
        )*
    };
    (field_udebug_stmts $reg:ident $dbg:ident $([$($tokens:tt)*] $Field:ident $field:ident,)*) => {
        $(
            $dbg.field(stringify!($field), impl_registers!(field_debug_expr $reg [$($tokens)*] $field))?;
        )*
    };

    // Generate the index consts for the register map, with the total `COUNT` at the end.
    (map_indices $ix:expr, $IX:ident, $($IXs:ident),*) => {
        pub(crate) const $IX: usize = $ix;
        impl_registers!(map_indices $IX + 1, $($IXs),*);
    };
    (map_indices $ix:expr, $IX:ident) => {
        pub(crate) const $IX: usize = $ix;
        pub(crate) const COUNT: usize = $IX + 1;
    };

    (declare_register_mod $bits:literal $RegTy:ident; $addr:literal $Reg:ident $reg:ident [$($fields:tt)*]) => {
        pub mod $reg {
            use bitvec::prelude::*;
            use core::fmt;
            use super::{Address, Register, R, W};

            pub type Fields = bitarr!(for $bits, in Lsb0, $RegTy);

            #[derive(Clone, Copy, Eq, Hash, PartialEq)]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
            pub struct $Reg {
                #[cfg_attr(feature = "serde", serde(default, with = "serde_impl"))]
                fields: Fields,
            }

            impl_registers!(declare_fields $($fields)*);

            // Generate methods for reading from the fields.
            impl<'a> R<&'a $Reg> {
                impl_registers!(define_read_fields $($fields)*);

                /// The bits of the register.
                pub fn bits(&self) -> $RegTy {
                    (*self.0).into()
                }
            }

            impl_registers!(define_field_r_impls $Reg $($fields)*);

            // Generate methods for writing to the fields.
            impl<'a> W<&'a mut $Reg> {
                impl_registers!(define_write_fields $($fields)*);

                /// Reset the writable fields of the register to their default state.
                pub fn reset(&mut self) -> &mut Self {
                    let reg = self;
                    impl_registers!(field_reset_stmts reg $($fields)*);
                    reg
                }

                /// Set all bits of the register.
                pub fn bits(&mut self, bits: $RegTy) -> &mut Self {
                    *self.0 = $Reg::from(bits);
                    self
                }
            }

            impl_registers!(define_field_w_impls $Reg $($fields)*);

            impl $Reg {
                /// Provides read access to the individual fields of the register.
                pub fn read(&self) -> R<&Self> {
                    R(self)
                }

                /// Provides write access to the individual fields of the register.
                pub fn write(&mut self) -> W<&mut Self> {
                    W(self)
                }
            }

            impl Register for $Reg {
                const ADDRESS: Address = Address::$Reg;
            }

            #[allow(unused_mut)]
            impl Default for $Reg {
                fn default() -> Self {
                    let fields = Default::default();
                    let mut reg = Self { fields };
                    impl_registers!(field_default_stmts reg $($fields)*);
                    reg
                }
            }

            impl From<$RegTy> for $Reg {
                fn from(bits: $RegTy) -> Self {
                    let fields = BitArray::new([bits]);
                    Self { fields }
                }
            }

            impl Into<$RegTy> for $Reg {
                fn into(self) -> $RegTy {
                    let [bits] = self.fields.value();
                    bits
                }
            }

            impl fmt::Debug for $Reg {
                #[allow(unused_variables)]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    let reg = self;
                    let dbg = &mut f.debug_struct(stringify!($Reg));
                    impl_registers!(field_debug_stmts reg dbg $($fields)*);
                    dbg.finish()
                }
            }

            #[cfg(feature = "ufmt")]
            impl ufmt::uDebug for $Reg {
                fn fmt<W: ?Sized>(&self, f: &mut ufmt::Formatter<W>) -> Result<(), W::Error>
                where
                    W: ufmt::uWrite,
                {
                    let reg = self;
                    let dbg = &mut f.debug_struct(stringify!($Reg))?;
                    impl_registers!(field_udebug_stmts reg dbg $($fields)*);
                    dbg.finish()
                }
            }

            #[cfg(feature = "hash-32")]
            impl hash32::Hash for $Reg {
                fn hash<H>(&self, state: &mut H)
                where
                    H: hash32::Hasher,
                {
                    let u: $RegTy = (*self).into();
                    u.hash(state)
                }
            }

            #[cfg(feature = "serde")]
            pub mod serde_impl {
                use serde::{Deserialize, Deserializer, Serialize, Serializer};

                pub fn serialize<S>(arr: &super::Fields, s: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let [val]: [$RegTy; 1] = arr.value();
                    val.serialize(s)
                }

                pub fn deserialize<'de, D>(d: D) -> Result<super::Fields, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let val = $RegTy::deserialize(d)?;
                    Ok(super::Fields::from([val]))
                }
            }
        }
    };

    // Top-level invocation.
    (
        size_bits $bits:literal;
        data_type $RegTy:ident;
        $($addr:literal $Reg:ident $reg:ident [ $($fields:tt)* ],)*
    ) => {
        $(
            pub use $reg::$Reg;
        )*

        /// The set of implemented MIIM register addresses on the KSZ8863.
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
        #[repr(u8)]
        pub enum Address {
            $(
                $Reg = $addr,
            )*
        }

        /// A dynamic representation of a register's state.
        #[derive(Clone, Copy, Eq, Hash, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
        pub enum State {
            $(
                $Reg($Reg),
            )*
        }

        /// A map of the state of all registers in the `impl_registers` invocation.
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub struct Map {
            arr: MapArray,
        }

        /// The inner array storing all register state within a `Map`.
        ///
        /// Each register is laid out in the array in the order in which they are declared in the
        /// `impl_registers` macro. The `map_index` module is used internally to map register
        /// addresses and their state to the associated elements in the array.
        type MapArray = [State; map_index::COUNT];

        $(
            impl_registers!(declare_register_mod $bits $RegTy; $addr $Reg $reg [$($fields)*]);
        )*

        /// A private, unique index for each register into the `Map`'s inner array.
        #[allow(non_upper_case_globals)]
        mod map_index {
            impl_registers!(map_indices 0, $($Reg),*);
        }

        impl Address {
            /// All register addresses.
            pub const ALL: &'static [Self] = &[
                $(
                    Self::$Reg,
                )*
            ];
        }

        impl State {
            /// Construct a register state from its address and data represented as a `u32`.
            pub fn from_addr_and_data(addr: Address, data: $RegTy) -> Self {
                match addr {
                    $(
                        Address::$Reg => State::$Reg($Reg::from(data)),
                    )*
                }
            }

            /// Construct the default register state associated with the given address.
            pub fn from_addr_default(addr: Address) -> Self {
                match addr {
                    $(
                        Address::$Reg => State::$Reg(<_>::default()),
                    )*
                }
            }

            /// The address of the register with which this state is associated.
            pub fn addr(&self) -> Address {
                match *self {
                    $(
                        State::$Reg(_) => Address::$Reg,
                    )*
                }
            }

            /// Attempt to retrieve a reference to a register of type `R` from the dynamic register
            /// `State` representation.
            ///
            /// Returns an `Err` if the register type does not match.
            pub fn reg<R>(&self) -> Result<&R, crate::InvalidAddress>
            where
                R: 'static + Register,
            {
                match *self {
                    $(
                        Self::$Reg(ref r) => (r as &dyn core::any::Any)
                            .downcast_ref()
                            .ok_or(crate::InvalidAddress),
                    )*
                }
            }

            /// Attempt to retrieve a mutable reference to a register of type `R` from the dynamic
            /// register `State` representation.
            ///
            /// Returns an `Err` if the register type does not match.
            pub fn reg_mut<R>(&mut self) -> Result<&mut R, crate::InvalidAddress>
            where
                R: 'static + Register,
            {
                match *self {
                    $(
                        Self::$Reg(ref mut r) => (r as &mut dyn core::any::Any)
                            .downcast_mut()
                            .ok_or(crate::InvalidAddress),
                    )*
                }
            }
        }

        impl Map {
            /// The total number of documented registers in the TMC2209.
            pub const LEN: usize = map_index::COUNT;

            /// Read-only access to the register of the given type.
            pub fn reg<T>(&self) -> &T
            where
                T: 'static + Register,
            {
                self.state(T::ADDRESS)
                    .reg::<T>()
                    // We gaurantee that `Map` will always have state for each register, but need
                    // to avoid generating panicking branches, so we use an infinite loop rather
                    // than unwrap.
                    .unwrap_or_else(|_| loop {})
            }

            /// Mutable access to the register of the given type.
            pub fn reg_mut<T>(&mut self) -> &mut T
            where
                T: 'static + Register,
            {
                self.state_mut(T::ADDRESS)
                    .reg_mut::<T>()
                    // We gaurantee that `Map` will always have state for each register, but need
                    // to avoid generating panicking branches, so we use an infinite loop rather
                    // than unwrap.
                    .unwrap_or_else(|_| loop {})
            }

            /// Read-only access to the dynamic representation of the register state at the given
            /// address.
            pub fn state(&self, addr: Address) -> &State {
                match addr {
                    $(
                        // We gaurantee that `Map` will always have state for each register.
                        Address::$Reg => unsafe {
                            self.arr.get_unchecked(map_index::$Reg)
                        }
                    )*
                }
            }

            /// Mutable access to the dynamic representation of the register state at the given
            /// address.
            ///
            /// Note: This should remain private for internal use only, as the user should never be
            /// allowed to change the stored `State` to a different variant.
            fn state_mut(&mut self, addr: Address) -> &mut State {
                match addr {
                    $(
                        // We gaurantee that `Map` will always have state for each register.
                        Address::$Reg => unsafe {
                            self.arr.get_unchecked_mut(map_index::$Reg)
                        }
                    )*
                }
            }

            /// Update the given register state.
            pub fn set_state(&mut self, state: State) {
                *self.state_mut(state.addr()) = state;
            }

            // Generate the short-hand names for gaining direct access to typed register state.
            $(
                // TODO: Provide immutable access too and rename mutable access to $reg_mut.
                pub fn $reg(&mut self) -> &mut $Reg {
                    self.reg_mut::<$Reg>()
                }
            )*
        }

        impl Default for Map {
            fn default() -> Self {
                let arr = [$(
                    State::$Reg($Reg::default()),
                )*];
                Map { arr }
            }
        }

        impl From<State> for $RegTy {
            fn from(s: State) -> Self {
                match s {
                    $(
                        State::$Reg(r) => r.into(),
                    )*
                }
            }
        }

        $(
            impl From<$Reg> for State {
                fn from(r: $Reg) -> Self {
                    State::$Reg(r)
                }
            }

            impl core::convert::TryFrom<State> for $Reg {
                type Error = crate::InvalidAddress;
                fn try_from(state: State) -> Result<Self, Self::Error> {
                    match state {
                        State::$Reg(r) => Ok(r),
                        _ => Err(crate::InvalidAddress),
                    }
                }
            }
        )*

        impl From<Address> for u8 {
            fn from(addr: Address) -> Self {
                addr as u8
            }
        }

        impl core::convert::TryFrom<u8> for Address {
            type Error = crate::InvalidAddress;
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $addr => Ok(Address::$Reg),
                    )*
                    _ => Err(crate::InvalidAddress),
                }
            }
        }

        impl core::ops::Index<Address> for Map {
            type Output = State;
            fn index(&self, addr: Address) -> &Self::Output {
                self.state(addr)
            }
        }

        impl core::ops::IndexMut<Address> for Map {
            fn index_mut(&mut self, addr: Address) -> &mut Self::Output {
                self.state_mut(addr)
            }
        }

        impl core::fmt::Debug for State {
            #[allow(unused_variables)]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    $(
                        State::$Reg(ref r) => r.fmt(f),
                    )*
                }
            }
        }

        #[cfg(feature = "hash-32")]
        mod hash_32 {
            impl hash32::Hash for super::Address {
                fn hash<H>(&self, state: &mut H)
                where
                    H: hash32::Hasher,
                {
                    (*self as u8).hash(state)
                }
            }

            impl hash32::Hash for super::State {
                fn hash<H>(&self, state: &mut H)
                where
                    H: hash32::Hasher,
                {
                    let u: $RegTy = (*self).into();
                    u.hash(state)
                }
            }
        }
    };

    // Generate methods for short-hand access to each of the SMI registers.
    (
        size_bits $size_bits:literal;
        data_type $DataType:ident;
        smi_register_methods $Smi:ident $SmiReg:ident;
        $($addr:literal $Reg:ident $reg:ident [ $($fields:tt)* ],)*
    ) => {
        impl_registers! {
            size_bits $size_bits;
            data_type $DataType;
            $($addr $Reg $reg [ $($fields)* ],)*
        }

        impl<T> $Smi<T> {
            $(
                pub fn $reg(&mut self) -> $SmiReg<T, $Reg> {
                    self.reg::<$Reg>()
                }
            )*
        }
    };

    // Generate methods for short-hand access to each of the MIIM registers associated with a PHY.
    (
        size_bits $size_bits:literal;
        data_type $DataType:ident;
        miim_phy_register_methods $Phy:ident $PhyReg:ident;
        $($addr:literal $Reg:ident $reg:ident [ $($fields:tt)* ],)*
    ) => {
        impl_registers! {
            size_bits $size_bits;
            data_type $DataType;
            $($addr $Reg $reg [ $($fields)* ],)*
        }

        impl<'miim, T> $Phy<'miim, T> {
            $(
                pub fn $reg<'phy>(&'phy mut self) -> $PhyReg<'phy, 'miim, T, $Reg> {
                    self.reg::<$Reg>()
                }
            )*
        }
    };
}
