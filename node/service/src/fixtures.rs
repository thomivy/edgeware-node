use straightedge_primitives::{BlockNumber, Balance, AuraId, AccountId};
use grandpa::AuthorityId as GrandpaId;
use im_online::ed25519::{AuthorityId as ImOnlineId};
use primitives::crypto::UncheckedInto;
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use serde_json::{Result};
use std::fs::File;
use std::io::Read;
use hex::FromHex;

#[derive(Serialize, Deserialize)]
struct Allocation {
    balances: Vec<(String, String)>,
    vesting: Vec<(String, u32, u32, String)>,
    validators: Vec<(String, String, String, String)>,
}

pub fn get_spec_allocation() 
	-> Result<(
		Vec<(AccountId, Balance)>,
		Vec<(AccountId, BlockNumber, BlockNumber, Balance)>,
	)> {
    let mut file = File::open("node/service/src/genesis.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

	let json: Allocation = serde_json::from_str(&data)?;;
	let balances_json = json.balances;
	let vesting_json = json.vesting;

	let balances: Vec<(AccountId, Balance)> = balances_json.into_iter().map(|e| {
		return (
			<[u8; 32]>::from_hex(e.0).unwrap().unchecked_into(),
			e.1.to_string().parse::<Balance>().unwrap(),
		);
	}).collect();
	let vesting: Vec<(AccountId, BlockNumber, BlockNumber, Balance)> = vesting_json.into_iter().map(|e| {
		return (
			<[u8; 32]>::from_hex(e.0).unwrap().unchecked_into(),
			e.1.to_string().parse::<BlockNumber>().unwrap(),
			e.2.to_string().parse::<BlockNumber>().unwrap(),
			e.3.to_string().parse::<Balance>().unwrap()
		);
	}).collect();
	Ok((balances, vesting))
}

/// This is the allocation that will fit into the "balances" collection
/// of the "balances" module. The total should be 5e26 - 1e26 or 400 million EDG,
/// since we have also allocated 1e26 to the validators below.
pub fn get_genesis_allocation() -> Vec<(AccountId, Balance)> {
	return vec![(
		hex!["1a925251d02fa027882bb1a5834abee8ac4ed06fa735d531cb89ccc478ee5b50"].unchecked_into(),
		1100000000000000000000000,
	), (
		hex!["5c1aee377fcdced7d6a1d0a1b3f89acfbc0f3c2ed6c4f324434aaf3934a14b4b"].unchecked_into(),
		1100000000000000000000000,
	), (
		hex!["f67cfe9c4bb11ac43f7926e177af33bff39a095ff344fb2b96f95cf6b648b745"].unchecked_into(),
		1100000000000000000000000,
	), (
		hex!["ce56ed9d0c61d409610244e91df80be6d29d56f7e8f73a9f8bbb9143e6d3ab2b"].unchecked_into(),
		1100000000000000000000000,
	), (
		hex!["1280d5fd2fe138d51bdd11f8657ef13261f6623dcc4af20f30eea42b5111340c"].unchecked_into(),
		1100000000000000000000000,
	), (
		hex!["4cb0922d0fb5217553da0da70bd4076812ad2a5cce860524ff7b5e6e3629f962"].unchecked_into(),
		3000000000000000000000000,
	), (
		hex!["12d490251399a081935bf731184d2bf37d228bc38d3d68a8e3822933bcf23a09"].unchecked_into(),
		5500000000000000000000000,
	), (
		hex!["78040adec849fff1c66c16ab8ac1534ed27e37a8a1da8aa3239267a883369566"].unchecked_into(),
		1500000000000000000000000,
	), (
		hex!["c0a8a737e77f8c6cb62a2ffa8c0b9d8d7191d46d0e09c48c1354dab109ac4c5e"].unchecked_into(),
		2750000000000000000000000,
	), (
		hex!["f89077e892d9861da018a6cfa4082ccf39cceb83c431cec51b937befb2caa949"].unchecked_into(),
		2750000000000000000000000,
	), (
		hex!["1ec5e3d9a77ac81d6da0290c04d003bbcb04af8c4902bd59dbf9be4dfa47234f"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["720967cda4c9097924d705695b62dfb6dc6dbeade65b5575abf5c4ca38e50503"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["de90c8b070c0a63fbf52655af7492dc8e7d985334a4c60c02bc2f59424ff1430"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["9665bd715c72b686c2557fe11e6cd54924adef62dc1f52cf43a503f363cf843c"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["464c96a206e310511a27acc92b2e410a14bd83cb8788b522d0cee03f0d285862"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["34c71b1e42e910b94b8cbb2c960873bd4bf0db6e80afdf41cdc52acd91d6393f"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["d02002915139ac3e4552c5006f92cccfbf8b02cb4d4ca1993a69d63368cc1f0c"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["143f9f9019fa62919ed6da39b8f147cb52501438e1e9e7a82d22d7b49df18e59"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["4e7de9c8f3564fe5cc5057de51c41b40a7da801d22c6ee5aa57f8bb2838ae857"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["ce64070e4dffe61183241dca3e922b65ecd509430a3e283fab5c143532f79d3e"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["c4fff312e8e224c9f1380d235c1a601d9f00213e6b4f4c86250f768563cb6f2a"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["3a9fcaef453185230c2d777fa7dda16a745f6840b0cbc28f3e8c2ab07e533d3b"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["e280d0fbfbcbdc070526d0997d0bfec3b3527b036bf5a68f87e6cc0422ced302"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["865d38926f2344541912b464e2bb910ef72f2c9d447bc26f996b46f67605b85b"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["6ea38f9aeb405131cfcd8e542d72cd2a3f3e56427b9162298d8e7f3529a6c34e"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["968a20a9c04a662831afd93d8ad834f69c6aeb37e9017b4b739fd94502aae543"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["fce2e25267c2f45932ca1cc9834d2db0b5438804af69a8af16bade1ca00c160d"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["86cdb7761daa3c8917719b057e83ca0377de8ca599624c4ff7bd541ab056427d"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["dc1ec6087624728ec118c4b4cd8e58bfe8c977195aac172473262242fba5d160"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["a0c53fe2dd2bdcb3b3da9612b5a185243057ae81a7e6092a0788672cc99f8b72"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["3401ae0dc64bf4bae104413d1e68380cd8f9f75b753b870b39352bc25e05c865"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["62f118b23e00a02f0e64b2cc5491226c24b7781fd60ca7ac330011173f6d4b68"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["0211cc0925ac6dd2d14fbdb63766d235d2a3c159d4c1dd3a260f3f179b05ce41"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["d6a6eb11c5bb7b37de7ebefdcb723016bf525f5af016aff323ed8b46e182e063"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["2a36057030d2b7034eb0da5188729859780d151bdd3a64295ddcf5514e21a679"].unchecked_into(),
		4999980000000000000000000,
	), (
		hex!["92c3225d346f99794812069a16cb91d979137d1534fff5795dacbaede7369f1d"].unchecked_into(),
		10000000000000000000,
	), (
		hex!["02456d652d95f78a97ff18e9f1170b3478f3644c4283f58312f2bb98f3ffe74e"].unchecked_into(),
		10000000000000000000,
	), (
		hex!["5283ca562da42f106978e4773e41c9f2585599f0f63b60c647ef35a5e929c356"].unchecked_into(),
		92500000000000000000000000,
	), (
		hex!["688421b084a363cb394c5e3a7c79f44482bf2f15f6d86ea37ae110a3af238f07"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["765169492c492ee29f2af9af46f9e1b117aa0283b73a4361ae12ace1c41a6c72"].unchecked_into(),
		150000000000000000000000000,
	), (
		hex!["6490626e3bde470c449e90b83df92ddb8514f02067a0ddd66f1080b5033dec2d"].unchecked_into(),
		1475000000000000000002363,
	), (
		hex!["ec80b8b78a2b283f0a48712c8446241cf5f36d2f480559cdc73253981963f402"].unchecked_into(),
		25000000000000000000000,
	),

	// moose key
	(
		hex!["063b8cece2f86e568e7e18017988ef4b970dda9275f6f90c2fc6a9388cefb44d"].unchecked_into(),
		31250000000000000000000000,
	),


	// edgeware genesis validators unstaked
	(
		hex!["e049bb3d3369ddf42e07a51e8eeadb660a618fc3cea63dc728a2b79683873855"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["b204d6aa8c13f1f274f5f849cb09ef4f3f2641acc3dec99f905e444784fbb247"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["84f5ab01391781936ac0447d2e801fa6531cbe478ae96c5583fef9e101001123"].unchecked_into(),
		10000000000000000000000000,
	), ( 
		hex!["6e70fa41746fd3e728e2abe2bed9c957c9ef1a5e67304ff0a0841551bcd4d621"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["aea65488616f3fa5dc0bb7b4a3a803185bf05a8e89c076f9abc18021694ebb38"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["743ea391454673ddacfd7b344790b7889ab8702cd06208347fb3c24ab9dcbe14"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["84798309f2d264c5bb5f3c651ddf5080c3494013b201859e62bd9352b32c1148"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["3e0dbf4f6e126100f2cc8c6b4c62fffa263387edd2894e7ab7fa3a7f876a9437"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["f2a644bf322abbe9d5fc0861056ad5220cdf64b15a3218a116d45f91c709bd79"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["ca5b161dbda0460526d45b8bca2265b3f17d1d26463219f7c05fa4e8f1bf6f4d"].unchecked_into(),
		10000000000000000000000000,
	)]
}

/// The mainnet commonwealth validator pubkeys and staked balances.
/// We give each of these 9999990000000000000000000 balance, which
/// is 1e19 less than 1e25, so that we reserve 1e19 for the controllers
/// of these accounts (enough balance for txs and existential balance)
/// 
/// The total of the Commonwealth Mainnet validators balances is thus 1e26 or 20%
/// i.e 10 * ((1e25 - 1e19) + 1e19) = 1e26 or 20% of 5e26
pub const CONTROLLER_ENDOWMENT: Balance = 10000000000000000000;
pub fn get_genesis_mainnet_validators() -> Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)> {
	return vec![( // sunny
		// 5EHuW1nXmHARKrFTbA7xsk66qNWGB374H34D3TZew8U7SQ1Y
		hex!["6289da9f0a3d164f829d9b30dd43cc87c3513848dd0dec309d88f09a501b492b"].unchecked_into(),
		// 5HR5z8aPqwiXLNzNuUhpt9Uz1ri8jM8qBfBjKz11X2M2kHwC
		hex!["ecb8912c7fcaa6ff472d91d130309a9c25c7a0c975dfad513837dc85dcb34d71"].unchecked_into(),
		// 5CsWm3BeQ76kJ2sJLRkN5zTJsQnxLncQgpgTsQn7tZb3afXn
		hex!["23b32d8c70a57b0a7fe360f88bfd3b3262985d4054caee06ee3d381bc5caed1b"].unchecked_into(),
		// Initial bonded stake
		(31250000000000000000000000 - CONTROLLER_ENDOWMENT),
		// 5DBTeCLgaARpYzPL8jSCPrkuPgnvkYXtkPKfCaUnQWCPB6xY
		hex!["31631964511baf8181c9fcad6e4e3327c8ae499112259554305fbee10d3ba929"].unchecked_into(),
		// 5C6K36YHugikBYbstsyUjnkH7SeuzqxwJ5WBcfQCS9JfBeKb
		hex!["0139abce81b2d960169f38211f54223d4d84a7b14d7d3da618fc80ff4208f643"].unchecked_into(),
	), ( // chris
		// 5DczZg7DZkUhmzkZpK2wcpVYX4WLQx4Kpjmrx77ddpcSMYbV
		hex!["44dc53c284de7a6160d338f98f570a2d1dfdd0ebc3afaed6017459312b662e69"].unchecked_into(),
		// 5Dny3yFVmvsabJMJmfgtFRiGqTY6naW7c7bbMTR2UfJKYNRJ
		hex!["4c77b0de81e44a2d02051f158ca1144ada29ba6928eb57487d91dc38839e5d64"].unchecked_into(),
		// 5F7G9tiaWNZNSUHNzBwE9DUM55LazUczpcngQZ8QzsyUuk6S
		hex!["86a7e994fe96668459c406cda26ddf520988800328f5935bad595f2ff545fc85"].unchecked_into(),
		// Initial bonded stake
		(31250000000000000000000000 - CONTROLLER_ENDOWMENT),
		// 5D34dZW1PouSqRkwXoEEjHQak9cdVbmiNWSb7m6aK1bmd5qj
		hex!["2afbac9772ca45c072b6c3896190b4dc993e9f4a295f89185b7ad2a2c79ea29e"].unchecked_into(),
		// 5DuELEa4hUQ6VXjybPYyXV5c2Zq3D9w8Wy3YTQEfp5hRgS3d
		hex!["513e9a0ec094aeb0cd3b30d10c3b45d8a6274b27ce2e7252670a5b73de987a41"].unchecked_into(),
	), ( // julien
		// 5CF9eXSr7EquhukBkPQHKwsxXBBsLR51F3N1gqUn9YztHNyw
		hex!["07f745e7e0b3c3017be0fc1dddbce8d39f0dd794a1c50b44eb62466aff153e94"].unchecked_into(),
		// 5HL3HnjBUTJ9AuCipDf7YH98NhHRNeB5fWhF9Hx1XNk2AYcN
		hex!["e8df44198e715a215481af9d776d23c004f736594e8d91c6b1c9bb7b40fb1d32"].unchecked_into(),
		// 5EFEWwscSrFMBAydR2sSmjbMgUvwUxBk2Ja5VCpDMFqGMAVR
		hex!["608021b0ae3a25b0f4b8a71f5baf2a5f9f530c0de6c53946593318b2f883b507"].unchecked_into(),
		// Initial bonded stake
		(31250000000000000000000000 - CONTROLLER_ENDOWMENT),
		// 5EM1gfhsv8jNTGTuwQwbLezFqhLH66ycxx265auNsyJUvd9r
		hex!["64e868063b00975d98f665bcb1955b9bde66c72a7d1d63495f9c97fc5671ce50"].unchecked_into(),
		// 5EUnKuXiWXPmcwg3t21TV7Y3K8asgQ2sR8zPov2K4rUd13ot
		hex!["6ad5671610c16a8a6321776df44a87e562a16fc280a3a3faca5fc35817d7cd14"].unchecked_into(),
	)];
}

pub fn get_mainnet_identity_verifiers() -> Vec<AccountId> {
	return vec![
		// 5FP8pEw3pCxTap1GjJh6JVjoLLvnsVyvKtrxu9PSQThMDn1M
		hex!["92c3225d346f99794812069a16cb91d979137d1534fff5795dacbaede7369f1d"].unchecked_into(),
	];
}

pub fn get_mainnet_root_key() -> AccountId {
	// 5C7gaRoByJ99HoiZT9zgJAfx9p3YHLASkdT4Tn3ScgzpX6nX
	return hex!["02456d652d95f78a97ff18e9f1170b3478f3644c4283f58312f2bb98f3ffe74e"].unchecked_into();
}

pub fn get_mainnet_election_members() -> Vec<AccountId> {
	return vec![
		// 5C7gaRoByJ99HoiZT9zgJAfx9p3YHLASkdT4Tn3ScgzpX6nX
		hex!["02456d652d95f78a97ff18e9f1170b3478f3644c4283f58312f2bb98f3ffe74e"].unchecked_into(),
	];
}