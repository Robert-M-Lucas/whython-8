use crate::root::name_resolver::resolve_names::UserType;

pub trait Type {
    fn id(&self) -> isize;

    fn size(&self) -> usize;
}
