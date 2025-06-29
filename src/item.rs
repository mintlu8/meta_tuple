use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

/// A [`MetaTuple`](crate::MetaTuple) containing a single item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct MetaItem<T>(pub T);

impl<T> AsRef<T> for MetaItem<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for MetaItem<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Borrow<T> for MetaItem<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> BorrowMut<T> for MetaItem<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Deref for MetaItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MetaItem<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: IntoIterator> IntoIterator for MetaItem<T> {
    type IntoIter = T::IntoIter;
    type Item = T::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'t, T> IntoIterator for &'t MetaItem<T>
where
    &'t T: IntoIterator,
{
    type IntoIter = <&'t T as IntoIterator>::IntoIter;
    type Item = <&'t T as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: 'static> MetaItem<T> {
    pub fn from_ref(value: &T) -> &Self {
        // Safety:
        //
        // Safe since `repr(transparent)`.
        // Same as the `ref_cast` crate.
        unsafe { (value as *const T as *const MetaItem<T>).as_ref() }.unwrap()
    }

    pub fn from_mut(value: &mut T) -> &mut Self {
        // Safety:
        //
        // Safe since `repr(transparent)`.
        // Same as the `ref_cast` crate.
        unsafe { (value as *mut T as *mut MetaItem<T>).as_mut() }.unwrap()
    }
}

impl<T: 'static> From<T> for MetaItem<T> {
    fn from(value: T) -> Self {
        MetaItem(value)
    }
}
