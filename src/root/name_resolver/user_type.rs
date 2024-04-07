use crate::root::ast::literals::Literal;
use crate::root::compiler::local_variable::{LocalVariable, TypeInfo};
use crate::root::parser::line_info::LineInfo;
use crate::root::name_resolver::processor::ProcessorError;
use crate::root::name_resolver::type_builder::{Type, TypeTable};

pub struct UserType {
    name: String,
    id: isize,
    path: LineInfo,
    attributes: Vec<(String, TypeInfo)>,
    destructor: Option<isize>,
}

impl UserType {
    pub fn new(
        name: String,
        id: isize,
        path: LineInfo,
        attributes: Vec<(String, TypeInfo)>,
        destructor: Option<isize>,
    ) -> UserType {
        UserType {
            name,
            id,
            path,
            attributes,
            destructor,
        }
    }

    pub fn get_attribute_offset_and_type(
        &self,
        name: &str,
        type_table: &TypeTable,
    ) -> Result<Option<LocalVariable>, ProcessorError> {
        let mut offset = 0;
        for (attrib_name, attrib_type) in &self.attributes {
            if name == attrib_name {
                return Ok(Some(LocalVariable::from_type_info(offset, *attrib_type)));
            }
            offset += type_table.get_type_size(*attrib_type)? as isize;
        }
        Ok(None)
    }

    pub fn get_attribute_types(&self) -> Vec<TypeInfo> {
        let mut out = Vec::new();
        for (_, i) in &self.attributes {
            out.push(*i);
        }
        out
    }
}

impl Type for UserType {
    fn get_id(&self) -> isize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_size(
        &self,
        type_table: &TypeTable,
        mut path: Option<Vec<isize>>,
    ) -> Result<usize, ProcessorError> {
        if path.is_none() {
            // path = Some(vec![self.get_id()])
            // ? 
        } else {
            let mut failed_check = false;
            for id in &**path.as_ref().unwrap() {
                if *id == self.get_id() {
                    failed_check = true;
                    break;
                }
            }

            if failed_check {
                let mut debug_str = String::new();
                for id in &**path.as_ref().unwrap() {
                    debug_str += &type_table.get_type(*id).unwrap().get_name();
                    debug_str += "->";
                }

                debug_str += &self.get_name();

                return Err(ProcessorError::CircularType(
                    self.path.clone(),
                    self.name.clone(),
                    debug_str,
                ));
            }

            path.as_mut().unwrap().push(self.get_id());
        };

        let mut size = 0;

        for (_name, id) in &self.attributes {
            size += type_table.get_type_size(*id)?;
        }

        Ok(size)
    }

    fn instantiate(
        &self,
        _literal: Option<&Literal>,
        _local_address: isize,
    ) -> Result<Vec<String>, ProcessorError> {
        panic!()
    }

    fn get_user_type(&self) -> Option<&UserType> {
        Some(self)
    }

    fn try_set_destructor(
        &mut self,
        line_info: &LineInfo,
        func: isize,
    ) -> Result<(), ProcessorError> {
        if self.destructor.is_some() {
            return Err(ProcessorError::MultipleDestructors(line_info.clone()));
        }
        self.destructor = Some(func);
        Ok(())
    }

    fn get_destructor(&self) -> Option<isize> {
        self.destructor
    }
}
