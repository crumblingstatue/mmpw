use mmpw_validate::binstring::BinString;

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

fn main() {
    let data = PlayerData {
        mystery_box_status: 60,
        abra_bead_capacity: 6,
        items: [true; 30],
        chat_states: [2; 10],
        cash: 999999,
        abra_story: 0,
        final_trial_count: 0,
        rank: 50,
        time_played: 0,
        five_peg: true,
        seven_peg: true,
    };
    let mut s = encode(&data);
    println!("{}", s.to_alphanumeric(18));
    let key = mmpw_validate::binstring::hash_name(b"DEW");
    s.hash(&key);
    println!("{}", s.to_alphanumeric(18));
}
