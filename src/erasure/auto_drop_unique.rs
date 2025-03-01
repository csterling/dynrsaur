use std::any::{type_name};
use std::marker::PhantomData;
use std::mem::{forget, ManuallyDrop, transmute};
use std::ops::{Deref, DerefMut};
use std::ptr::{addr_of_mut, NonNull, read};
use crate::erasure::erased::Erased;
use crate::erasure::{Erase, Erasure};

// TODO: Add ?Sized capability once core::ptr::Pointee stabilised

/// An owning pointer to a value on the heap.
/// 
/// If the pointer is dropped, the underlying value will be dropped automatically, even if the
/// pointer has been [type-erased](AutoDropUnique::erase).
/// 
/// Generic Parameters:
/// - `T`: the type of the owned value. 
#[repr(transparent)]
pub struct AutoDropUnique<T = Erased> {
    /// Tagged pointer representation of the underlying data (see [StackOrHeap]).
    tagged_pointer: usize,
    /// Marker indicating that we own a `T`.
    _marker: PhantomData<T>
}

impl<T> AutoDropUnique<T> {
    /// Takes ownership of the given value.
    /// 
    /// This will allocate the value onto the heap, if is not a ZST.
    pub fn new(value: T) -> Self {
        let tagged_pointer = match size_of::<T>() {
            0 => {
                // Throw away the value without dropping, as we can trivially recreate it later.
                forget(value);
                
                StackOrHeap::Stack(Self::zst_drop_impl) 
            },
            _ => StackOrHeap::Heap(
                NonNull::from(
                    Box::leak(
                        Box::new(
                            CPair(
                                Self::non_zst_drop_impl as unsafe fn(NonNull<()>),
                                ManuallyDrop::new(value)
                            )
                        )
                    )
                )
            )
        };

        AutoDropUnique {
            tagged_pointer: tagged_pointer.into_usize(),
            _marker: PhantomData
        }
    }
    
    /// Erases the type of the underlying value.
    pub fn erase(self) -> AutoDropUnique {
        unsafe { transmute(self) }
    }

    /// Erases the type of a reference to the underlying value.
    pub fn erase_ref(&self) -> &AutoDropUnique {
        // SAFETY: Invariant that
        unsafe { &*(self as *const Self as *const AutoDropUnique) }
    }

    /// Erases the type of a mutable reference to the underlying value.
    pub fn erase_mut(&mut self) -> &mut AutoDropUnique {
        unsafe { &mut *(self as *mut Self as *mut AutoDropUnique) }
    }
    
    /// Returns ownership of the underlying value.
    ///
    /// Panics if this pointer has been type-erased (i.e. `T` is [`Erased`]).
    pub fn into_inner(self) -> T {
        self.assert_not_erased();
        
        match self.take_inner() {
            StackOrHeap::Stack(_) => Self::zst_instance(),
            StackOrHeap::Heap(inner) => {
                // SAFETY: created by Box::leak
                let boxed = unsafe { Box::from_raw(inner.as_ptr()) };

                let value = (*boxed).1;

                ManuallyDrop::into_inner(value)
            }
        }
    }
    
    /// Takes ownership of the inner representation.
    fn take_inner(self) -> StackOrHeap<T> {
        let location = self.location();
        
        forget(self);
        
        location
    }

    /// Implementation of drop which remembers the type of the underlying value.
    /// 
    /// Takes the inner value of the AutoDropUnique, cast to a `NonNull<()>`.
    /// 
    /// SAFETY: Must only be called when dropping the AutoDropUnique, and `inner` must be
    ///         the inner value of the AutoDropUnique [cast to unit](NonNull::cast).
    unsafe fn non_zst_drop_impl(inner: NonNull<()>) {
        assert_ne!(size_of::<T>(), 0);
        
        // Cast the inner pointer back to its original type
        let mut ptr = inner.cast::<AutoDropUniqueInner<T>>();

        // Extract the value from the inner pair and drop it
        let value_mut = &mut ptr.as_mut().1;
        ManuallyDrop::drop(value_mut);
        
        // Drop the heap memory
        drop(Box::from_raw(ptr.as_ptr()))
    }
    
    /// Drop implementation for ZSTs.
    fn zst_drop_impl() {
        // As ZSTs are not stored, generate a new instance of the ZST, and then drops it.
        let _zst: T = Self::zst_instance();
    }
    
    /// Helper function to generate a new instance of a ZST.
    /// 
    /// Generic Parameters:
    /// - `Z`: The ZST.
    fn zst_instance<Z>() -> Z {
        assert_eq!(size_of::<Z>(), 0);
        unsafe { read(NonNull::<Z>::dangling().as_ptr()) }
    }
    
    fn location(&self) -> StackOrHeap<T> {
        // SAFETY: Created with StackOrHeap::into_usize.
        unsafe {
            StackOrHeap::<T>::from_usize(self.tagged_pointer)
        }
    }
    
    /// Returns a pointer to the value.
    fn as_ptr(&self) -> *mut T {
        match self.location() {
            StackOrHeap::Stack(_) => NonNull::<T>::dangling().as_ptr(),
            StackOrHeap::Heap(inner) => {
                let inner_ptr = inner.as_ptr();
                // SAFETY: `inner_ptr` is from NonNull.
                let inner = unsafe { addr_of_mut!((*inner_ptr).1) };
                inner as *mut T
            }
        }
    }
    
    /// Panics if `T` is uninhabited (i.e. `T` is [`Erased`]).
    fn assert_not_erased(&self) {
        // Detect if T is uninhabited. `size_of::<T>()` is zero for uninhabited types (identical
        // to ZSTs), but `size_of::<Option<T>>()` is still zero for uninhabited types, but non-zero
        // for inhabited types, so that is the test we use here.
        if size_of::<Option<T>>() == 0 {
            panic!("type {} is uninhabited", type_name::<T>());
        }
    }
}

impl<T> Deref for AutoDropUnique<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `self.as_ptr` is always valid.
        unsafe { &*self.as_ptr() }
    }
}

impl<T> DerefMut for AutoDropUnique<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `self.as_ptr` is always valid.
        unsafe { &mut *self.as_ptr() }
    }
}

impl<T> Drop for AutoDropUnique<T> {
    fn drop(&mut self) {
        match self.location() {
            StackOrHeap::Stack(drop) => drop(),
            StackOrHeap::Heap(inner) => {
                // SAFETY: inner is unique during drop
                let drop = unsafe { inner.as_ref().0 };

                // SAFETY: Called during drop with inner cast to unit
                unsafe { drop(inner.cast::<()>()) }
            }
        };
    }
}

impl<T> Erasure<T> for AutoDropUnique {
    unsafe fn downcast_unchecked(self) -> T {
        transmute::<_, AutoDropUnique<T>>(self).into_inner()
    }
}

impl<T> Erase<T> for AutoDropUnique {
    fn erase(value: T) -> Self {
        AutoDropUnique::<T>::new(value).erase()
    }
}

impl<'borrow, T> Erasure<&'borrow mut T> for &'borrow mut AutoDropUnique {
    unsafe fn downcast_unchecked(self) -> &'borrow mut T {
        &mut *(self.as_ptr() as *mut T)
    }
}

impl<'borrow, T> Erasure<&'borrow T> for &'borrow AutoDropUnique {
    unsafe fn downcast_unchecked(self) -> &'borrow T {
        &*(self.as_ptr() as *const T)
    }
}

/// Inner data-structure that determines what information needs storing about a value, and where
/// it should be stored.
/// 
/// Converts to/from a word-sized "tagged-pointer" representation, where the LSB of the 
/// `fn`/`NonNull` pointer is used to tag from which variant it was created.
enum StackOrHeap<T> {
    /// `T` is zero-sized, so we don't need to store it at all. Retain a `drop` function which
    /// regenerates the value of `T` and then drops it.
    Stack(fn()),
    /// `T` is non-zero-sized, so we place it with its `drop` function on the heap.
    Heap(NonNull<AutoDropUniqueInner<T>>)
}

impl<T> StackOrHeap<T> {
    /// Mask used to extract the discriminant from the tagged-pointer representation.
    const DISCRIMINANT_MASK: usize = 1;

    /// Mask used to extract the original pointer from the tagged-pointer representation.
    const ADDRESS_MASK: usize = !1;
    
    /// Converts the value to tagged-pointer representation.
    pub fn into_usize(self) -> usize {
        match self {
            StackOrHeap::Stack(ptr) => ptr as usize,
            StackOrHeap::Heap(ptr) => ptr.as_ptr() as usize | 1
        }
    }
    
    /// Recovers the enum from tagged-pointer representation.
    /// 
    /// SAFETY: `address` must have been created with [StackOrHeap::into_usize].
    pub unsafe fn from_usize(address: usize) -> Self {
        let heap = (address & Self::DISCRIMINANT_MASK) == 1;
        let address = address & Self::ADDRESS_MASK;
        
        match heap {
            false => Self::Stack(transmute(address)),
            true => Self::Heap(NonNull::new_unchecked(address as *mut AutoDropUniqueInner<T>)),
        }
    }
}

/// A C-like pair of values.
#[repr(C)]
struct CPair<A, B: ?Sized>(pub A, pub B);

/// The heap data-structure held by [AutoDropUnique].
/// 
/// A C-like pair of:
/// 1. a [pointer to the `drop` implementation](AutoDropUnique::non_zst_drop_impl) for the value, and,
/// 2. the value itself.
type AutoDropUniqueInner<T> = CPair<
    unsafe fn (NonNull<()>),
    ManuallyDrop<T>
>;

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::hint::black_box;
    use std::ops::{Deref, DerefMut};
    use std::sync::atomic::{AtomicBool, Ordering};
    use crate::erasure::{AutoDropUnique, Erasure};
    use crate::erasure::auto_drop_unique::AutoDropUniqueInner;

    #[test]
    fn single_word_size() {
        const SINGLE_WORD_SIZE: usize = size_of::<usize>();

        assert_eq!(size_of::<AutoDropUnique>(), SINGLE_WORD_SIZE);
        assert_eq!(size_of::<AutoDropUnique<u8>>(), SINGLE_WORD_SIZE);
        assert_eq!(size_of::<AutoDropUnique<usize>>(), SINGLE_WORD_SIZE);
        assert_eq!(size_of::<AutoDropUnique<String>>(), SINGLE_WORD_SIZE);
        assert_eq!(size_of::<AutoDropUnique<&str>>(), SINGLE_WORD_SIZE);
    }

    #[test]
    fn single_word_align() {
        const SINGLE_WORD_ALIGN: usize = align_of::<usize>();

        assert_eq!(align_of::<AutoDropUnique>(), SINGLE_WORD_ALIGN);
        assert_eq!(align_of::<AutoDropUnique<u8>>(), SINGLE_WORD_ALIGN);
        assert_eq!(align_of::<AutoDropUnique<usize>>(), SINGLE_WORD_ALIGN);
        assert_eq!(align_of::<AutoDropUnique<String>>(), SINGLE_WORD_ALIGN);
        assert_eq!(align_of::<AutoDropUnique<&str>>(), SINGLE_WORD_ALIGN);
    }
    
    #[test]
    fn inner_size_and_align() {
        const WORD_SIZE: usize = size_of::<usize>();
        const WORD_ALIGN: usize = align_of::<usize>();

        const fn expected_auto_drop_inner_size<T>() -> usize {
            let t_size = size_of::<T>();
            let t_align = align_of::<T>();
            if t_align >= WORD_ALIGN {
                t_align + t_size
            } else {
                2 * WORD_SIZE
            }
        }

        const fn expected_auto_drop_inner_align<T>() -> usize {
            let t_align = align_of::<T>();
            if t_align > WORD_ALIGN {
                t_align
            } else {
                WORD_ALIGN
            }
        }

        assert_eq!(
            align_of::<AutoDropUniqueInner<String>>(),
            expected_auto_drop_inner_align::<String>(),
            "unexpected String inner align"
        );
        assert_eq!(
            align_of::<AutoDropUniqueInner<usize>>(),
            expected_auto_drop_inner_align::<usize>(),
            "unexpected usize inner align"
        );
        assert_eq!(
            align_of::<AutoDropUniqueInner<u8>>(),
            expected_auto_drop_inner_align::<u8>(),
            "unexpected u8 inner align"
        );
        assert_eq!(
            align_of::<AutoDropUniqueInner<u16>>(),
            expected_auto_drop_inner_align::<u16>(),
            "unexpected u16 inner align"
        );
        
        macro_rules! expect_size {
            ($typ:ty) => {
                let actual = size_of::<AutoDropUniqueInner<$typ>>();
                let expected = expected_auto_drop_inner_size::<$typ>();
                assert_eq!(
                    actual, expected,
                    "unexpected {} (size: {}, align: {}) inner size (expected {}, actual {})",
                    stringify!($typ), size_of::<$typ>(), align_of::<$typ>(), expected, actual
                )
            };
        }
        
        expect_size!(String);
        expect_size!(usize);
        expect_size!(u8);
        expect_size!(u16);
    }

    #[test]
    fn into_inner() {
        fn inner<T: Clone + Eq + Debug>(value: T) {
            let erased = AutoDropUnique::new(value.clone());

            let unerased = erased.into_inner();

            assert_eq!(unerased, value);
        }

        inner(String::from("this is a test"));
        inner(123usize);
        inner(42u8);
        inner(32u16);
    }

    #[test]
    fn readback() {
        fn inner<T: Clone + Eq + Debug>(value: T) {
            let erased = AutoDropUnique::new(value.clone()).erase();

            let unerased: T = unsafe { Erasure::downcast_unchecked(erased) };

            assert_eq!(unerased, value);
        }

        inner(String::from("this is a test"));
        inner(123usize);
        inner(42u8);
        inner(32u16);
    }

    #[test]
    fn readback_into_ref() {
        fn inner<T: Clone + Eq + Debug>(value: T) {
            let erased = AutoDropUnique::new(value.clone()).erase();

            let unerased: &T = unsafe { (&erased).downcast_unchecked() };

            assert_eq!(unerased, &value);
        }

        inner(String::from("this is a test"));
        inner(123usize);
        inner(42u8);
        inner(32u16);
    }

    #[test]
    fn readback_ref() {
        fn inner<T: Clone + Eq + Debug>(value: T) {
            let unique = AutoDropUnique::new(value.clone());

            let unerased: &T = unsafe { unique.erase_ref().downcast_unchecked() };

            assert_eq!(unerased, &value);
        }

        inner(String::from("this is a test"));
        inner(123usize);
        inner(42u8);
        inner(32u16);
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
            let unique = AutoDropUnique::new(SetTrueOnDrop(&mut drop_called));
            
            assert_eq!(unique.value(), false);
            
            let _erased = unique.erase();
        }
        
        assert_eq!(drop_called, true)
    }
    
    #[test]
    fn drop_zst() {
        struct DropZST;
        
        static DROPPED: AtomicBool = AtomicBool::new(false);
        
        impl Drop for DropZST {
            fn drop(&mut self) {
                DROPPED.store(true, Ordering::Release);
            }
        }

        {
            let unique = AutoDropUnique::new(DropZST);

            assert_eq!(DROPPED.load(Ordering::Acquire), false);

            let _erased = unique.erase();
        }

        assert_eq!(DROPPED.load(Ordering::Acquire), true);
    }
    
    #[test]
    #[should_panic]
    fn erased_into_inner_panics() {
        let string = AutoDropUnique::new(String::from("Test String"));
        
        let erased = string.erase();
        
        let _erased_inner = black_box(erased.into_inner());
        
        #[allow(unreachable_code)]
        { assert!(true, "Post `erased.into_inner()`"); }
    }

    #[test]
    fn erased_deref_doesnt_panic() {
        let string = AutoDropUnique::new(String::from("Test String"));

        let erased = string.erase();

        let _erased_deref = black_box(erased.deref());
    }

    #[test]
    fn erased_deref_mut_doesnt_panic() {
        let string = AutoDropUnique::new(String::from("Test String"));

        let mut erased = string.erase();

        let _erased_deref_mut = black_box(erased.deref_mut());
    }
}