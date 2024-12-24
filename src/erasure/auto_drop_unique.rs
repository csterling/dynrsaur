use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use crate::erasure::erased::Erased;
use crate::erasure::Erasure;

// TODO: Add ?Sized capability once Pointee stabilised

/// An owning pointer to a value on the heap. If the [pointer](AutoDropUnique) is 
/// dropped, the underlying value will be dropped automatically, even if the pointer has been cast
/// to a different type than it's original type.
#[repr(transparent)]
pub struct AutoDropUnique<T = Erased>(NonNull<CPair<unsafe fn(NonNull<()>), ManuallyDrop<T>>>);

impl<T> AutoDropUnique<T> {
    pub fn new(value: T) -> Self {
        let boxed = Box::new(
            CPair(
                Self::drop_impl as unsafe fn(_),
                ManuallyDrop::new(value)
            )
        );

        let inner = NonNull::from(Box::leak(boxed));

        AutoDropUnique(inner)
    }
    
    /*TODO
    pub fn from_box<T>(value: Box<T>) -> Self {
        let value = NonNull::from(Box::leak(value)).cast::<MaybeUninit<T>>();
        
        let mut boxed = Box::new(CPair(Self::drop_impl::<T>, MaybeUninit::<T>::uninit()));

        unsafe { std::ptr::swap(value.as_ptr(), &mut boxed.1) }
        
        let _ = unsafe { Box::from_raw(value.cast::<ManuallyDrop<MaybeUninit<T>>>().as_ptr()) };
        
        let ptr = NonNull::from(Box::leak(boxed));

        let inner = ptr.cast::<()>();

        AutoDropUnique(inner)

    }*/
    
    pub fn erase(self) -> AutoDropUnique {
        AutoDropUnique(self.0.cast())
    }

    /// SAFETY: Must only be called when dropping T
    unsafe fn drop_impl(inner: NonNull<()>) {
        let mut ptr = inner.cast::<CPair<unsafe fn(NonNull<()>), ManuallyDrop<T>>>();

        let value_mut = &mut ptr.as_mut().1;

        ManuallyDrop::drop(value_mut)
    }
}

impl<T> Deref for AutoDropUnique<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: TODO
        unsafe { self.0.as_ref() }.1.deref()
    }
}

impl<T> DerefMut for AutoDropUnique<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: TODO
        unsafe { self.0.as_mut() }.1.deref_mut()
    }
}

impl<T> Drop for AutoDropUnique<T> {
    fn drop(&mut self) {
        let inner = self.0;

        // SAFETY: ptr is unique
        let drop = unsafe { inner.as_ref().0 };

        // SAFETY: Called during drop
        unsafe { drop(inner.cast::<()>()) }
    }
}

impl<'borrow, T> Erasure<T> for AutoDropUnique {
    unsafe fn downcast(self) -> T {
        let inner = self.0;

        let ptr = inner.cast::<CPair<unsafe fn(NonNull<()>), T>>();
        
        let boxed = Box::from_raw(ptr.as_ptr());

        boxed.1
    }
}

impl<'borrow, T> Erasure<&'borrow mut T> for &'borrow mut AutoDropUnique {
    unsafe fn downcast(self) -> &'borrow mut T {
        let inner = self.0;

        let ptr = inner.as_ptr() as *mut CPair<unsafe fn(NonNull<()>), T>;

        &mut (&mut *ptr).1
    }
}

impl<'borrow, T> Erasure<&'borrow T> for &'borrow AutoDropUnique {
    unsafe fn downcast(self) -> &'borrow T {
        let inner = self.0;

        let ptr = inner.as_ptr() as *const CPair<unsafe fn(NonNull<()>), T>;

        &(&*ptr).1
    }
}

#[repr(C)]
struct CPair<A, B: ?Sized>(pub A, pub B);    
