pub use derive::TypeInfo;

use std::{fmt, rc::Rc};

pub trait TypeInfo: Clone + 'static {
    const DYNAMIC_TYPE: DynamicType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DynamicType {
    pub crate_name: &'static str,
    pub crate_version: &'static str,
    pub module: &'static str,
    pub type_name: &'static str,
    pub generics: &'static [DynamicType],
}

impl DynamicType {
    #[inline]
    pub fn of<T: TypeInfo>() -> Self {
        T::DYNAMIC_TYPE
    }
}

pub struct Dynamic {
    pub r#type: DynamicType,
    value: *mut (),
    clone: Rc<dyn Fn(*mut ()) -> *mut ()>,
    drop: Option<Rc<dyn Fn(*mut ())>>,
}

impl Drop for Dynamic {
    fn drop(&mut self) {
        if let Some(drop) = &self.drop {
            drop(self.value);
        }
    }
}

impl Dynamic {
    pub fn new<T: TypeInfo>(value: T) -> Self {
        Dynamic {
            r#type: DynamicType::of::<T>(),
            value: Box::into_raw(Box::new(value)) as *mut (),
            clone: Rc::new(|value| {
                let value = unsafe { &*(value as *mut T) };
                let cloned = value.clone();

                Box::into_raw(Box::new(cloned)) as *mut ()
            }),
            drop: Some(Rc::new(|value| {
                let value = *unsafe { Box::from_raw(value as *mut T) };
                drop(value)
            })),
        }
    }

    pub fn try_into_cast<T: TypeInfo>(mut self) -> Option<T> {
        if self.r#type == DynamicType::of::<T>() {
            let value = *unsafe { Box::from_raw(self.value as *mut T) };
            self.drop = None; // prevent double free
            Some(value)
        } else {
            None
        }
    }

    pub fn into_cast<T: TypeInfo>(self) -> T {
        let type_info = self.r#type;

        self.try_into_cast().unwrap_or_else(|| {
            panic!(
                "Cannot cast from {:?} to {:?}",
                type_info,
                DynamicType::of::<T>()
            )
        })
    }

    pub fn try_cast<T: TypeInfo>(&self) -> Option<&T> {
        if self.r#type == DynamicType::of::<T>() {
            let value = unsafe { &*(self.value as *mut T) };
            Some(value)
        } else {
            None
        }
    }

    pub fn cast<T: TypeInfo>(&self) -> &T {
        let type_info = self.r#type;

        self.try_cast().unwrap_or_else(|| {
            panic!(
                "Cannot cast from {:?} to {:?}",
                type_info,
                DynamicType::of::<T>()
            )
        })
    }

    pub fn try_cast_mut<T: TypeInfo>(&mut self) -> Option<&mut T> {
        if self.r#type == DynamicType::of::<T>() {
            let value = unsafe { &mut *(self.value as *mut T) };
            Some(value)
        } else {
            None
        }
    }

    pub fn cast_mut<T: TypeInfo>(&mut self) -> &mut T {
        let type_info = self.r#type;

        self.try_cast_mut().unwrap_or_else(|| {
            panic!(
                "Cannot cast from {:?} to {:?}",
                type_info,
                DynamicType::of::<T>()
            )
        })
    }
}

impl Clone for Dynamic {
    fn clone(&self) -> Self {
        Dynamic {
            r#type: self.r#type,
            value: (self.clone)(self.value),
            clone: self.clone.clone(),
            drop: self.drop.clone(),
        }
    }
}

impl fmt::Debug for Dynamic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Dynamic {:?})", self.r#type)
    }
}

#[cfg(test)]
mod tests;
