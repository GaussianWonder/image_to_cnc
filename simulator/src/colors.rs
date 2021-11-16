use nannou::prelude::rgb::Srgb;
use nannou::prelude::rgb::Srgba;
use nannou::color::srgba;
use nannou::color::srgb;
use core::marker::PhantomData;

pub fn monochrome(shade: u8) -> Srgb<u8> {
  srgb(shade, shade, shade)
}

pub fn monochroma(shade: u8, alpha: u8) -> Srgba<u8> {
  srgba(shade, shade, shade, alpha)
}

pub const DARK_GRAY: Srgb<u8> = Srgb { red: 22, green: 22, blue: 22, standard: PhantomData };
// pub const DARK_GRAY_A: Srgba<u8> = Srgba { color: Srgb { red: 22, green: 22, blue: 22, standard: PhantomData }, alpha: 22 };