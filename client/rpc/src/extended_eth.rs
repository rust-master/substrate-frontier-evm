// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
// This file is part of Frontier.
//
// Copyright (c) 2020-2022 Parity Technologies (UK) Ltd.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::sync::Arc;

use ethereum_types::H256;
use jsonrpsee::core::RpcResult as Result;
use sc_network::{ExHashT, NetworkService};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

use fc_rpc_core::{types::PeerCount, ExtendedEthApiServer};
use fp_rpc::EthereumRuntimeRPCApi;

/// ExtendedEth API implementation.
pub struct ExtendedEthApi<B: BlockT, C, H: ExHashT> {
	client: Arc<C>,
	network: Arc<NetworkService<B, H>>,
	peer_count_as_hex: bool,
}

impl<B: BlockT, C, H: ExHashT> ExtendedEthApi<B, C, H> {
	pub fn new(
		client: Arc<C>,
		network: Arc<NetworkService<B, H>>,
		peer_count_as_hex: bool,
	) -> Self {
		Self {
			client,
			network,
			peer_count_as_hex,
		}
	}
}

impl<B, C, H: ExHashT> ExtendedEthApiServer for ExtendedEthApi<B, C, H>
where
	B: BlockT<Hash = H256> + Send + Sync + 'static,
	C: HeaderBackend<B> + ProvideRuntimeApi<B> + Send + Sync + 'static,
	C::Api: EthereumRuntimeRPCApi<B>,
{
	//return supported compilers
	fn compilers(&self) -> Result<Vec<String>> {
		Ok(vec![])
	}

	
}
