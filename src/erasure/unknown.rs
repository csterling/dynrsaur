use std::alloc::{dealloc, Layout};
use std::panic::{UnwindSafe, RefUnwindSafe};
use std::ptr::read;
use crate::erasure::{Erase, Erasure};
use crate::for_all_combinations;

/// Trait implemented for all types, providing no information about them.
pub trait Unknown {}

impl<T: ?Sized> Unknown for T {}

macro_rules! impl_erase_traits_for_unknown {
    ($($traits:ident),*) => {
        impl<'lifetime, T: 'lifetime$( + $traits)*> Erasure<T> for Box<dyn Unknown$( + $traits)* + 'lifetime> {
            unsafe fn downcast_unchecked(self) -> T {
                let ptr = Box::into_raw(self);
        
                let value = read(ptr as *const T);
        
                dealloc(
                    ptr as *mut u8,
                    Layout::new::<T>()
                );
        
                value
            }
        }
        
        impl<'lifetime, T: 'lifetime$( + $traits)*> Erase<T> for Box<dyn Unknown$( + $traits)* + 'lifetime> {
            fn erase(value: T) -> Self {
                Box::new(value) as Box<dyn Unknown$( + $traits)*>
            }
        }
        
        impl<'borrow, T: 'borrow$( + $traits)*> Erasure<&'borrow T> for &'borrow (dyn Unknown$( + $traits)* + 'borrow) {
            unsafe fn downcast_unchecked(self) -> &'borrow T {
                &*(self as *const dyn Unknown as *const T)
            }
        }
        
        impl<'borrow, T: 'borrow$( + $traits)*> Erase<&'borrow T> for &'borrow (dyn Unknown$( + $traits)* + 'borrow) {
            fn erase(value: &'borrow T) -> Self {
                value as &(dyn Unknown$( + $traits)*)
            }
        }
        
        impl<'borrow, T: 'borrow$( + $traits)*> Erasure<&'borrow mut T> for &'borrow mut (dyn Unknown$( + $traits)* + 'borrow) {
            unsafe fn downcast_unchecked(self) -> &'borrow mut T {
                &mut *(self as *mut dyn Unknown as *mut T)
            }
        }
        
        impl<'borrow, T: 'borrow$( + $traits)*> Erase<&'borrow mut T> for &'borrow mut (dyn Unknown$( + $traits)* + 'borrow) {
            fn erase(value: &'borrow mut T) -> Self {
                value as &mut (dyn Unknown$( + $traits)*)
            }
        }
    };
}

for_all_combinations!(impl_erase_traits_for_unknown => Send, Sync, Unpin, UnwindSafe, RefUnwindSafe);

#[cfg(test)]
mod tests {
    use std::hint::black_box;
    use std::ops::Deref;
    use std::panic::{RefUnwindSafe, UnwindSafe};
    use crate::erasure::Erasure;
    use crate::erasure::unknown::Unknown;
    
    #[test]
    fn double_word_size() {
        const DOUBLE_WORD_SIZE: usize = size_of::<usize>() * 2;

        assert_eq!(size_of::<Box<dyn Unknown>>(), DOUBLE_WORD_SIZE);
    }

    #[test]
    fn readback() {
        const STRING: &str = "test string";

        let string = Box::new(String::from(STRING));

        let erased: Box<dyn Unknown> = string;

        let unerased: String = unsafe { erased.downcast_unchecked() };

        assert_eq!(unerased.deref(), STRING);
    }

    #[test]
    fn readback_ref() {
        const STRING: &str = "test string";

        let string = String::from(STRING);

        let erased: Box<dyn Unknown> = Box::new(string);

        let unerased: &String = unsafe { erased.deref().downcast_unchecked() };

        assert_eq!(unerased, STRING);
    }

    #[test]
    fn erased_drop() {
        let mut drop_called = false;

        struct SetTrueOnDrop<'a>(&'a mut bool);

        impl SetTrueOnDrop<'_> {
            pub fn value(&self) -> bool {
                *self.0
            }
        }

        impl Drop for SetTrueOnDrop<'_> {
            fn drop(&mut self) {
                *self.0 = true;
            }
        }

        {
            let unique = Box::new(SetTrueOnDrop(&mut drop_called));

            assert_eq!(unique.value(), false);

            let _erased = black_box(unique as Box<dyn Unknown>);
        }

        assert_eq!(drop_called, true)
    }
    
    #[test]
    fn without_auto_traits() {
        const TEST_STRING: &'static str = "Test String";
        
        let string = String::from(TEST_STRING);
        
        let erased: Box<dyn Unknown + Sync + Unpin + UnwindSafe + RefUnwindSafe> = Box::new(string);
        
        let downcast_ref = unsafe { <&(dyn Unknown + Sync + Unpin + UnwindSafe + RefUnwindSafe) as Erasure<&String>>::downcast_unchecked(erased.deref()) };
        
        assert_eq!(downcast_ref, TEST_STRING)
    }
}