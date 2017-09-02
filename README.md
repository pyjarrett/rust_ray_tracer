# A Rust Ray Tracer (Version 1)

This is just a really simple ray tracer that I'm writing for myself to learn
about the mathematics and ideas behind ray tracers.

# API Documentation

Follow the [Rust API Guidelines](https://github.com/brson/rust-api-guidelines).

## Sections

- Safety
- Examples
- Panics
- Return(s)
- Units and dimensional analysis
- Definitions

## Crate large scale documents

## My ideas

- Moving vs. copying
- Definitions
- Using types to prevent bad uses (e.g. irradiance vs radiance, normals vs
  vectors)

# Precision

## f32 vs f64 use

I have ended up binding myself to using of `f32` and other types such as u16 to
the `Dimension` type.

## Precision issues

# Lessons Learned

- Use templates instead of `f32`
- Use `copy` for cheap types, rather than pass by references.
- Start bench marking early.
- Should I have used ray bounds?  PBRT uses them, but I haven't had a use for
  them yet.
- Prefer to hide types, unless they're immediately needed elsewhere.
- I feel like I should have shared work between the Vector and Point types.
  (using custom derive or something?)
- Haven't gotten to make/use "tensors" yet.
- It is hard to learn a language which is evolving, and stay up to date, while
  writing software.
- Learn memory management conventions before working in a language (e.g. Rc, vs
  Box, etc.)

# References

- https://doc.rust-lang.org/
- https://facility9.com/2016/05/writing-documentation-in-rust/

