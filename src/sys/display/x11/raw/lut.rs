// devela::sys::display::x11::raw::lut
//
//! Defines [`LUT_SCANCODE_TO_KEY`].
//

use crate::{Key, KeyMedia as Media, KeyMod as Mod, KeyPad as Pad, init_array, whilst};

/// Scancodes to keys.
//
// NOTE: extracted from /usr/include/linux/input-event-codes.h
/*
 * Keys and buttons
 *
 * Most of the keys/buttons are modeled after USB HUT 1.12
 * (see http://www.usb.org/developers/hidpage).
 * Abbreviations in the comments:
 * AC - Application Control
 * AL - Application Launch Button
 * SC - System Control
 */
pub(crate) const LUT_SCANCODE_TO_KEY: [Key; 256] = {
    const fn init(n: usize) -> Key {
        Key::Scancode(n as u16)
    }
    let mut t = init_array![const_fn [Key; 256], "safe_sys", "unsafe_array", init, Key::Unknown];

    // t[0] = KEY_RESERVED;
    t[1] = Key::Escape;
    t[2] = Key::Digit1;
    t[3] = Key::Digit2;
    t[4] = Key::Digit3;
    t[5] = Key::Digit4;
    t[6] = Key::Digit5;
    t[7] = Key::Digit6;
    t[8] = Key::Digit7;
    t[9] = Key::Digit8;
    t[10] = Key::Digit9;
    t[11] = Key::Digit0;
    t[12] = Key::Minus;
    t[13] = Key::Equal;
    t[14] = Key::Backspace;
    t[15] = Key::Tab;
    t[16] = Key::Q;
    t[17] = Key::W;
    t[18] = Key::E;
    t[19] = Key::R;
    t[20] = Key::T;
    t[21] = Key::Y;
    t[22] = Key::U;
    t[23] = Key::I;
    t[24] = Key::O;
    t[25] = Key::P;
    t[26] = Key::BracketLeft;
    t[27] = Key::BracketRight;
    t[28] = Key::Enter;
    t[29] = Key::Mod(Mod::LeftControl);
    t[30] = Key::A;
    t[31] = Key::S;
    t[32] = Key::D;
    t[33] = Key::F;
    t[34] = Key::G;
    t[35] = Key::H;
    t[36] = Key::J;
    t[37] = Key::K;
    t[38] = Key::L;
    t[39] = Key::Semicolon;
    t[40] = Key::Quote;
    t[41] = Key::Backquote; // KEY_GRAVE;
    t[42] = Key::Mod(Mod::LeftShift);
    t[43] = Key::Backslash;
    t[44] = Key::Z;
    t[45] = Key::X;
    t[46] = Key::C;
    t[47] = Key::V;
    t[48] = Key::B;
    t[49] = Key::N;
    t[50] = Key::M;
    t[51] = Key::Comma;
    t[52] = Key::Dot;
    t[53] = Key::Slash;
    t[54] = Key::Mod(Mod::RightShift);
    t[55] = Key::Pad(Pad::Multiply);
    t[56] = Key::Mod(Mod::LeftAlt);
    t[57] = Key::Space;
    t[58] = Key::CapsLock;
    t[59] = Key::Fn(1);
    t[60] = Key::Fn(2);
    t[61] = Key::Fn(3);
    t[62] = Key::Fn(4);
    t[63] = Key::Fn(5);
    t[64] = Key::Fn(6);
    t[65] = Key::Fn(7);
    t[66] = Key::Fn(8);
    t[67] = Key::Fn(9);
    t[68] = Key::Fn(10);
    t[69] = Key::NumLock;
    t[70] = Key::ScrollLock;
    t[71] = Key::Pad(Pad::Num7);
    t[72] = Key::Pad(Pad::Num8);
    t[73] = Key::Pad(Pad::Num9);
    t[74] = Key::Pad(Pad::Subtract);
    t[75] = Key::Pad(Pad::Num4);
    t[76] = Key::Pad(Pad::Num5);
    t[77] = Key::Pad(Pad::Num6);
    t[78] = Key::Pad(Pad::Add);
    t[79] = Key::Pad(Pad::Num1);
    t[80] = Key::Pad(Pad::Num2);
    t[81] = Key::Pad(Pad::Num3);
    t[82] = Key::Pad(Pad::Num0);
    t[83] = Key::Pad(Pad::Decimal);

    // t[85] = KEY_ZENKAKUHANKAKU;
    t[86] = Key::IntlBackslash; // KEY_102ND;
    t[87] = Key::Fn(11);
    t[88] = Key::Fn(12);
    // t[89] = KEY_RO;
    // t[90] = KEY_KATAKANA;
    // t[91] = KEY_HIRAGANA;
    // t[92] = KEY_HENKAN;
    // t[93] = KEY_KATAKANAHIRAGANA;
    // t[94] = KEY_MUHENKAN;
    // t[95] = KEY_KPJPCOMMA;
    t[96] = Key::Pad(Pad::Enter);
    t[97] = Key::Mod(Mod::RightControl);
    t[98] = Key::Pad(Pad::Divide);
    // t[99] = KEY_SYSRQ;

    t[100] = Key::Mod(Mod::RightAlt); // Key::Mod(Mod::IsoLevel3Shift);

    // t[101] = KEY_LINEFEED;
    t[102] = Key::Home;
    t[103] = Key::Up;
    t[104] = Key::PageUp;
    t[105] = Key::Left;
    t[106] = Key::Right;
    t[107] = Key::End;
    t[108] = Key::Down;
    t[109] = Key::PageDown;
    t[110] = Key::Insert;
    t[111] = Key::Delete;
    //t[112] = KEY_MACRO;
    t[113] = Key::Media(Media::MuteVolume);
    t[114] = Key::Media(Media::LowerVolume);
    t[115] = Key::Media(Media::RaiseVolume);
    t[116] = Key::Media(Media::Power); /* SC System Power Down */
    t[117] = Key::Pad(Pad::Equal);
    // t[118] = Key::Pad(Pad::PlusMinus);
    t[119] = Key::Pause;
    // t[120] = KEY_SCALE; /* AL Compiz Scale (Expose) */
    t[121] = Key::Pad(Pad::Comma);
    // t[122] = KEY_HANGEUL; // KEY_HANGUEL
    // t[123] = KEY_HANJA;
    // t[124] = KEY_YEN;
    t[125] = Key::Mod(Mod::LeftSuper);
    t[126] = Key::Mod(Mod::RightSuper);
    // t[127] = KEY_COMPOSE;

    // t[128] = KEY_STOP; /* AC Stop */
    // t[129] = KEY_AGAIN;
    // t[130] = KEY_PROPS; /* AC Properties */
    // t[131] = KEY_UNDO; /* AC Undo */
    // t[132] = KEY_FRONT;
    // t[133] = KEY_COPY; /* AC Copy */
    // t[134] = KEY_OPEN; /* AC Open */
    // t[135] = KEY_PASTE; /* AC Paste */
    // t[136] = KEY_FIND; /* AC Search */
    // t[137] = KEY_CUT; /* AC Cut */
    // t[138] = KEY_HELP; /* AL Integrated Help Center */
    t[139] = Key::Menu; /* Menu (show menu) */
    // t[140] = KEY_CALC; /* AL Calculator */
    // t[141] = KEY_SETUP;
    t[142] = Key::Media(Media::Sleep); /* SC System Sleep */
    t[143] = Key::Media(Media::Wake); /* System Wake Up */
    // t[144] = KEY_FILE; /* AL Local Machine Browser */
    // t[145] = KEY_SENDFILE;
    // t[146] = KEY_DELETEFILE;
    // t[147] = KEY_XFER;
    // t[148] = KEY_PROG1;
    // t[149] = KEY_PROG2;
    // t[150] = KEY_WWW; /* AL Internet Browser */
    // t[151] = KEY_MSDOS;
    // t[152] = KEY_COFFEE; KEY_SCREENLOCK /* AL Terminal Lock/Screensaver */
    // t[153] = KEY_ROTATE_DISPLAY; KEY_DIRECTION /* Display orientation for e.g. tablets */
    // t[154] = KEY_CYCLEWINDOWS;
    // t[155] = KEY_MAIL;
    // t[156] = KEY_BOOKMARKS; /* AC Bookmarks */
    // t[157] = KEY_COMPUTER;
    // t[158] = KEY_BACK; /* AC Back */
    // t[159] = KEY_FORWARD; /* AC Forward */
    // t[160] = KEY_CLOSECD;
    t[161] = Key::Media(Media::Eject);
    // t[162] = KEY_EJECTCLOSECD;
    t[163] = Key::Media(Media::Next);
    t[164] = Key::Media(Media::PlayPause);
    t[165] = Key::Media(Media::Previous);
    t[166] = Key::Media(Media::Stop);
    t[167] = Key::Media(Media::Record);
    t[168] = Key::Media(Media::Rewind);
    // t[169] = KEY_PHONE; /* Media Select Telephone */
    // t[170] = KEY_ISO;
    // t[171] = KEY_CONFIG; /* AL Consumer Control Configuration */
    // t[172] = KEY_HOMEPAGE; /* AC Home */
    // t[173] = KEY_REFRESH; /* AC Refresh */
    // t[174] = KEY_EXIT; /* AC Exit */
    // t[175] = KEY_MOVE;
    // t[176] = KEY_EDIT;
    // t[177] = KEY_SCROLLUP;
    // t[178] = KEY_SCROLLDOWN;
    // t[179] = KEY_KPLEFTPAREN;
    // t[180] = KEY_KPRIGHTPAREN;
    // t[181] = KEY_NEW; /* AC New */
    // t[182] = KEY_REDO; /* AC Redo/Repeat */
    t[183] = Key::Fn(13);
    t[184] = Key::Fn(14);
    t[185] = Key::Fn(15);
    t[186] = Key::Fn(16);
    t[187] = Key::Fn(17);
    t[188] = Key::Fn(18);
    t[189] = Key::Fn(19);
    t[190] = Key::Fn(20);
    t[191] = Key::Fn(21);
    t[192] = Key::Fn(22);
    t[193] = Key::Fn(23);
    t[194] = Key::Fn(24);

    t[200] = Key::Media(Media::Play);
    t[201] = Key::Media(Media::Pause);
    // t[202] = KEY_PROG3;
    // t[203] = KEY_PROG4;
    // t[204] = KEY_ALL_APPLICATIONS; KEY_DASHBOARD /* AC Desktop Show All Applications */
    // t[205] = KEY_SUSPEND;
    // t[206] = KEY_CLOSE; /* AC Close */
    t[207] = Key::Media(Media::Play);
    t[208] = Key::Media(Media::FastForward);
    t[209] = Key::Media(Media::BassBoost);
    t[210] = Key::PrintScreen; /* AC Print */
    // t[211] = KEY_HP;
    // t[212] = KEY_CAMERA;
    // t[213] = KEY_SOUND;
    // t[214] = KEY_QUESTION;
    // t[215] = KEY_EMAIL;
    // t[216] = KEY_CHAT;
    // t[217] = KEY_SEARCH;
    // t[218] = KEY_CONNECT;
    // t[219] = KEY_FINANCE; /* AL Checkbook/Finance */
    // t[220] = KEY_SPORT;
    // t[221] = KEY_SHOP;
    // t[222] = KEY_ALTERASE;
    // t[223] = KEY_CANCEL; /* AC Cancel */
    t[224] = Key::Media(Media::BrightnessDown);
    t[225] = Key::Media(Media::BrightnessUp);
    // t[226] = KEY_MEDIA; // MAYBE: Media::MediaSelect or Media::LaunchMedia

    // t[227] = KEY_SWITCHVIDEOMODE; /* Cycle between available video
    //  outputs (Monitor/LCD/TV-out/etc) */
    // t[228] = KEY_KBDILLUMTOGGLE;
    // t[229] = KEY_KBDILLUMDOWN;
    // t[230] = KEY_KBDILLUMUP;

    // t[231] = KEY_SEND; /* AC Send */
    // t[232] = KEY_REPLY; /* AC Reply */
    // t[233] = KEY_FORWARDMAIL; /* AC Forward Msg */
    // t[234] = KEY_SAVE; /* AC Save */
    // t[235] = KEY_DOCUMENTS;

    // t[236] = KEY_BATTERY;
    // t[237] = KEY_BLUETOOTH;
    // t[238] = KEY_WLAN;
    // t[239] = KEY_UWB;

    t[240] = Key::Unknown;

    // t[241] = KEY_VIDEO_NEXT; /* drive next video source */
    // t[242] = KEY_VIDEO_PREV; /* drive previous video source */
    // t[243] = KEY_BRIGHTNESS_CYCLE; /* brightness up, after max is min */
    // t[244] = KEY_BRIGHTNESS_AUTO; /* Set Auto Brightness: manual brightness control is off, rely on ambient */
    // KEY_BRIGHTNESS_ZERO KEY_BRIGHTNESS_AUTO
    // t[245] = KEY_DISPLAY_OFF; /* display device to off state */
    // t[246] = KEY_WWAN; /* Wireless WAN (LTE, UMTS, GSM, etc.) */
    // KEY_WIMAX KEY_WWAN
    // t[247] = KEY_RFKILL; /* Key that controls all radios */
    t[248] = Key::Media(Media::MicrophoneMute); /* Mute / unmute the microphone */

    /* Code 255 is reserved for special needs of AT keyboard driver */

    t
};

// MAYBE sparse static map option for full USB HID coverage
// if sc < 256 { LUT[sc] } else { linear search EXTRA_KEYS }
// const EXTRA_KEYS: &[(u32, Key)] = &[
//     (0x209, Key::Media(Media::FastReverse)), // 521
//     (0x275, Key::Media(Media::FastReverse)), // 629
//     (0x535, Key::Media(Media::MicrophoneVolumeUp), // 217
//     (0x536, Key::Media(MEdia::MicrophoneVolumeDown), // 218
//     // ...
// ];
