use winapi::*;

pub struct Point2F(pub D2D1_POINT_2F);
pub struct Vector2F(pub D2D_VECTOR_2F);
pub struct SizeF(pub D2D1_SIZE_F);
pub struct RectF(pub D2D1_RECT_F);
pub struct RoundedRect(pub D2D1_ROUNDED_RECT);
pub struct Ellipse(pub D2D1_ELLIPSE);
pub struct Matrix3x2F(pub D2D1_MATRIX_3X2_F);

impl RectF {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> RectF {
        RectF(D2D1_RECT_F {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
        })
    }
}
