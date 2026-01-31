use syn::LitStr;

syn::custom_keyword!(data);

impl data {
    pub fn lit(self) -> LitStr {
        LitStr::new("data", self.span)
    }
}

syn::custom_keyword!(DOCTYPE);

impl DOCTYPE {
    pub fn lit(self) -> LitStr {
        LitStr::new("DOCTYPE", self.span)
    }
}

syn::custom_keyword!(html);

impl html {
    pub fn lit(self) -> LitStr {
        LitStr::new("html", self.span)
    }
}
