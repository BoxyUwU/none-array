#![feature(const_in_array_repeat_expressions)]
#![allow(incomplete_features)]
#![feature(const_generics)]

pub const fn create_array<T, const SIZE: usize>() -> [Option<T>; SIZE] {
    [Option::<T>::None; SIZE]
}