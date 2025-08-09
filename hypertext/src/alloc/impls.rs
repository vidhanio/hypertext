use core::fmt::{self, Write};

use super::alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    rc::Rc,
    string::String,
    sync::Arc,
    vec::Vec,
};
use crate::{AttributeBuffer, AttributeValue, Buffer, Context, Node, Raw, Renderable, Rendered};

impl<T: AsRef<str>, C: Context> Renderable<C> for Raw<T, C> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer<C>) {
        // XSS SAFETY: `Raw` values are expected to be pre-escaped for
        // the rendering context.
        buffer
            .dangerously_get_string()
            .push_str(self.inner.as_ref());
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(self.inner.as_ref().into())
    }
}

impl Renderable for fmt::Arguments<'_> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        struct ElementEscaper<'a>(&'a mut String);

        impl Write for ElementEscaper<'_> {
            #[inline]
            fn write_str(&mut self, s: &str) -> fmt::Result {
                html_escape::encode_text_to_string(s, self.0);
                Ok(())
            }
        }

        // XSS SAFETY: `ElementEscaper` will escape special characters.
        _ = ElementEscaper(buffer.dangerously_get_string()).write_fmt(*self);
    }
}

impl Renderable<AttributeValue> for fmt::Arguments<'_> {
    #[inline]
    fn render_to(&self, buffer: &mut AttributeBuffer) {
        struct AttributeEscaper<'a>(&'a mut String);

        impl Write for AttributeEscaper<'_> {
            #[inline]
            fn write_str(&mut self, s: &str) -> fmt::Result {
                html_escape::encode_double_quoted_attribute_to_string(s, self.0);
                Ok(())
            }
        }

        // XSS SAFETY: `AttributeEscaper` will escape special characters.
        _ = AttributeEscaper(buffer.dangerously_get_string()).write_fmt(*self);
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
    fn render(&self) -> Rendered<String> {
        Rendered(match *self {
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
}

impl Renderable for str {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        // XSS SAFETY: we use `html_escape` to ensure the text is properly escaped
        html_escape::encode_text_to_string(self, buffer.dangerously_get_string());
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(html_escape::encode_text(self).into_owned())
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
}

impl Renderable for String {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        self.as_str().render_to(buffer);
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        Renderable::<Node>::render(self.as_str())
    }
}

impl Renderable<AttributeValue> for String {
    #[inline]
    fn render_to(&self, buffer: &mut AttributeBuffer) {
        self.as_str().render_to(buffer);
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
    fn render(&self) -> Rendered<String> {
        Rendered(if *self { "true" } else { "false" }.into())
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
                fn render(&self) -> Rendered<String> {
                    Rendered(itoa::Buffer::new().format(*self).into())
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
                fn render(&self) -> Rendered<String> {
                    Rendered(ryu::Buffer::new().format(*self).into())
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
            impl<T: Renderable + ?Sized> Renderable for $Ty {
                #[inline]
                fn render_to(&self, buffer: &mut Buffer) {
                    T::render_to(&**self, buffer);
                }

                #[inline]
                fn render(&self) -> Rendered<String> {
                    T::render(&**self)
                }
            }

            impl<T: Renderable<AttributeValue> + ?Sized> Renderable<AttributeValue> for $Ty {
                #[inline]
                fn render_to(&self, buffer: &mut AttributeBuffer) {
                    T::render_to(&**self, buffer);
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

impl<'a, B: 'a + Renderable + ToOwned + ?Sized> Renderable for Cow<'a, B> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        B::render_to(&**self, buffer);
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        B::render(&**self)
    }
}

impl<'a, B: 'a + Renderable<AttributeValue> + ToOwned + ?Sized> Renderable<AttributeValue>
    for Cow<'a, B>
{
    #[inline]
    fn render_to(&self, buffer: &mut AttributeBuffer) {
        B::render_to(&**self, buffer);
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

impl<T: Renderable, const C: usize> Renderable for [T; C] {
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
        #[cfg_attr(docsrs, doc(fake_variadic))]
        #[cfg_attr(docsrs, doc = "This trait is implemented for tuples up to twelve items long.")]
        impl<$T: Renderable<C>, C: Context> Renderable<C> for ($T,) {
            #[inline]
            fn render_to(&self, buffer: &mut Buffer<C>) {
                self.$i.render_to(buffer);
            }
        }
    };
    (($i0:tt $T0:ident) $(($i:tt $T:ident))+) => {
        #[cfg_attr(docsrs, doc(hidden))]
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
