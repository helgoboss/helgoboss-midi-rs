use crate::U7;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ControllerNumber(pub(crate) u8);

impl ControllerNumber {
    pub const BANK_SELECT: ControllerNumber = ControllerNumber(0x00);
    pub const MODULATION_WHEEL: ControllerNumber = ControllerNumber(0x01);
    pub const BREATH_CONTROLLER: ControllerNumber = ControllerNumber(0x02);
    pub const FOOT_CONTROLLER: ControllerNumber = ControllerNumber(0x04);
    pub const PORTAMENTO_TIME: ControllerNumber = ControllerNumber(0x05);
    pub const DATA_ENTRY_MSB: ControllerNumber = ControllerNumber(0x06);
    pub const CHANNEL_VOLUME: ControllerNumber = ControllerNumber(0x07);
    pub const BALANCE: ControllerNumber = ControllerNumber(0x08);
    pub const PAN: ControllerNumber = ControllerNumber(0x0A);
    pub const EXPRESSION_CONTROLLER: ControllerNumber = ControllerNumber(0x0B);
    pub const EFFECT_CONTROL_1: ControllerNumber = ControllerNumber(0x0C);
    pub const EFFECT_CONTROL_2: ControllerNumber = ControllerNumber(0x0D);
    pub const GENERAL_PURPOSE_CONTROLLER_1: ControllerNumber = ControllerNumber(0x10);
    pub const GENERAL_PURPOSE_CONTROLLER_2: ControllerNumber = ControllerNumber(0x11);
    pub const GENERAL_PURPOSE_CONTROLLER_3: ControllerNumber = ControllerNumber(0x12);
    pub const GENERAL_PURPOSE_CONTROLLER_4: ControllerNumber = ControllerNumber(0x13);
    pub const BANK_SELECT_LSB: ControllerNumber = ControllerNumber(0x20);
    pub const MODULATION_WHEEL_LSB: ControllerNumber = ControllerNumber(0x21);
    pub const BREATH_CONTROLLER_LSB: ControllerNumber = ControllerNumber(0x22);
    pub const FOOT_CONTROLLER_LSB: ControllerNumber = ControllerNumber(0x24);
    pub const PORTAMENTO_TIME_LSB: ControllerNumber = ControllerNumber(0x25);
    pub const DATA_ENTRY_MSB_LSB: ControllerNumber = ControllerNumber(0x26);
    pub const CHANNEL_VOLUME_LSB: ControllerNumber = ControllerNumber(0x27);
    pub const BALANCE_LSB: ControllerNumber = ControllerNumber(0x28);
    pub const PAN_LSB: ControllerNumber = ControllerNumber(0x2A);
    pub const EXPRESSION_CONTROLLER_LSB: ControllerNumber = ControllerNumber(0x2B);
    pub const EFFECT_CONTROL_1_LSB: ControllerNumber = ControllerNumber(0x2C);
    pub const EFFECT_CONTROL_2_LSB: ControllerNumber = ControllerNumber(0x2D);
    pub const GENERAL_PURPOSE_CONTROLLER_1_LSB: ControllerNumber = ControllerNumber(0x30);
    pub const GENERAL_PURPOSE_CONTROLLER_2_LSB: ControllerNumber = ControllerNumber(0x31);
    pub const GENERAL_PURPOSE_CONTROLLER_3_LSB: ControllerNumber = ControllerNumber(0x32);
    pub const GENERAL_PURPOSE_CONTROLLER_4_LSB: ControllerNumber = ControllerNumber(0x33);
    pub const DAMPER_PEDAL_ON_OFF: ControllerNumber = ControllerNumber(0x40);
    pub const PORTAMENTO_ON_OFF: ControllerNumber = ControllerNumber(0x41);
    pub const SOSTENUTO_ON_OFF: ControllerNumber = ControllerNumber(0x42);
    pub const SOFT_PEDAL_ON_OFF: ControllerNumber = ControllerNumber(0x43);
    pub const LEGATO_FOOTSWITCH: ControllerNumber = ControllerNumber(0x44);
    pub const HOLD_2: ControllerNumber = ControllerNumber(0x45);
    pub const SOUND_CONTROLLER_1: ControllerNumber = ControllerNumber(0x46);
    pub const SOUND_CONTROLLER_2: ControllerNumber = ControllerNumber(0x47);
    pub const SOUND_CONTROLLER_3: ControllerNumber = ControllerNumber(0x48);
    pub const SOUND_CONTROLLER_4: ControllerNumber = ControllerNumber(0x49);
    pub const SOUND_CONTROLLER_5: ControllerNumber = ControllerNumber(0x4A);
    pub const SOUND_CONTROLLER_6: ControllerNumber = ControllerNumber(0x4B);
    pub const SOUND_CONTROLLER_7: ControllerNumber = ControllerNumber(0x4C);
    pub const SOUND_CONTROLLER_8: ControllerNumber = ControllerNumber(0x4D);
    pub const SOUND_CONTROLLER_9: ControllerNumber = ControllerNumber(0x4E);
    pub const SOUND_CONTROLLER_10: ControllerNumber = ControllerNumber(0x4F);
    pub const GENERAL_PURPOSE_CONTROLLER_5: ControllerNumber = ControllerNumber(0x50);
    pub const GENERAL_PURPOSE_CONTROLLER_6: ControllerNumber = ControllerNumber(0x51);
    pub const GENERAL_PURPOSE_CONTROLLER_7: ControllerNumber = ControllerNumber(0x52);
    pub const GENERAL_PURPOSE_CONTROLLER_8: ControllerNumber = ControllerNumber(0x53);
    pub const PORTAMENTO_CONTROL: ControllerNumber = ControllerNumber(0x54);
    pub const HIGH_RESOLUTION_VELOCITY_PREFIX: ControllerNumber = ControllerNumber(0x58);
    pub const EFFECTS_1_DEPTH: ControllerNumber = ControllerNumber(0x5B);
    pub const EFFECTS_2_DEPTH: ControllerNumber = ControllerNumber(0x5C);
    pub const EFFECTS_3_DEPTH: ControllerNumber = ControllerNumber(0x5D);
    pub const EFFECTS_4_DEPTH: ControllerNumber = ControllerNumber(0x5E);
    pub const EFFECTS_5_DEPTH: ControllerNumber = ControllerNumber(0x5F);
    pub const DATA_INCREMENT: ControllerNumber = ControllerNumber(0x60);
    pub const DATA_DECREMENT: ControllerNumber = ControllerNumber(0x61);
    pub const NON_REGISTERED_PARAMETER_NUMBER_LSB: ControllerNumber = ControllerNumber(0x62);
    pub const NON_REGISTERED_PARAMETER_NUMBER_MSB: ControllerNumber = ControllerNumber(0x63);
    pub const REGISTERED_PARAMETER_NUMBER_LSB: ControllerNumber = ControllerNumber(0x64);
    pub const REGISTERED_PARAMETER_NUMBER_MSB: ControllerNumber = ControllerNumber(0x65);
    pub const ALL_SOUND_OFF: ControllerNumber = ControllerNumber(0x78);
    pub const RESET_ALL_CONTROLLERS: ControllerNumber = ControllerNumber(0x79);
    pub const LOCAL_CONTROL_ON_OFF: ControllerNumber = ControllerNumber(0x7A);
    pub const ALL_NOTES_OFF: ControllerNumber = ControllerNumber(0x7B);
    pub const OMNI_MODE_OFF: ControllerNumber = ControllerNumber(0x7C);
    pub const OMNI_MODE_ON: ControllerNumber = ControllerNumber(0x7D);
    pub const MONO_MODE_ON: ControllerNumber = ControllerNumber(0x7E);
    pub const POLY_MODE_ON: ControllerNumber = ControllerNumber(0x7F);

    pub const COUNT: u8 = 128;

    pub fn new(number: u8) -> ControllerNumber {
        assert!(number < ControllerNumber::COUNT);
        ControllerNumber(number)
    }

    pub const unsafe fn new_unchecked(number: u8) -> ControllerNumber {
        ControllerNumber(number)
    }

    pub fn can_be_part_of_14_bit_message(&self) -> bool {
        self.0 < 64
    }

    pub fn can_act_as_14_bit_msb(&self) -> bool {
        self.0 < 32
    }

    pub fn get_corresponding_14_bit_lsb(&self) -> Result<ControllerNumber, ()> {
        if !self.can_act_as_14_bit_msb() {
            return Err(());
        }
        Ok(ControllerNumber(self.0 + 32))
    }

    pub fn can_be_part_of_parameter_number_message(&self) -> bool {
        matches!(self.0, 98 | 99 | 100 | 101 | 38 | 6)
    }
}

impl From<U7> for ControllerNumber {
    fn from(value: U7) -> Self {
        ControllerNumber(value.into())
    }
}

impl From<ControllerNumber> for U7 {
    fn from(value: ControllerNumber) -> Self {
        U7(value.into())
    }
}

impl From<ControllerNumber> for u8 {
    fn from(value: ControllerNumber) -> Self {
        value.0
    }
}

impl From<ControllerNumber> for usize {
    fn from(value: ControllerNumber) -> Self {
        value.0 as usize
    }
}

pub fn controller_number(number: u8) -> ControllerNumber {
    ControllerNumber::new(number)
}
