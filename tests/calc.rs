use kssw::{
	get_repair_cost,
	get_repair_time,
	formation_countered,
};

use kssw::types::{Ship, ShipType};

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_repair_cost() {
		let result = get_repair_cost(generate_mock_ship());

		assert_eq!(result.0, 518);
		assert_eq!(result.1, 972);
	}

	#[test]
	fn test_get_repair_time() {
		let result = get_repair_time(generate_mock_ship());
		dbg!(result);

		assert_eq!(result, 114330.0);
	}

	#[test]
	fn test_formation_countered() {
		let result1 = formation_countered(2, 5);
		let result2 = formation_countered(4, 1);
		let result3 = formation_countered(5, 4);
		let result4 = formation_countered(1, 4);

		assert_eq!(result1, true);
		assert_eq!(result2, true);
		assert_eq!(result3, true);
		assert_eq!(result4, false);
	}
}




fn generate_mock_ship() -> Ship {
    Ship {
        id: 341,
        name: String::from("長門改二(モック)"),
		is_our_side: true,
        lv: 99,
        current_hp: 1,
        max_hp: 91,
        fp: 91,
        tp: 0,
        aa: 100,
        ar: 110,
        ev: 70,
        asw: 0,
        los: 55,
        luk: 40,
        rng: 3,
		plane_slots: vec![3,3,6,3],
		ship_type: ShipType::BB,
        fuel: Some(180),
        ammo: Some(225),
    }
}
