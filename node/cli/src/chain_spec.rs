// Copyright 2018 Commonwealth Labs, Inc.
// This file is part of Straightedge.

// Straightedge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Straightedge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Straightedge.  If not, see <http://www.gnu.org/licenses/>

use straightedge_service as service;

/// The chain specification option.
#[derive(Clone, Debug, PartialEq)]
pub enum ChainSpec {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Whatever the current runtime is, with simple Alice/Bob auths.
	LocalTestnet,
	/// Straightedge mainnet configuration (intermediate build process)
	StraightedgeMainnetConfiguration,
	/// Straightedge Mainnet
	StraightedgeMainnet,
}

impl Default for ChainSpec {
	fn default() -> Self {
		ChainSpec::Development
	}
}

/// Get a chain config from a spec setting.
impl ChainSpec {
	pub(crate) fn load(self) -> Result<service::chain_spec::ChainSpec, String> {
		Ok(match self {
			ChainSpec::StraightedgeMainnet => service::chain_spec::straightedge_mainnet(),
			ChainSpec::StraightedgeMainnetConfiguration => service::chain_spec::straightedge_mainnet_config()?,
			ChainSpec::Development => service::chain_spec::development_config(),
			ChainSpec::LocalTestnet => service::chain_spec::local_testnet_config(),
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(ChainSpec::Development),
			"local" => Some(ChainSpec::LocalTestnet),
			"straightedge-mainnet-conf" => Some(ChainSpec::StraightedgeMainnetConfiguration),
			"straightedge" => Some(ChainSpec::StraightedgeMainnet),
			"" => Some(ChainSpec::default()),
			_ => None,
		}
	}
}