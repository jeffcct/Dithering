use std::ops::{ Sub, Mul, Div, Add };

use palette::{ color_difference::{DeltaE, ImprovedDeltaE}, FromColor, Lab, LinSrgb, Srgb };


pub trait Similarity {
    type Output: PartialOrd;
    fn similar_to(self, other: Self) -> Self::Output;
}

impl Similarity for i16 {
    type Output = i16;

    fn similar_to(self, other: Self) -> Self {
        return (self - other).abs();
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGB {
    pub red: i16,
    pub green: i16,
    pub blue: i16,
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        return RGB {
            red: red as i16,
            green: green as i16,
            blue: blue as i16,
        }
    }
}

impl Similarity for RGB {
    type Output = f32;

    fn similar_to(self, other: Self) -> f32 {
        let self_rgb = Srgb::new(self.red as f32 / 255.0, self.green as f32 / 255.0, self.blue as f32 / 255.0);
        let other_rgb = Srgb::new(other.red as f32 / 255.0, other.green as f32 / 255.0, other.blue as f32 / 255.0);

        let self_lab: Lab = Lab::from_color(self_rgb.into_linear());
        let other_lab: Lab = Lab::from_color(other_rgb.into_linear());

        return self_lab.improved_delta_e(other_lab);
    }
}

impl Sub for RGB {
    type Output = RGB;

    fn sub(self, other: Self) -> RGB {
        return RGB {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul<i16> for RGB {
    type Output = RGB;

    fn mul(self, factor: i16) -> RGB {
        return RGB {
            red: self.red * factor,
            green: self.green * factor,
            blue: self.blue * factor,
        }
    }

}

impl Div<i16> for RGB {
    type Output = RGB;

    fn div(self, divisor: i16) -> RGB {
        return RGB {
            red: self.red / divisor,
            green: self.green / divisor,
            blue: self.blue / divisor
        }
    }
}

impl Add for RGB {
    type Output = RGB;

    fn add(self, other: Self) -> Self {
        return RGB {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MyLab {
    pub lab: Lab,
}

impl MyLab {
    pub fn new(red: u8, green: u8, blue: u8) -> MyLab {
        let rgb = Srgb::new(red as f32 / 255.0, green as f32 / 255.0, blue as f32 / 255.0); // Red color
    
        // Convert to CIELAB
        let lab: Lab = Lab::from_color(rgb.into_linear());
        return MyLab { lab: lab };
    }

    pub fn to_rgb(self) -> RGB {
        let rgb_converted: Srgb = Srgb::from_linear(LinSrgb::from_color(self.lab));
        return RGB {
            red: (rgb_converted.red * 255.0) as i16,
            green: (rgb_converted.green * 255.0) as i16,
            blue: (rgb_converted.blue * 255.0) as i16,
        }
    }
}

impl Add for MyLab {
    type Output = MyLab;

    fn add(self, other: Self) -> Self::Output {
        MyLab { lab: Lab::new(self.lab.l + other.lab.l, self.lab.a + other.lab.a, self.lab.b + other.lab.b) }
    }
}

// Implement subtraction for Lab
impl Sub for MyLab {
    type Output = MyLab;

    fn sub(self, other: Self) -> Self::Output {
        MyLab { lab: Lab::new(self.lab.l - other.lab.l, self.lab.a - other.lab.a, self.lab.b - other.lab.b) }
    }
}

// Implement scalar multiplication for Lab
impl Mul<f32> for MyLab {
    type Output = MyLab;

    fn mul(self, factor: f32) -> Self::Output {
        MyLab { lab: Lab::new(self.lab.l * factor, self.lab.a * factor, self.lab.b * factor) }
    }
}

// Implement scalar division for Lab
impl Div<f32> for MyLab {
    type Output = MyLab;

    fn div(self, divisor: f32) -> Self::Output {
        MyLab { lab: Lab::new(self.lab.l / divisor, self.lab.a / divisor, self.lab.b / divisor) }
    }
}

impl Similarity for MyLab {
    type Output = f32;

    fn similar_to(self, other: Self) -> Self::Output {
        return self.lab.improved_delta_e(other.lab)
    }
}