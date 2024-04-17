#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TypeInfo {
    pub type_id: isize,
    pub reference_depth: usize,
}

impl TypeInfo {
    pub fn new(type_id: isize, reference_depth: usize) -> TypeInfo {
        TypeInfo {
            type_id,
            reference_depth,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct LocalVariable {
    pub offset: isize,
    pub type_info: TypeInfo,
}

impl LocalVariable {
    pub fn new(offset: isize, type_id: isize, reference_depth: usize) -> LocalVariable {
        LocalVariable {
            offset,
            type_info: TypeInfo {
                type_id,
                reference_depth,
            },
        }
    }

    pub fn from_type_info(offset: isize, type_info: TypeInfo) -> LocalVariable {
        LocalVariable { offset, type_info }
    }
}
