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

pub fn get_allocation() 
	-> Result<(
		Vec<(AccountId, Balance)>,
		Vec<(AccountId, BlockNumber, BlockNumber, Balance)>,
		Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)>
	)> {
    let mut file = File::open("node/service/src/genesis.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

	let json: Allocation = serde_json::from_str(&data)?;;
	let balances_json = json.balances;
	let vesting_json = json.vesting;
	let validators_json = json.validators;

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
	let validators: Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)> = validators_json.into_iter().map(|e| {
		return (
			<[u8; 32]>::from_hex(e.0).unwrap().unchecked_into(),
			<[u8; 32]>::from_hex(e.1).unwrap().unchecked_into(),
			<[u8; 32]>::from_hex(e.2.clone()).unwrap().unchecked_into(),
			e.3.to_string().parse::<Balance>().unwrap(),
			<[u8; 32]>::from_hex(e.2.clone()).unwrap().unchecked_into(),
			<[u8; 32]>::from_hex(e.2).unwrap().unchecked_into(),
		);
	}).collect();
	Ok((balances, vesting, validators))
}

pub fn get_genesis_validators() -> Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)> {
	return vec![( // sunny
		// 5EHuW1nXmHARKrFTbA7xsk66qNWGB374H34D3TZew8U7SQ1Y
		hex!["6289da9f0a3d164f829d9b30dd43cc87c3513848dd0dec309d88f09a501b492b"].unchecked_into(),
		// 5HR5z8aPqwiXLNzNuUhpt9Uz1ri8jM8qBfBjKz11X2M2kHwC
		hex!["ecb8912c7fcaa6ff472d91d130309a9c25c7a0c975dfad513837dc85dcb34d71"].unchecked_into(),
		// 5CsWm3BeQ76kJ2sJLRkN5zTJsQnxLncQgpgTsQn7tZb3afXn
		hex!["23b32d8c70a57b0a7fe360f88bfd3b3262985d4054caee06ee3d381bc5caed1b"].unchecked_into(),
		50000000000000000000000000,
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
		50000000000000000000000000,
		// 5D34dZW1PouSqRkwXoEEjHQak9cdVbmiNWSb7m6aK1bmd5qj
		hex!["2afbac9772ca45c072b6c3896190b4dc993e9f4a295f89185b7ad2a2c79ea29e"].unchecked_into(),
		// 5DuELEa4hUQ6VXjybPYyXV5c2Zq3D9w8Wy3YTQEfp5hRgS3d
		hex!["513e9a0ec094aeb0cd3b30d10c3b45d8a6274b27ce2e7252670a5b73de987a41"].unchecked_into(),
	), ( // julien 
		// 5DczZg7DZkUhmzkZpK2wcpVYX4WLQx4Kpjmrx77ddpcSMYbV
		hex!["44dc53c284de7a6160d338f98f570a2d1dfdd0ebc3afaed6017459312b662e69"].unchecked_into(),
		// 5Dny3yFVmvsabJMJmfgtFRiGqTY6naW7c7bbMTR2UfJKYNRJ
		hex!["4c77b0de81e44a2d02051f158ca1144ada29ba6928eb57487d91dc38839e5d64"].unchecked_into(),
		// 5F7G9tiaWNZNSUHNzBwE9DUM55LazUczpcngQZ8QzsyUuk6S
		hex!["86a7e994fe96668459c406cda26ddf520988800328f5935bad595f2ff545fc85"].unchecked_into(),
		50000000000000000000000000,
		// 5D34dZW1PouSqRkwXoEEjHQak9cdVbmiNWSb7m6aK1bmd5qj
		hex!["2afbac9772ca45c072b6c3896190b4dc993e9f4a295f89185b7ad2a2c79ea29e"].unchecked_into(),
		// 5DuELEa4hUQ6VXjybPYyXV5c2Zq3D9w8Wy3YTQEfp5hRgS3d
		hex!["513e9a0ec094aeb0cd3b30d10c3b45d8a6274b27ce2e7252670a5b73de987a41"].unchecked_into(),
	)];
}

pub fn get_more_endowed() -> Vec<AccountId> {
	return vec![
		// 5GbmRGiV3roZkekSVHigiRonQnauNg5SB3DEnjF1a3VaW2uY
		hex!["c8a18852ebde806e011b4df37861a7b9b3960eea3383d8c5938a2feb6585ca60"].unchecked_into(),
		// 5DaJBK1GzL8ppRvGFGZzCtvXGUzEuqed2dQCp9bQmSWBcovH
		hex!["42cde2ea2ebad078c18be12d567bde59fe019243ddf670c9b46b30cbe0210d44"].unchecked_into(),
		// 5CexFLkSDHFn6jWSaiQs6s3QURig5rPoTCWecdysVxWRD8Jo
		hex!["1a1ec9b100da17f4062f5b23d4c442d6845ac913ff7d6d1ef5357be688b4ef16"].unchecked_into(),
		// 5HNVCZustH928S8mWbQWPcNfEQ5zipccQTiCDbYDrdSDno4f
		hex!["eabcfa4431091ab5742e21164a2e24b0e0d6f3ab96018c002b0188c213272d47"].unchecked_into(),
		// 5CrLFRMME6MbaFXrTUuHZ6VcS3RFHNwtqiP52f8BnEQTARVB
		hex!["22CC8CEE58420B7FF445DC9D6AFAEFC33B658A5F3A26322BA8DAB2D3FB6D2F1F"].unchecked_into(),
	];
}

pub fn get_identity_verifiers() -> Vec<AccountId> {
	return vec![
		// 5DXp7vdd8uS5HnodyNzXhE7oGrJWh7PpVa3DzwoAtRH66SMv
		hex!["40e8f152a7015fb3867e7c108514029942ef9004602d0f3a5f8061a54dfa6f35"].unchecked_into(),
	];
}

pub fn get_root_key() -> AccountId {
	// 5DXp7vdd8uS5HnodyNzXhE7oGrJWh7PpVa3DzwoAtRH66SMv
	return hex!["40e8f152a7015fb3867e7c108514029942ef9004602d0f3a5f8061a54dfa6f35"].unchecked_into();
}

