#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, StorageMap};
use frame_system::ensure_signed;
pub use sp_std::vec::Vec;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// Pallets use events to inform users when important changes are made.
// Event documentation should end with an array that provides descriptive names for parameters.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event! {
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
			ClaimCreated(AccountId, Vec<u8>),
			ClaimRevoked(AccountId, Vec<u8>),
			ClaimTransferred(AccountId, Vec<u8>),

	}
}

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
			ProofClaimedExist,
			NoSuchProof,
			NotProofOwner,
	}
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Config> as TemplateModule {
			Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
			// Errors must be initialized if they are used by the pallet.
			type Error = Error<T>;

			// Events must be initialized if they are used by the pallet.
			fn deposit_event() = default;

			/// 创建存证
			#[weight = 10_000]
			fn create_claim(origin, proof: Vec<u8>) {
					let sender = ensure_signed(origin)?;

					ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofClaimedExist);

					let current_block = <frame_system::Module<T>>::block_number();

					Proofs::<T>::insert(&proof, (&sender, current_block));

					Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
			}

			/// 撤销存证
			#[weight = 10_000]
			fn revoke_claim(origin, proof: Vec<u8>) {
					let sender = ensure_signed(origin)?;

					ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

					let (owner, _) = Proofs::<T>::get(&proof);

					ensure!(sender == owner, Error::<T>::NotProofOwner);

					Proofs::<T>::remove(&proof);

					Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
			}

			/// 转移存证
			#[weight = 10_000]
			fn transfer_claim(to, proof: Vec<u8>) {
					let receiver = ensure_signed(to)?;

					ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

					let (_owner, block_number) = Proofs::<T>::get(&proof);

					Proofs::<T>::insert(&proof, (&receiver, &block_number));

					Self::deposit_event(RawEvent::ClaimTransferred(receiver, proof));
			}
	}
}
