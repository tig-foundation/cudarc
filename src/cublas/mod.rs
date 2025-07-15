//! Wrappers around the [cublas API](https://docs.nvidia.com/cuda/cublas/index.html),
//! in three levels. See crate documentation for description of each.

pub mod result;
pub mod safe;
#[allow(warnings)]
pub mod sys;

pub use safe::*;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::AtomicU64;
use std::sync::OnceLock;

pub struct CublasRegistry {
    //pub(crate) fuel_used: AtomicUsize,
    //pub(crate) runtime_signature: AtomicU64
}

//pub(crate) static REGISTRY: OnceLock<CublasRegistry> = OnceLock::new();

use crate::RTSigFuel;

impl CublasRegistry {
    /*pub fn get() -> &'static CublasRegistry {
        REGISTRY.get_or_init(|| CublasRegistry {
            fuel_used: AtomicUsize::new(0),
            runtime_signature: AtomicU64::new(0),
        })
    }*/

    pub(crate) fn add_fuel(amount: u64) {
        //REGISTRY.get().unwrap().fuel_used.fetch_add(amount, std::sync::atomic::Ordering::Relaxed);
        RTSigFuel::add_fuel(amount as u64);
    }

    /*pub fn get_total_fuel_used() -> usize {
        //REGISTRY.get().unwrap().fuel_used.load(std::sync::atomic::Ordering::Relaxed)
        RTSigFuel::get_total_fuel_used() as usize
    }*/

    pub(crate) fn mix_runtime_signature(signature: u64) {
        //REGISTRY.get().unwrap().runtime_signature.fetch_xor(signature, std::sync::atomic::Ordering::Relaxed);
        RTSigFuel::mix_runtime_signature(signature);
    }

    /*pub fn get_runtime_signature() -> u64 {
        //REGISTRY.get().unwrap().runtime_signature.load(std::sync::atomic::Ordering::Relaxed)
        RTSigFuel::get_runtime_signature()
    }*/
}