#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_support::sp_runtime::DispatchError;
use frame_system::ensure_signed;
use log::info;
use sp_std::vec::Vec;

pub trait Trait: frame_system::Trait + orml_nft::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as NftModule {
		NftModuleCID get(fn ntf_token_cid): Vec<u8>;
		NftModuleClass get(fn ntf_token_class): T::ClassId;
	}
}

decl_event!(
	pub enum Event<T> where
		AccountId = <T as frame_system::Trait>::AccountId, 
		ClassId = <T as orml_nft::Trait>::ClassId,
		TokenId = <T as orml_nft::Trait>::TokenId
	{
		Create(AccountId, ClassId),
		Mint(AccountId, TokenId),
		Burn(AccountId, ClassId, TokenId),
		Transfer(AccountId, AccountId, ClassId, TokenId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 0]
		pub fn create(origin, metadata: Vec<u8>, data: <T as orml_nft::Trait>::ClassData) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			info!("create :: data = {:?}", data);
			<NftModuleCID>::put(metadata.clone());
			let result: Result<T::ClassId, DispatchError> = orml_nft::Module::<T>::create_class(&who, metadata.clone(), data);
			info!("create :: result = {:?}", result.unwrap());
			<NftModuleClass<T>>::put(result.unwrap());
			Self::deposit_event(RawEvent::Create(who, result.unwrap()));
			Ok(())
		}

		#[weight = 0]
		pub fn mint(origin, data: <T as orml_nft::Trait>::TokenData) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			info!("mint :: data = {:?}", data);
			let result: Result<T::TokenId, DispatchError> = orml_nft::Module::<T>::mint(&who, <NftModuleClass<T>>::get(), <NftModuleCID>::get(), data);
			info!("mint :: result = {:?}", result.unwrap());
			Self::deposit_event(RawEvent::Mint(who, result.unwrap()));
			Ok(())
		}

		#[weight = 0]
		pub fn transfer(origin, to: T::AccountId, token: (T::ClassId, T::TokenId)) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			if who == to {
					return Ok(());
			}
			info!("transfer :: token_class = {:?} :: token_id = {:?}", token.0, token.1);
			let result: Result<(), DispatchError> = orml_nft::Module::<T>::transfer(&who, &to, token);
			info!("transfer :: result = {:?}", result.unwrap());
			let token_class = token.0;
			let token_id = token.1;
			Self::deposit_event(RawEvent::Transfer(who, to, token_class, token_id));
			Ok(())
		}

		#[weight = 0]
		pub fn burn(origin, token: (T::ClassId, T::TokenId)) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			info!("burn :: token = {:?}", token);
			let result: Result<(), DispatchError> = orml_nft::Module::<T>::burn(&who, token);
			info!("burn :: result = {:?}", result.unwrap());
			let token_class = token.0;
			let token_id = token.1;
			Self::deposit_event(RawEvent::Burn(who, token_class, token_id));
			Ok(())
		}
	}
}