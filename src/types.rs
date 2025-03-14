pub enum ShipType {
	DD,
	CL,
	CA,
	CAV,
	BBV,
	FBB,
	BB,
	CVL,
	CV,
	CVB,
	DE,
	AO,
	CLT,
	AV,
	SS,
	SSV,
	AS,
	CT,
	LHA,
	AR,
}

pub struct Ship {
	pub id: u32,
	pub name: String,
	pub is_our_side: bool,
	pub lv: u32,
	pub current_hp: u32,
	pub max_hp: u32,
	pub fp: u32,
	pub tp: u32,
	pub aa: u32,
	pub ar: u32,
	pub ev: u32,
	pub asw: u32,
	pub los: u32,
	pub luk: u32,
	pub rng: u32,
	pub plane_slots: Vec<u32>,
	pub ship_type: ShipType,
	pub fuel: Option<u32>,
	pub ammo: Option<u32>,
}