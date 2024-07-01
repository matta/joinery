pub(crate) struct FontContext {}
pub(crate) struct LayoutContext<T> {
    mystery_type: std::marker::PhantomData<T>,
}
pub(crate) struct Layout;

pub(crate) mod layout {
    pub(crate) struct Alignment {}
    pub(crate) struct Cursor {}
}
pub(crate) mod style {
    pub(crate) struct FontFamily {}
    pub(crate) struct FontStack {}
}
pub(crate) mod context {
    pub(crate) struct RangedBuilder {}
}
pub(crate) mod fontique {
    pub(crate) struct Style {}
    pub(crate) struct Weight {}
}
