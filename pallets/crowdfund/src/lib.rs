#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod tipos;

use frame_support::traits::{Currency, Get};
use tipos::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {

use super::*;
	use frame_support::{pallet_prelude::*, sp_runtime::Saturating, Blake2_128Concat};
	use frame_system::{pallet_prelude::*};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type LargoMinimoNombreProyecto: Get<u32>;

		#[pallet::constant]
		type LargoMaximoNombreProyecto: Get<u32>;

		type Currency: Currency<Self::AccountId>; // Pueden no utilizarlo.

	}

	#[pallet::storage]
	#[pallet::getter(fn proyectos)]
	pub type Proyectos<T> =
		StorageMap<_, Blake2_128Concat, BoundedString<T>, BalanceDe<T>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProyectoCreado { quien: T::AccountId, proyecto_tanjiro: NombreProyecto<T> },
		ProyectoApoyado { proyecto_tanjiro: NombreProyecto<T>, cantidad: BalanceDe<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		NombreMuyLargo,
		NombreMuyCorto,
		/// El usuario quiso apoyar un proyecto con más fondos de los que dispone.
		FondosInsuficientes,
		/// El usuario quiso apoyar un proyecto inexistente.
		ProyectoNoExiste,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Crea un proyecto.
		pub fn crear_proyecto(origen: OriginFor<T>, nombre: String) -> DispatchResult {
			// Completar este método.
			
			let nombre_decoded: String = nombre.clone().try_into().unwrap();

			print!("lingitud {}", nombre_decoded.len());

			if nombre_decoded.len() <= 2 {
				ensure!(false, Error::<T>::NombreMuyCorto);
			} else if nombre_decoded.len() >= 33 {
				ensure!(false, Error::<T>::NombreMuyLargo);
			}

			let quien = ensure_signed(origen)?;
			let nombre_acotado: NombreProyecto<T> = nombre.clone().try_into().unwrap();
			let proyecto_tanjiro: BoundedString<T> = nombre_acotado;

			let balance_tanjiro: BalanceDe<T> = T::Currency::total_balance(&quien) - T::Currency::total_balance(&quien); //free_balance(&quien);//.saturating_sub(T::Currency::minimum_balance());

			Proyectos::<T>::insert(&proyecto_tanjiro, balance_tanjiro);
			
			Self::deposit_event(Event::ProyectoCreado { quien, proyecto_tanjiro });

			Ok(())
		}

		pub fn apoyar_proyecto(
			origen: OriginFor<T>,
			nombre: String,
			cantidad: BalanceDe<T>,
		) -> DispatchResult {
			// Completar este método.

			let nombre_acotado: NombreProyecto<T> = nombre.clone().try_into().unwrap();
			let proyecto_tanjiro: BoundedString<T> = nombre_acotado;
/* 
			let balance_actual = Proyectos::<T>::get(&proyecto_tanjiro);

			let balance_tanjiro = balance_actual + cantidad;
			Proyectos::<T>::insert(&proyecto_tanjiro, balance_tanjiro);
*/
			
			let balance_actual = match <Proyectos<T>>::try_get(&proyecto_tanjiro) {
				Ok(mi_balance) => mi_balance,
				Err(e) => ensure!(true, Error::<T>::ProyectoNoExiste),
			};

			let balance_tanjiro = balance_actual + cantidad;
			Proyectos::<T>::insert(&proyecto_tanjiro, balance_tanjiro);

			Self::deposit_event(Event::ProyectoApoyado { proyecto_tanjiro, cantidad});

			Ok(())

		}

	}

}
