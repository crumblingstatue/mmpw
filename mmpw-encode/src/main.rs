use fltk::{app, button::*, group::*, input::*, output::*, valuator::*, window::*};
use mmpw_validate::binstring::BinString;

const PASSWORD_CASH: [u32; 64] = [
    0, 100, 200, 300, 400, 500, 700, 900, 1100, 1300, 1500, 1600, 1900, 2100, 2400, 2700, 3100,
    3500, 3900, 4400, 5000, 5700, 6400, 7300, 8200, 9300, 11000, 12000, 14000, 15000, 17000, 20000,
    22000, 25000, 28000, 32000, 36000, 41000, 47000, 53000, 60000, 68000, 77000, 87000, 98000,
    110000, 130000, 140000, 160000, 180000, 210000, 230000, 260000, 300000, 340000, 380000, 440000,
    490000, 560000, 630000, 710000, 810000, 920000, 1040000,
];
const PASSWORD_TIME_PLAYED: [u32; 64] = [
    0, 110, 220, 430, 610, 820, 1030, 1240, 1550, 1860, 2070, 2380, 2740, 3150, 3620, 4160, 4790,
    5510, 6330, 7280, 8370, 9630, 11080, 12740, 14650, 16840, 19370, 22280, 25620, 29460, 33880,
    38960, 44800, 51530, 59250, 68140, 78360, 90120, 103640, 119180, 137060, 157620, 181260,
    208450, 239720, 275670, 317020, 364580, 419260, 482150, 554480, 637650, 733300, 843290, 969780,
    1115250, 1282540, 1474920, 1696160, 1950580, 2243170, 2579650, 2966590, 3600000,
];
const ITEM_NAMES: [&str; 30] = [
    "Cornstarch Cookies",
    "Graphing Calculator",
    "Hydrogen Peroxide",
    "Acetone",
    "Romantic Incense",
    "Puzzle Keys",
    "Diamond Puzzle Key",
    "Mars Software",
    "Venus Software",
    "Magnetic Desk Toy",
    "Grey Anal Beads",
    "Purple Anal Beads",
    "Huge Glass Anal Beads",
    "Huge Grey Anal Beads",
    "Fruit Bugs",
    "Marshmallow Bugs",
    "Spooky Bugs",
    "RetroPie",
    "Assorted Gloves",
    "Insulated Gloves",
    "Ghost Gloves",
    "Vanishing Gloves",
    "Streaming Package",
    "Vibrator",
    "Blue Dildo",
    "Gummy Dildo",
    "Huge Dildo",
    "Happy Meal Toys",
    "Pizza Coupons",
    "???",
];

pub struct PlayerData {
    mystery_box_status: u16,
    abra_bead_capacity: u8,
    items: [bool; 30],
    chat_states: [u8; 10],
    cash: u32,
    abra_story: u8,
    final_trial_count: u8,
    rank: u8,
    time_played: u32,
    five_peg: bool,
    seven_peg: bool,
}

fn cash_index(cash: u32) -> usize {
    let mut idx = 0;
    (1..PASSWORD_CASH.len()).for_each(|i| {
        if cash >= PASSWORD_CASH[i] - 50 {
            idx = i;
        }
    });
    idx
}

pub fn encode(player_data: &PlayerData) -> BinString {
    let mut bs = BinString::zeroed();
    let n_items = player_data.items.iter().filter(|b| **b).count();
    let has_every_item = n_items == 30;
    let mut writer = bs.writer();
    if has_every_item {
        writer.write_int(1, 1);
        writer.write_int(player_data.mystery_box_status as i32, 12);
        writer.write_int(player_data.abra_bead_capacity as i32, 8);
        writer.write_int(326, 10);
    } else {
        writer.write_int(0, 1);
        for i in 0..30 {
            writer.write_int(if player_data.items[i] { 1 } else { 0 }, 1);
        }
    }
    writer.write_int(player_data.chat_states[0] as i32, 2);
    writer.write_int(player_data.chat_states[1] as i32, 3);
    writer.write_int(player_data.chat_states[2] as i32, 2);
    writer.write_int(player_data.chat_states[3] as i32, 2);
    writer.write_int(player_data.chat_states[4] as i32, 3);
    writer.write_int(player_data.chat_states[5] as i32, 2);
    writer.write_int(player_data.chat_states[6] as i32, 2);
    writer.write_int(player_data.chat_states[7] as i32, 2);
    writer.write_int(player_data.chat_states[8] as i32, 2);
    writer.write_int(player_data.chat_states[9] as i32, 2);
    writer.write_int(player_data.cash as i32, 6);
    writer.write_int(player_data.abra_story as i32, 3);
    writer.write_int(player_data.final_trial_count as i32, 3);
    writer.write_int(player_data.rank as i32, 7);
    writer.write_int(player_data.time_played as i32, 6);
    writer.write_int(if player_data.five_peg { 1 } else { 0 }, 1);
    writer.write_int(if player_data.seven_peg { 1 } else { 0 }, 1);
    let checksum = bs.calc_checksum();
    let mut writer = bs.writer();
    writer.skip(81);
    writer.write_int(checksum as i32, mmpw_validate::CKSUM_BITS);
    bs
}

fn bounded_int_input(label: &str, min: i32, max: i32) -> ValueInput {
    let mut inp = ValueInput::default().with_label(label).with_size(100, 32);
    inp.set_precision(0);
    inp.set_bounds(min as f64, max as f64);
    inp
}

#[derive(Clone, Copy)]
enum Msg {
    MoneyInpChanged,
    RankInpChanged,
    MysteryBoxInpChanged,
    AbraBeadInpChanged,
    GenerateClicked,
    ItemClicked,
    AllItemsClicked,
}

fn clamp_valuator(w: &mut impl ValuatorExt) {
    let val = w.value();
    let clamped = w.clamp(val);
    w.set_value(clamped);
}

fn main() {
    let app = app::App::default();
    let (s, r) = app::channel::<Msg>();
    let mut wind =
        Window::new(0, 0, 640, 640, "Monster Mind Password Generator v0.1").center_screen();
    let mut pack = Pack::default()
        .with_size(wind.w() - 170, wind.h())
        .with_pos(150, 0);
    pack.set_spacing(8);
    let mut name_inp = Input::default().with_label("Name").with_size(0, 32);
    name_inp.set_maximum_size(17);
    let mut pack2 = Pack::default().with_size(0, 32);
    pack2.set_type(PackType::Horizontal);
    pack2.set_spacing(100);
    let mut money_inp = bounded_int_input("Cash", 0, 2_000_000);
    let mut money_rounded = ValueOutput::default()
        .with_label("Rounded")
        .with_size(200, 40);
    money_rounded.set_precision(0);
    pack2.end();
    money_inp.emit(s, Msg::MoneyInpChanged);
    let mut pack2 = Pack::default().with_size(0, 32);
    pack2.set_spacing(8);
    pack2.set_type(PackType::Horizontal);
    let mut rank_inp = bounded_int_input("Puzzle rank", 0, 65);
    rank_inp.set_size(40, 0);
    rank_inp.emit(s, Msg::RankInpChanged);
    let five_pin_chk = CheckButton::default()
        .with_size(150, 0)
        .with_label("5 peg unlocked");
    let seven_pin_chk = CheckButton::default()
        .with_size(150, 0)
        .with_label("7 peg unlocked");
    pack2.end();
    let mut pack2 = Pack::default();
    pack2.end();
    let mut buttons = Vec::new();
    for (i, &name) in ITEM_NAMES.iter().enumerate() {
        let cond = i % 2 == 0;
        if cond {
            pack2 = Pack::default().with_size(0, 12);
            pack2.set_spacing(4);
            pack2.set_type(PackType::Horizontal);
        }
        let mut but = CheckButton::default().with_label(name).with_size(200, 0);
        but.emit(s, Msg::ItemClicked);
        buttons.push(but);
        if !cond {
            pack2.end();
        }
    }
    pack2.end();
    // All items, stuff that depends on it
    let mut all_items_chk = CheckButton::default()
        .with_label("Got all items")
        .with_size(0, 32);
    all_items_chk.emit(s, Msg::AllItemsClicked);
    let mut all_items_pak = Pack::default().with_size(0, 32);
    all_items_pak.set_spacing(200);
    all_items_pak.set_type(PackType::Horizontal);
    let mut mystery_box_inp = bounded_int_input("Myst. boxes bought", 0, 9999);
    mystery_box_inp.emit(s, Msg::MysteryBoxInpChanged);
    let mut abra_bead_inp = bounded_int_input("Abra bead capacity", 0, 255);
    abra_bead_inp.emit(s, Msg::AbraBeadInpChanged);
    all_items_pak.end();
    all_items_pak.deactivate();
    // Generate button + output
    let mut pack2 = Pack::default().with_size(0, 40);
    pack2.set_type(PackType::Horizontal);
    pack2.set_spacing(16);
    let mut out = Output::default();
    out.set_label("Password");
    out.set_size(200, 0);
    let mut button = Button::default();
    button.set_label("Generate");
    button.set_size(80, 0);
    button.emit(s, Msg::GenerateClicked);
    pack2.end();
    pack.end();
    wind.end();
    wind.show();
    let mut cash_index_val = 0;
    while app.wait() {
        if let Some(msg) = r.recv() {
            macro_rules! item_change_routine {
                () => {{
                    let all = buttons.iter().all(|b| b.is_checked());
                    if all {
                        all_items_pak.activate();
                    } else {
                        all_items_pak.deactivate();
                    }
                    all
                }};
            }
            match msg {
                Msg::MoneyInpChanged => {
                    clamp_valuator(&mut money_inp);
                    let val = money_inp.value();
                    cash_index_val = cash_index(val as u32);
                    money_rounded.set_value(PASSWORD_CASH[cash_index_val] as f64);
                }
                Msg::RankInpChanged => clamp_valuator(&mut rank_inp),
                Msg::MysteryBoxInpChanged => clamp_valuator(&mut mystery_box_inp),
                Msg::AbraBeadInpChanged => clamp_valuator(&mut abra_bead_inp),
                Msg::GenerateClicked => {
                    let mut items = [false; 30];
                    for (i, b) in buttons.iter().enumerate() {
                        items[i] = b.is_checked();
                    }
                    let player_data = PlayerData {
                        mystery_box_status: mystery_box_inp.value() as u16,
                        abra_bead_capacity: abra_bead_inp.value() as u8,
                        items,
                        chat_states: [2; 10],
                        cash: cash_index_val as u32,
                        abra_story: 0,
                        final_trial_count: 0,
                        rank: rank_inp.value() as u8,
                        time_played: 0,
                        five_peg: five_pin_chk.is_checked(),
                        seven_peg: seven_pin_chk.is_checked(),
                    };
                    let pw = encode(&player_data);
                    if mmpw_validate::validate_bin(&pw) {
                        let pw = pretty_print_password(&pw, &name_inp.value());
                        out.set_value(&pw);
                    } else {
                        println!(
                            "Invalid pw: {:?} raw: {:?}",
                            pretty_print_password(&pw, &name_inp.value()),
                            &pw
                        );
                        out.set_value("[invalid password]");
                    }
                }
                Msg::ItemClicked => {
                    let all = item_change_routine!();
                    all_items_chk.set_checked(all);
                }
                Msg::AllItemsClicked => {
                    let checked = all_items_chk.is_checked();
                    for b in buttons.iter() {
                        b.set_checked(checked);
                    }
                    item_change_routine!();
                }
            }
        }
    }
}

fn pretty_print_password(pw: &BinString, name: &str) -> String {
    let mut pw = pw.clone();
    let key = mmpw_validate::binstring::hash_name(name.as_bytes());
    pw.hash(&key);
    let mut pw = pw.to_alphanumeric(18);
    pw.insert(6, ' ');
    pw.insert(13, ' ');
    pw
}
