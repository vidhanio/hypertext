use core::fmt::{self, Write};

use crate::{
    AttributeBuffer, Buffer, Raw, Renderable,
    alloc::{
        borrow::{Cow, ToOwned},
        boxed::Box,
        rc::Rc,
        string::String,
        sync::Arc,
        vec::Vec,
    },
    context::{AttributeValue, Context, Node},
};

impl<T: AsRef<str>, C: Context> Renderable<C> for Raw<T, C> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer<C>) {
        // XSS SAFETY: `Raw` values are expected to be pre-escaped for
        // their respective rendering context.
        buffer.dangerously_get_string().push_str(self.as_str());
    }

    #[inline]
    fn to_buffer(&self) -> Buffer<C> {
        // XSS SAFETY: `Raw` values are expected to be pre-escaped for
        // their respective rendering context.
        Buffer::dangerously_from_string(self.as_str().into())
    }
}

impl<C: Context> Renderable<C> for fmt::Arguments<'_>
where
    str: Renderable<C>,
{
    #[inline]
    fn render_to(&self, buffer: &mut Buffer<C>) {
        struct Escaper<'a, C: Context>(&'a mut Buffer<C>);

        impl<C: Context> Write for Escaper<'_, C>
        where
            str: Renderable<C>,
        {
            #[inline]
            fn write_str(&mut self, s: &str) -> fmt::Result {
                s.render_to(self.0);
                Ok(())
            }
        }

        _ = Escaper(buffer).write_fmt(*self);
    }
}

impl Renderable for char {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        let s = buffer.dangerously_get_string();
        match *self {
            '&' => s.push_str("&amp;"),
            '<' => s.push_str("&lt;"),
            '>' => s.push_str("&gt;"),
            c => s.push(c),
        }
    }

    #[inline]
    fn to_buffer(&self) -> Buffer<Node> {
        // XSS SAFETY: we are manually performing escaping here
        Buffer::dangerously_from_string(match *self {
            '&' => "&amp;".into(),
            '<' => "&lt;".into(),
            '>' => "&gt;".into(),
            c => c.into(),
        })
    }
}

impl Renderable<AttributeValue> for char {
    #[inline]
    fn render_to(&self, buffer: &mut AttributeBuffer) {
        // XSS SAFETY: we are manually performing escaping here
        let s = buffer.dangerously_get_string();

        match *self {
            '&' => s.push_str("&amp;"),
            '<' => s.push_str("&lt;"),
            '>' => s.push_str("&gt;"),
            '"' => s.push_str("&quot;"),
            c => s.push(c),
        }
    }

    #[inline]
    fn to_buffer(&self) -> AttributeBuffer {
        // XSS SAFETY: we are manually performing escaping here
        AttributeBuffer::dangerously_from_string(match *self {
            '&' => "&amp;".into(),
            '<' => "&lt;".into(),
            '>' => "&gt;".into(),
            '"' => "&quot;".into(),
            c => c.into(),
        })
    }
}

impl Renderable for str {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        // XSS SAFETY: we use `html_escape` to ensure the text is properly escaped
        html_escape::encode_text_to_string(self, buffer.dangerously_get_string());
    }

    #[inline]
    fn to_buffer(&self) -> Buffer {
        // XSS SAFETY: we use `html_escape` to ensure the text is properly escaped
        Buffer::dangerously_from_string(html_escape::encode_text(self).into_owned())
    }
}

impl Renderable<AttributeValue> for str {
    #[inline]
    fn render_to(&self, buffer: &mut AttributeBuffer) {
        // XSS SAFETY: we use `html_escape` to ensure the text is properly escaped
        html_escape::encode_double_quoted_attribute_to_string(
            self,
            buffer.dangerously_get_string(),
        );
    }

    #[inline]
    fn to_buffer(&self) -> AttributeBuffer {
        // XSS SAFETY: we use `html_escape` to ensure the text is properly escaped
        AttributeBuffer::dangerously_from_string(
            html_escape::encode_double_quoted_attribute(self).into_owned(),
        )
    }
}

impl<C: Context> Renderable<C> for String
where
    str: Renderable<C>,
{
    #[inline]
    fn render_to(&self, buffer: &mut Buffer<C>) {
        self.as_str().render_to(buffer);
    }

    #[inline]
    fn to_buffer(&self) -> Buffer<C> {
        self.as_str().to_buffer()
    }
}

impl<C: Context> Renderable<C> for bool {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer<C>) {
        // XSS SAFETY: "true" and "false" are safe strings
        buffer
            .dangerously_get_string()
            .push_str(if *self { "true" } else { "false" });
    }

    #[inline]
    fn to_buffer(&self) -> Buffer<C> {
        // XSS SAFETY: "true" and "false" are safe strings
        Buffer::dangerously_from_string(if *self { "true" } else { "false" }.into())
    }
}

macro_rules! render_via_itoa {
    ($($Ty:ty)*) => {
        $(
            impl<C: Context> Renderable<C> for $Ty {
                #[inline]
                fn render_to(&self, buffer: &mut Buffer<C>) {
                    // XSS SAFETY: integers are safe
                    buffer.dangerously_get_string().push_str(itoa::Buffer::new().format(*self));
                }

                #[inline]
                fn to_buffer(&self) -> Buffer<C> {
                    // XSS SAFETY: integers are safe
                    Buffer::dangerously_from_string(itoa::Buffer::new().format(*self).into())
                }
            }
        )*
    };
}

render_via_itoa! {
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
}

macro_rules! render_via_ryu {
    ($($Ty:ty)*) => {
        $(
            impl<C: Context> Renderable<C> for $Ty {
                #[inline]
                fn render_to(&self, buffer: &mut Buffer<C>) {
                    // XSS SAFETY: floats are safe
                    buffer.dangerously_get_string().push_str(ryu::Buffer::new().format(*self));
                }

                #[inline]
                fn to_buffer(&self) -> Buffer<C> {
                    // XSS SAFETY: floats are safe
                    Buffer::dangerously_from_string(ryu::Buffer::new().format(*self).into())
                }
            }
        )*
    };
}

render_via_ryu! {
    f32 f64
}

macro_rules! render_via_deref {
    ($($Ty:ty)*) => {
        $(
            impl<T: Renderable<C> + ?Sized, C: Context> Renderable<C> for $Ty {
                #[inline]
                fn render_to(&self, buffer: &mut Buffer<C>) {
                    // T::render_to(&**self, buffer);
                    (**self).render_to(buffer);
                }

                #[inline]
                fn to_buffer(&self) -> Buffer<C> {
                    (**self).to_buffer()
                }
            }
        )*
    };
}

render_via_deref! {
    &T
    &mut T
    Box<T>
    Rc<T>
    Arc<T>
}

impl<'a, B: 'a + Renderable<C> + ToOwned + ?Sized, C: Context> Renderable<C> for Cow<'a, B> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer<C>) {
        (**self).render_to(buffer);
    }

    #[inline]
    fn to_buffer(&self) -> Buffer<C> {
        (**self).to_buffer()
    }
}

impl<T: Renderable> Renderable for [T] {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        for item in self {
            item.render_to(buffer);
        }
    }
}

impl<T: Renderable, const N: usize> Renderable for [T; N] {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        self.as_slice().render_to(buffer);
    }
}

impl<T: Renderable> Renderable for Vec<T> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        self.as_slice().render_to(buffer);
    }
}

impl<T: Renderable<C>, C: Context> Renderable<C> for Option<T> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer<C>) {
        if let Some(value) = self {
            value.render_to(buffer);
        }
    }
}

impl<T: Renderable<C>, E: Renderable<C>, C: Context> Renderable<C> for Result<T, E> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer<C>) {
        match self {
            Ok(value) => value.render_to(buffer),
            Err(err) => err.render_to(buffer),
        }
    }
}

macro_rules! impl_tuple {
    () => {
        impl<C: Context> Renderable<C> for () {
            #[inline]
            fn render_to(&self, _: &mut Buffer<C>) {}
        }
    };
    (($i:tt $T:ident)) => {
        #[cfg_attr(all(docsrs, not(doctest)), doc(fake_variadic))]
        #[cfg_attr(all(docsrs, not(doctest)), doc = "This trait is implemented for tuples up to twelve items long.")]
        impl<$T: Renderable<C>, C: Context> Renderable<C> for ($T,) {
            #[inline]
            fn render_to(&self, buffer: &mut Buffer<C>) {
                self.$i.render_to(buffer);
            }
        }
    };
    (($i0:tt $T0:ident) $(($i:tt $T:ident))+) => {
        #[cfg_attr(all(docsrs, not(doctest)), doc(hidden))]
        impl<$T0: Renderable<C>, $($T: Renderable<C>),*, C: Context> Renderable<C> for ($T0, $($T,)*) {
            #[inline]
            fn render_to(&self, buffer: &mut Buffer<C>) {
                self.$i0.render_to(buffer);
                $(self.$i.render_to(buffer);)*
            }
        }
    }
}

impl_tuple!();
impl_tuple!((0 T));
impl_tuple!((0 T0) (1 T1));
impl_tuple!((0 T0) (1 T1) (2 T2));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7) (8 T8));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7) (8 T8) (9 T9));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7) (8 T8) (9 T9) (10 T10));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7) (8 T8) (9 T9) (10 T10) (11 T11));
