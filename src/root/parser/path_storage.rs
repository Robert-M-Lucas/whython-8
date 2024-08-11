use crate::root::errors::parser_errors::create_custom_error;
use crate::root::errors::WErr;
use crate::root::parser::location::Location;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::utils::identify_first_last::IdentifyLast;
use derive_getters::Getters;
use derive_new::new;
use nom::character::complete::anychar;
use nom::InputTake;
use std::collections::HashMap;
use std::fs;

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub struct FileID(usize);

impl FileID {
    pub fn main_file() -> FileID {
        FileID(0)
    }
}

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub struct FolderID(usize);

#[derive(Getters)]
pub struct CodeFolder {
    parent: FolderID,
    child_folders: HashMap<String, FolderID>,
    child_files: HashMap<String, FileID>,
    current: String,
}

impl CodeFolder {
    pub fn root() -> CodeFolder {
        CodeFolder {
            parent: FolderID(0),
            child_folders: Default::default(),
            child_files: Default::default(),
            current: "[ROOT]".to_string(),
        }
    }
}

#[derive(Clone, Default, Getters, new)]
pub struct Scope {
    files_used: Vec<(FileID, Location)>,
    files_imported: Vec<(FileID, Location)>,
    folders_imported: Vec<(FolderID, Location)>,
}

#[derive(Getters)]
pub struct CodeFile {
    parent: FolderID,
    current: String,
    scope: Scope,
}

pub struct PathStorage {
    folders: Vec<CodeFolder>,
    files: Vec<CodeFile>,
}

impl PathStorage {
    pub fn new(main: &str) -> Result<PathStorage, WErr> {
        // TODO: Only allow certain characters in base
        if !main.ends_with(".why") {
            todo!()
        }
        let main = &main[..main.len() - ".why".len()];

        let mut folders = vec![CodeFolder::root()];
        let mut files = Vec::new();

        let mut current = FolderID(0);
        for (is_last, section) in main.split('/').identify_last() {
            if is_last {
                files.push(CodeFile {
                    parent: current,
                    current: section.to_string(),
                    scope: Scope::default(),
                });
            } else {
                folders.push(CodeFolder {
                    parent: current,
                    child_folders: Default::default(),
                    child_files: Default::default(),
                    current: section.to_string(),
                });
                current = FolderID(folders.len() - 1);
            }
        }

        Ok(PathStorage { folders, files })
    }

    #[inline(always)]
    pub fn get_file(&self, file_id: FileID) -> &CodeFile {
        &self.files[file_id.0]
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn get_file_mut(&mut self, file_id: FileID) -> &mut CodeFile {
        &mut self.files[file_id.0]
    }

    #[inline(always)]
    pub fn get_folder(&self, folder_id: FolderID) -> &CodeFolder {
        &self.folders[folder_id.0]
    }

    #[inline(always)]
    pub fn get_folder_mut(&mut self, folder_id: FolderID) -> &mut CodeFolder {
        &mut self.folders[folder_id.0]
    }

    pub fn reconstruct_file(&self, id: FileID) -> String {
        let mut sb = self.get_file(id).current.clone() + ".why";
        let mut current = self.get_file(id).parent;
        while current.0 != 0 {
            sb = self.get_folder(current).current.clone() + std::path::MAIN_SEPARATOR_STR + &sb;
            current = self.get_folder(current).parent;
        }
        sb
    }

    pub fn reconstruct_folder(&self, id: FolderID) -> String {
        let mut sb = self.get_folder(id).current.to_string();
        let mut current = self.get_folder(id).parent;
        while current.0 != 0 {
            sb = self.get_folder(current).current.clone() + std::path::MAIN_SEPARATOR_STR + &sb;
            current = self.get_folder(current).parent;
        }
        sb
    }

    pub fn get_id_and_add_to_file<'a>(
        &mut self,
        current_file: FileID,
        is_use: bool,
        path_span: Span<'a>,
    ) -> ParseResult<(), Vec<FileID>, ErrorTree<'a>> {
        let mut path_span = path_span;

        let Some(last) = path_span.chars().last() else {
            todo!()
        };
        // let wildcard = if last == '*' {
        //     let (_, p) = path_rem.take_split(path_rem.len() - 1);
        //     path_rem = p;
        //     true
        // } else {
        //     false
        // };

        let is_folder = if let Some(last) = path_span.chars().last() {
            if last == '/' {
                let (_, p) = path_span.take_split(path_span.len() - 1);
                path_span = p;
                true
            } else {
                false
            }
        } else {
            false
        };

        let is_absolute = if let Some(last) = path_span.chars().next() {
            if last == '/' {
                let (p, _) = path_span.take_split(1);
                path_span = p;
                true
            } else {
                false
            }
        } else {
            false
        };

        let mut path_rem = path_span;

        // if wildcard && !is_folder {
        //     todo!()
        // }

        while let Ok((rem, c)) = anychar::<_, ErrorTree>(path_rem) {
            if c.is_alphanumeric() || c == '_' || c == '/' {
                path_rem = rem;
                continue;
            }

            let mut utf8 = [0u8; 4];
            c.encode_utf8(&mut utf8);
            let mut utf8_str = "[".to_string();
            utf8_str += &utf8.map(|b| format!("{b:02X}")).join(", ");
            utf8_str.push(']');

            return Err(create_custom_error(
                format!("Invalid character in path '{}' - UTF-8 bytes: {}. Allowed characters are alphanumerics, '_' and '/'", c, utf8_str),
                path_rem,
            ));
        }

        let mut current: FolderID = if is_absolute {
            FolderID(0)
        } else {
            self.get_file(current_file).parent
        };

        for (is_last, section) in path_span.split_terminator('/').identify_last() {
            if is_last {
                if is_folder {
                    let folder = self.add_folder(section, current);
                    if !is_use {
                        self.get_file_mut(current_file)
                            .scope
                            .folders_imported
                            .push((folder, Location::from_span(&path_span)));
                    }
                    let folder_path = self.reconstruct_folder(folder);
                    let mut new_files = Vec::new();

                    let Ok(subpaths) = fs::read_dir(folder_path) else {
                        todo!()
                    };
                    for path in subpaths {
                        let Ok(path) = path else { todo!() };
                        let Ok(t) = path.file_type() else { todo!() };
                        if !t.is_file() {
                            continue;
                        }
                        let path = path.path();
                        if !path
                            .extension()
                            .and_then(|e| e.to_str())
                            .is_some_and(|e| e == "why")
                        {
                            continue;
                        }
                        let Some(name) = path.file_stem().and_then(|f| f.to_str()) else {
                            todo!()
                        };

                        let (file, is_new) = self.add_file(name, folder);
                        if is_new {
                            new_files.push(file);
                        }
                        if is_use {
                            self.get_file_mut(current_file)
                                .scope
                                .files_imported
                                .push((file, Location::from_span(&path_span)));
                        }
                    }

                    return Ok(((), new_files));
                } else {
                    let (file, is_new) = self.add_file(section, current);

                    if is_use {
                        self.get_file_mut(current_file)
                            .scope
                            .files_used
                            .push((file, Location::from_span(&path_span)));
                    } else {
                        self.get_file_mut(current_file)
                            .scope
                            .files_imported
                            .push((file, Location::from_span(&path_span)));
                    }

                    return if is_new {
                        Ok(((), vec![file]))
                    } else {
                        Ok(((), vec![]))
                    };
                }
            } else {
                current = self.add_folder(section, current);
            }
        }

        panic!()
    }

    fn add_file(&mut self, name: &str, parent: FolderID) -> (FileID, bool) {
        if let Some(id) = self.get_folder(parent).child_files.get(name) {
            (*id, false)
        } else {
            self.files.push(CodeFile {
                parent,
                current: name.to_string(),
                scope: Scope::default(),
            });
            let id = FileID(self.files.len() - 1);
            self.get_folder_mut(parent)
                .child_files
                .insert(name.to_string(), id);
            (id, true)
        }
    }

    fn add_folder(&mut self, name: &str, parent: FolderID) -> FolderID {
        if let Some(id) = self.get_folder(parent).child_folders.get(name) {
            *id
        } else {
            self.folders.push(CodeFolder {
                parent,
                child_folders: Default::default(),
                child_files: Default::default(),
                current: name.to_string(),
            });
            let id = FolderID(self.folders.len() - 1);
            self.get_folder_mut(parent)
                .child_folders
                .insert(name.to_string(), id);
            id
        }
    }
}
