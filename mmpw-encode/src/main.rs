use mmpw_validate::binstring::BinString;

pub struct PlayerData {
    mystery_box_status: u16,
    abra_bead_capacity: u8,
    items: [bool; 30],
    character_states: [u8; 10],
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
        todo!("need to write more stuff here");
    } else {
        writer.write_int(0, 1);
        for i in 0..30 {
            writer.write_int(if player_data.items[i] { 1 } else { 0 }, 1);
        }
    }
    writer.skip(22 + 19 + 6 + 2);
    let checksum = bs.calc_checksum();
    let mut writer = bs.writer();
    writer.skip(81);
    writer.write_int(checksum as i32, mmpw_validate::CKSUM_BITS);
    bs
}

fn main() {
    let data = PlayerData {
        mystery_box_status: 0,
        abra_bead_capacity: 0,
        items: [false; 30],
        character_states: [0; 10],
        cash: 0,
        abra_story: 0,
        final_trial_count: 0,
        rank: 0,
        time_played: 0,
        five_peg: false,
        seven_peg: false,
    };
    let mut s = encode(&data);
    println!("{}", s.to_alphanumeric(18));
    let key = mmpw_validate::binstring::hash_name(b"DEW");
    s.hash(&key);
    println!("{}", s.to_alphanumeric(18));
}
