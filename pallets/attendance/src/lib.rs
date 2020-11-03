#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use frame_support::codec::{Encode, Decode};
use frame_support::traits::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct AttendanceStruct {	 
	title: Vec<u8>,
	date: Vec<u8>,
	location: Vec<u8>,
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as AttendanceModule {
		StructData get(fn struct_data): AttendanceStruct;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		AttendanceStored(AttendanceStruct, AccountId), 
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = 10_000]
 		fn store_attendance(origin, title: Vec<u8>, date: Vec<u8>, location: Vec<u8>) -> dispatch::DispatchResult {
 			let who = ensure_signed(origin)?;
 			let attendance_struct = AttendanceStruct{
 				title,
 				date,
				location,
			};
			StructData::put(attendance_struct.clone());
 			Self::deposit_event(RawEvent::AttendanceStored(attendance_struct, who));
 			Ok(())
 		}
	}
}