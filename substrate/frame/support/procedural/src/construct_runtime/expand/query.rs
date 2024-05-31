// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License

use crate::construct_runtime::Pallet;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

/// Expands implementation of runtime level `DispatchQuery`.
pub fn expand_outer_query(
	_runtime_name: &Ident,
	pallet_decls: &[Pallet],
	scrate: &TokenStream2,
) -> TokenStream2 {
	let runtime_query = syn::Ident::new("RuntimeQuery", Span::call_site());

	let query_id_prefix_impls = pallet_decls.iter().map(|pallet| {
		let pallet_path = &pallet.path;
		let pallet_name = &pallet.name;

		let mut output = [0u8; 16];
		let pallet_signature = format!(
			"{path}::{pallet}",
			path = quote::quote!(#pallet_path).to_string(),
			pallet = quote::quote!(#pallet_name).to_string()
		);
		let hash = sp_crypto_hashing::blake2_128(pallet_signature.as_bytes());
		output.copy_from_slice(&hash);
		let query_id_prefix_bytes =
			output.map(|byte| syn::LitInt::new(&format!("0x{:X}_u8", byte), Span::call_site()));

		quote::quote! {
			impl #scrate::traits::QueryIdPrefix for #scrate::traits::PalletQueryId<#pallet_name> {
				const PREFIX: [::core::primitive::u8; 16usize] = [ #( #query_id_prefix_bytes ),* ];
			}
		}
	});

	let query_match_arms = pallet_decls.iter().map(|pallet| {
		let pallet_name = &pallet.name;
		quote::quote! {
			< #scrate::traits::PalletQueryId<#pallet_name> as #scrate::traits::QueryIdPrefix>::PREFIX => {
				< #pallet_name as #scrate::traits::DispatchQuery>::dispatch_query(id, input, output)
			}
		}
	});

	quote::quote! {
		/// Runtime query type.
		#[derive(
			Clone, PartialEq, Eq,
			#scrate::__private::codec::Encode,
			#scrate::__private::codec::Decode,
			#scrate::__private::scale_info::TypeInfo,
			#scrate::__private::RuntimeDebug,
		)]
		pub enum #runtime_query {}

		const _: () = {
			#( #query_id_prefix_impls )*

			impl #scrate::traits::DispatchQuery for #runtime_query {
				#[deny(unreachable_patterns)] // todo: [AJ] should error if identical prefixes
				fn dispatch_query<O: #scrate::__private::codec::Output>(
					id: & #scrate::traits::QueryId,
					input: &mut &[u8],
					output: &mut O
				) -> Result<(), #scrate::__private::codec::Error>
				{
					// let y = 1; // todo: [AJ] why is unused variable error not triggered here - unused functions?
					match id.suffix {
						#( #query_match_arms )*
						_ => Err(#scrate::__private::codec::Error::from("DispatchQuery not implemented")), // todo: [AJ]
					}
				}
			}
		};
	}
}