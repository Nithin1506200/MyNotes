use std::fmt;

use zeroize::{Zeroize, ZeroizeOnDrop};
/*
Mask the data
*/

pub trait MaskingStrategy<T> {
    fn mask(data: T) -> String;
}

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Secret<T: Zeroize + ZeroizeOnDrop + MaskingStrategy<T>>(T);

impl<T: Zeroize + ZeroizeOnDrop + MaskingStrategy<T>> fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f.debug_struct("Secret").field("data", &self.data).finish()
        todo!()
    }
}

impl<T: Zeroize + ZeroizeOnDrop + MaskingStrategy<T>> Secret<T> {
    // pub fn new(data: T) -> Self {}
    pub fn peak(&self) -> &T {
        &self.0
    }
    pub fn peak_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
