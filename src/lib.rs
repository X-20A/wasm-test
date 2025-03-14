use wasm_bindgen::prelude::*;
// use rand::Rng;

pub mod types;

use types::Ship;
use types::ShipType::*;


/// 入渠コストを計算して返す
/// 
/// ### 引数
/// - `ship`: 艦船オブジェクト
///
/// ### 返り値
/// - 戻り値はタプルで、最初の値が燃料コスト、2番目の値が鋼材コスト    
///   それぞれのコストはHPの減少に基づいて計算される    
///   Ship.fuelが存在しない場合は(0, 0)を返す
/// 
pub fn get_repair_cost(ship: Ship) -> (u32, u32) {
    let Some(fuel) = ship.fuel else { return (0, 0) };
    let base = (ship.max_hp - ship.current_hp) * fuel;
    (
        (base as f64 * 0.032).floor() as u32,
        (base as f64 * 0.06).floor() as u32
    )
}

/// 入渠時間を計算して返す
/// ### 引数
/// - `ship`: 艦船オブジェクト
/// ### 返り値
/// - 秒
pub fn get_repair_time(ship: Ship) -> f64 {
    if ship.current_hp >= ship.max_hp {
        return 0.0;
    }

    let base: f64;
    
    if ship.lv <= 11 {
        base = 10.0 * ship.lv as f64;
    } else {
        base = 5.0 * ship.lv as f64 + 10.0 * (f64::sqrt((ship.lv - 11) as f64).floor()) + 50.0;
    }

    let r#mod: f64;
    match ship.ship_type {
    	BB | BBV | CV | CVB | AR => r#mod = 2.0,
        CA | CAV | FBB | CVL | AS => r#mod = 1.5,
        SS | DE => r#mod = 0.5,
        _ => r#mod = 1.0,
    }

    (ship.max_hp - ship.current_hp) as f64 * base * r#mod + 30.0
}

/// 陣形が以下の組み合わせであったときにtrue、それ以外はfalseを返す    
/// 1: 単縦陣, 2: 複縦陣, 3: 輪形陣, 4: 梯形陣, 5: 単横陣    
/// 2 - 5    
/// 4 - 1    
/// 5 - 4    
/// 参考: https://x.gd/o1EpH 昼砲撃戦の命中率 > 陣形
/// ### 引数
/// - `our_formation`: 攻撃側の陣形ID
/// - `their_formation`: 回避側の陣形ID
/// ### 返り値
/// - 真偽値
pub fn formation_countered(our_formation_id: u32, their_formation_id: u32) -> bool {
    if our_formation_id == 2 && their_formation_id == 5 {
        return true;
    }
    if our_formation_id == 4 && their_formation_id == 1 {
        return true;
    }
    if our_formation_id == 5 && their_formation_id == 4 {
        return true;
    }
    false
}

/*
pub fn shell(
	aggressor_ship: Ship,
	defender_ship: Ship,
	sp_atk_id: u32,
	is_union: bool,
) {
	let mut double_attack: u32 = 0;
	let mut cv_ci: u32 = 0;

}*/

#[wasm_bindgen] // Functions that can be called from js
pub fn update_settings() -> String {
	"run update_settings".to_string()
}

#[wasm_bindgen] // Functions that can be called from js
pub fn sim() -> String {
	"run sim".to_string()
}

