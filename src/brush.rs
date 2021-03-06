use std::mem;
use math::*;
use comptr::ComPtr;
use factory::Factory;

use winapi::um::d2d1::*;

pub trait Brush {
    unsafe fn get_ptr(&self) -> *mut ID2D1Brush;

    fn get_factory(&self) -> Factory {
        unsafe {
            let ptr = self.get_ptr();
            let mut factory = ComPtr::<ID2D1Factory>::new();
            (*ptr).GetFactory(factory.raw_addr());

            Factory::from_ptr(factory.query_interface().unwrap())
        }
    }

    fn to_generic(&self) -> GenericBrush {
        GenericBrush {
            ptr: unsafe { ComPtr::from_existing(self.get_ptr()) },
        }
    }

    fn set_opacity(&mut self, opacity: f32) {
        unsafe {
            (*self.get_ptr()).SetOpacity(opacity);
        }
    }

    fn set_transform(&mut self, transform: &Matrix3x2F) {
        unsafe {
            (*self.get_ptr()).SetTransform(&transform.0);
        }
    }

    fn get_opacity(&self) -> f32 {
        unsafe { (*self.get_ptr()).GetOpacity() }
    }

    fn get_transform(&self) -> Matrix3x2F {
        unsafe {
            let mut mat: Matrix3x2F = mem::uninitialized();
            (*self.get_ptr()).GetTransform(&mut mat.0);
            mat
        }
    }
}

brush_type! { pub struct GenericBrush(ID2D1Brush); }

brush_type! { pub struct SolidColor(ID2D1SolidColorBrush); }

impl SolidColor {
    pub fn set_color(&mut self, color: &ColorF) {
        unsafe { (*self.ptr.raw_value()).SetColor(&color.0) };
    }

    pub fn get_color(&self) -> ColorF {
        unsafe { ColorF((*self.ptr.raw_value()).GetColor()) }
    }
}
