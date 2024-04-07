use crate::root::name_resolver::preprocess::{PreProcessFunction, PreprocessSymbol};
use crate::root::name_resolver::processor::ProcessorError;

use std::collections::HashMap;

use crate::root::ast::literals::Literal;
use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::compiler::local_variable::{LocalVariable, TypeInfo};
use crate::root::custom::types::bool::Bool;
use crate::root::custom::types::float::Float;
use crate::root::custom::types::int::Int;
use crate::root::parser::line_info::LineInfo;

use crate::root::name_resolver::user_type::UserType;
use crate::root::utils::align;

struct UninitialisedType {
    pub path: LineInfo,
    pub id: isize,
    pub attributes: Vec<(String, Result<TypeInfo, ((String, usize), LineInfo)>)>,
}

impl UninitialisedType {
    pub fn new(
        path: LineInfo,
        id: isize,
        attributes: Vec<(String, LineInfo, (String, usize), LineInfo)>,
    ) -> UninitialisedType {
        let mut attributes_processed = Vec::new();
        for (attr_name, _attr_line, attr_type, attr_type_line) in attributes {
            attributes_processed.push((attr_name, Err((attr_type, attr_type_line))));
        }

        UninitialisedType {
            path,
            id,
            attributes: attributes_processed,
        }
    }

    // pub fn to_initialised(self) -> Result<UserType, ProcessorError> {
    //     let mut attributes_processed = Vec::new();
    //     for (attr_name, attr_type) in self.attributes {
    //         if attr_type.is_err() {
    //             let (attr_type, attr_type_line) = attr_type.unwrap_err();
    //             return Err(TypeNotFoundError(self.path, attr_type_line, attr_type));
    //         }
    //
    //         attributes_processed.push((attr_name, attr_type.unwrap()))
    //     }
    //
    //     Ok(UserType::new(self.name, self.id, attributes_processed))
    // }
}

pub trait Type {
    fn get_id(&self) -> isize;

    fn get_name(&self) -> &str;

    fn get_indirect_name(&self, indirection: usize) -> String {
        let mut out = String::new();
        for _ in 0..indirection {
            out.push('$');
        }
        out += self.get_name();
        out
    }

    fn get_size(
        &self,
        type_table: &TypeTable,
        path: Option<Vec<isize>>,
    ) -> Result<usize, ProcessorError>;

    fn instantiate(
        &self,
        literal: Option<&Literal>,
        local_address: isize,
    ) -> Result<Vec<String>, ProcessorError>;

    fn get_user_type(&self) -> Option<&UserType> {
        None
    }

    fn try_set_destructor(
        &mut self,
        line_info: &LineInfo,
        _func: isize,
    ) -> Result<(), ProcessorError> {
        Err(ProcessorError::CantSetBuiltinDestructor(line_info.clone()))
    }

    fn get_destructor(&self) -> Option<isize> {
        None
    }
}

// pub struct TypeIdentifier {
//     id: isize,
//     indirections: usize,
// }

pub struct TypeTable {
    types: HashMap<isize, Box<dyn Type>>,
}

impl TypeTable {
    pub fn new() -> TypeTable {
        TypeTable {
            types: HashMap::new(),
        }
    }

    pub fn add_builtin(mut self) -> TypeTable {
        self.add_type(Int::get_id(), Box::new(Int::new()));
        self.add_type(Bool::get_id(), Box::new(Bool::new()));
        self.add_type(Float::get_id(), Box::new(Float::new()));
        self
    }

    pub fn add_type(&mut self, id: isize, type_: Box<dyn Type>) {
        if self.types.insert(id, type_).is_some() {
            panic!("Attempted to override type")
        }
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<isize> {
        for (id, type_) in &self.types {
            if type_.get_name() == name {
                return Some(*id);
            }
        }
        None
    }

    pub fn get_type(&self, id: isize) -> Option<&Box<dyn Type>> {
        self.types.get(&id)
    }

    pub fn get_type_mut(&mut self, id: isize) -> Option<&mut Box<dyn Type>> {
        self.types.get_mut(&id)
    }

    pub fn get_type_size(&self, id: TypeInfo) -> Result<usize, ProcessorError> {
        if id.reference_depth != 0 {
            return Ok(Int::get_size(&Int::new(), self, None)?);
        }
        self.types.get(&id.type_id).unwrap().get_size(self, None)
    }
}

#[derive(Debug)]
pub struct UserTypedFunction {
    pub id: isize,
    pub name: String,
    pub line: LineInfo,
    pub args: Vec<(String, TypeInfo)>,
    pub return_type: Option<TypeInfo>,
    pub contents: Option<Vec<(BasicSymbol, LineInfo)>>,
}

impl TypedFunction for UserTypedFunction {
    fn get_id(&self) -> isize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_args(&self) -> &[(String, TypeInfo)] {
        &self.args
    }

    fn get_line(&self) -> LineInfo {
        self.line.clone()
    }

    fn get_return_type(&self) -> Option<TypeInfo> {
        self.return_type
    }

    fn is_inline(&self) -> bool {
        false
    }

    fn contents(&self) -> &Vec<(BasicSymbol, LineInfo)> {
        self.contents.as_ref().unwrap()
    }

    fn take_contents(&mut self) -> Vec<(BasicSymbol, LineInfo)> {
        self.contents.take().unwrap()
    }

    fn get_inline(&self, _args: Vec<isize>) -> Vec<String> {
        panic!()
    }
}

pub trait TypedFunction {
    fn get_id(&self) -> isize {
        panic!();
    }
    fn get_name(&self) -> &str;
    fn get_args(&self) -> &[(String, TypeInfo)];
    fn get_line(&self) -> LineInfo;
    fn get_args_positioned(
        &self,
        type_table: &TypeTable,
    ) -> Result<Vec<(String, LocalVariable)>, ProcessorError> {
        let mut offset = 16isize;
        if let Some(return_type) = self.get_return_type() {
            offset += type_table.get_type_size(return_type)? as isize;
            offset = align(offset, 8);
        }
        let mut output = Vec::new();

        for (name, _type) in self.get_args() {
            output.push((name.clone(), LocalVariable::from_type_info(offset, *_type)));
            offset += type_table.get_type_size(*_type).unwrap() as isize;
            offset = align(offset, 8);
        }

        Ok(output)
    }
    fn get_return_type(&self) -> Option<TypeInfo>;
    fn is_inline(&self) -> bool;
    fn contents(&self) -> &Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }
    fn take_contents(&mut self) -> Vec<(BasicSymbol, LineInfo)> {
        panic!()
    }
    fn get_inline(&self, _args: Vec<isize>) -> Vec<String> {
        panic!();
    }
}

// #[derive(Debug)]
// pub enum TypedImplsFns {
//     Impl(isize, Vec<TypedFunction>),
//     Fn(TypedFunction)
// }

pub fn build_types(
    pre_ast: Vec<PreprocessSymbol>,
) -> Result<
    (
        TypeTable,
        HashMap<Option<isize>, HashMap<String, isize>>,
        HashMap<isize, Box<dyn TypedFunction>>,
    ),
    ProcessorError,
> {
    let mut remaining_pre_ast = Vec::new();

    let mut uninitialised_types: HashMap<String, UninitialisedType> = HashMap::new();
    let mut type_counter = 0isize;

    let mut type_table = TypeTable::new().add_builtin();

    for symbol in pre_ast {
        match symbol {
            PreprocessSymbol::Struct(line, name, args) => {
                uninitialised_types.insert(
                    name.clone(),
                    UninitialisedType::new(line, type_counter, args),
                );
                type_counter += 1;
            }
            other => remaining_pre_ast.push(other),
        }
    }

    let mut uninitialised_types: Vec<_> = uninitialised_types.into_iter().collect();

    for i in 0..uninitialised_types.len() {
        'attr_loop: for a in 0..uninitialised_types[i].1.attributes.len() {
            for j in 0..uninitialised_types.len() {
                if uninitialised_types[i].1.attributes[a]
                    .1
                    .as_ref()
                    .unwrap_err()
                    .0
                     .0
                    == uninitialised_types[j].0
                {
                    uninitialised_types[i].1.attributes[a].1 = Ok(TypeInfo::new(
                        uninitialised_types[j].1.id,
                        uninitialised_types[i].1.attributes[a]
                            .1
                            .as_ref()
                            .unwrap_err()
                            .0
                             .1,
                    ));
                    continue 'attr_loop;
                }
            }

            if let Some(id) = type_table.get_id_by_name(
                &uninitialised_types[i].1.attributes[a]
                    .1
                    .as_ref()
                    .unwrap_err()
                    .0
                     .0,
            ) {
                uninitialised_types[i].1.attributes[a].1 = Ok(TypeInfo::new(
                    id,
                    uninitialised_types[i].1.attributes[a]
                        .1
                        .as_ref()
                        .unwrap_err()
                        .0
                         .1,
                ));
                continue 'attr_loop;
            }

            let type_ = uninitialised_types.remove(i).1;
            let (_line, mut attributes) = (type_.path, type_.attributes);
            let (type_name, line) = attributes.remove(a).1.unwrap_err();
            return Err(ProcessorError::TypeNotFound(line, type_name.0));
        }
    }

    for (name, type_) in uninitialised_types {
        let (_id, line, attributes) = (type_.id, type_.path, type_.attributes);

        let mut attributes_processed = Vec::new();
        for (attr_name, attr_type) in attributes {
            if attr_type.is_err() {
                let (attr_type, attr_type_line) = attr_type.unwrap_err();
                return Err(ProcessorError::TypeNotFound(attr_type_line, attr_type.0));
            }

            attributes_processed.push((attr_name, attr_type.unwrap()))
        }

        type_table.add_type(
            type_.id,
            Box::new(UserType::new(
                name,
                type_.id,
                line,
                attributes_processed,
                None,
            )),
        )
    }

    let mut typed_fns = HashMap::new();
    let mut fn_name_map = HashMap::new();
    fn_name_map.insert(None, HashMap::new());
    let mut id_counter: isize = 1;
    for symbol in remaining_pre_ast {
        match symbol {
            PreprocessSymbol::Impl(line, type_name, functions) => {
                let type_id = type_table
                    .get_id_by_name(&type_name)
                    .ok_or(ProcessorError::BadImplType(line))?;
                fn_name_map
                    .entry(Some(type_id))
                    .or_insert_with(HashMap::new);
                for (function, line) in functions {
                    fn_name_map
                        .get_mut(&Some(type_id))
                        .unwrap()
                        .insert(function.0.clone(), id_counter);

                    if function.0 == "destroy" {
                        type_table
                            .get_type_mut(type_id)
                            .unwrap()
                            .try_set_destructor(&line, id_counter)?;
                    }

                    typed_fns.insert(
                        id_counter,
                        process_function(function, &type_table, id_counter, Some(type_id), line)?,
                    );
                    id_counter += 1;
                }
            }
            PreprocessSymbol::Fn(line, function) => {
                let id = if &function.0 == "main" {
                    0
                } else {
                    id_counter += 1;
                    id_counter - 1
                };
                fn_name_map
                    .get_mut(&None)
                    .unwrap()
                    .insert(function.0.clone(), id);
                typed_fns.insert(id, process_function(function, &type_table, id, None, line)?);
                id_counter += 1;
            }
            _ => panic!("Expected Impl of Functions"),
        }
    }

    if let Some(main) = typed_fns.get(&0) {
        if !main.get_args().is_empty() {
            return Err(ProcessorError::MainFunctionParams);
        }
        if main.get_return_type() != Some(TypeInfo::new(Int::get_id(), 0)) {
            return Err(ProcessorError::MainFunctionBadReturn);
        }
    } else {
        return Err(ProcessorError::NoMainFunction);
    }

    Ok((type_table, fn_name_map, typed_fns))
}

fn process_function(
    function: PreProcessFunction,
    type_table: &TypeTable,
    id: isize,
    _impl_type: Option<isize>,
    line: LineInfo,
) -> Result<Box<dyn TypedFunction>, ProcessorError> {
    let (name, args, return_type, contents) = function;

    let mut args_processed: Vec<(String, TypeInfo)> = Vec::new();

    for (param_name, param_line, type_name, indirection, type_line) in args {
        for (existing_arg, _) in &args_processed {
            if &param_name == existing_arg {
                return Err(ProcessorError::ParameterNameInUse(param_line, param_name));
            }
        }
        args_processed.push((
            param_name,
            TypeInfo::new(
                type_table
                    .get_id_by_name(&type_name)
                    .ok_or(ProcessorError::TypeNotFound(type_line, type_name))?,
                indirection,
            ),
        ));
    }

    let return_type = if let Some((type_name, type_line)) = return_type {
        Some(TypeInfo::new(
            type_table
                .get_id_by_name(&type_name.0)
                .ok_or(ProcessorError::TypeNotFound(type_line, type_name.0))?,
            type_name.1,
        ))
    } else {
        None
    };

    Ok(Box::new(UserTypedFunction {
        id,
        name,
        line,
        args: args_processed,
        return_type,
        contents: Some(contents),
    }))
}
