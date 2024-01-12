# `hypertext`

```rs
hypertext::maud! {
    "see" a href="https://docs.rs/hypertext" { "docs.rs" } "for documentation."
}
```

## Features

### Speed

The macros generate code that is as fast as writing HTML to a string by hand. The macro automatically combines what would be multiple `push_str` calls into one if there is no dynamic content between them.

The crate gives extreme importance to lazy rendering and minimizing allocation, so it will only render the HTML to a string when you finally call the render function at the end.

### Type-Checking

All macros are validated at compile time, so you can't ever misspell an element/attribute or use invalid attributes. All of this validation has absolutely no runtime cost however, and it is just used for developer experience.

## Multiple Syntaxes

The crate provides a macro for writing rsx-style code, and another macro for writing [maud](https://maud.lambda.xyz)-style code, and lets you decide whichever one you like more.
