use crate::root::errors::parser_errors::{create_custom_error, create_custom_error_tree};
use crate::root::errors::WErr;
use crate::root::parser::location::{Location, LocationFilledFmt};
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::utils::identify_first_last::{IdentifyFirstLast, IdentifyLast};
use nom::character::complete::anychar;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub struct FileID(usize);

impl FileID {
    pub fn main_file() -> FileID {
        FileID(0)
    }
}

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub struct FolderID(usize);

struct CodeFolder {
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

struct CodeFile {
    parent: FolderID,
    current: String,
    use_files: Vec<FileID>,
    use_folders: Vec<FolderID>,
}

pub struct PathStorage {
    folders: Vec<CodeFolder>,
    files: Vec<CodeFile>,
}

impl PathStorage {
    pub fn new(base: &str) -> Result<PathStorage, WErr> {
        // TODO: Only allow certain characters in base
        let mut folders = vec![CodeFolder::root()];
        let mut files = Vec::new();

        let mut current = FolderID(0);
        for (is_last, section) in base.split('/').identify_last() {
            if is_last {
                files.push(CodeFile {
                    parent: current,
                    current: section.to_string(),
                    use_files: vec![],
                    use_folders: vec![],
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

    fn get_file(&self, file_id: FileID) -> &CodeFile {
        &self.files[file_id.0]
    }

    fn get_file_mut(&mut self, file_id: FileID) -> &mut CodeFile {
        &mut self.files[file_id.0]
    }

    fn get_folder(&self, folder_id: FolderID) -> &CodeFolder {
        &self.folders[folder_id.0]
    }

    fn get_folder_mut(&mut self, folder_id: FolderID) -> &mut CodeFolder {
        &mut self.folders[folder_id.0]
    }

    pub fn reconstruct_file(&self, id: FileID) -> String {
        let mut sb = self.get_file(id).current.clone();
        let mut current = self.get_file(id).parent;
        while current.0 != 0 {
            sb = self.get_folder(current).current.clone() + "/" + &sb;
            current = self.get_folder(current).parent;
        }
        sb
    }

    pub fn reconstruct_folder(&self, id: FolderID) -> String {
        let mut sb = format!("{}", &self.get_folder(id).current);
        let mut current = self.get_folder(id).parent;
        while current.0 != 0 {
            sb = self.get_folder(current).current.clone() + "/" + &sb;
            current = self.get_folder(current).parent;
        }
        sb
    }

    pub fn get_file_path_id_checked<'a>(
        &mut self,
        path: Span<'a>,
    ) -> ParseResult<(), (FileID, bool), ErrorTree<'a>> {
        let mut path_rem = path;
        let mut last_dot = false;
        while let Ok((rem, c)) = anychar::<_, ErrorTree>(path_rem) {
            if c.is_alphanumeric() || c == '_' || c == '/' {
                last_dot = false;
                path_rem = rem;
                continue;
            }
            if c == '.' {
                if last_dot {
                    return Err(create_custom_error(
                        "Double '.'s not allowed in path".to_string(),
                        path_rem,
                    ));
                }
                path_rem = rem;
                last_dot = true;
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

        let mut current = FolderID(0);

        for (is_last, section) in path.split('/').identify_last() {
            if is_last {
                return if let Some(id) = self.get_folder(current).child_files.get(section) {
                    Ok(((), (*id, false)))
                } else {
                    self.files.push(CodeFile {
                        parent: current,
                        current: section.to_string(),
                        use_files: vec![],
                        use_folders: vec![],
                    });
                    let id = FileID(self.files.len() - 1);
                    self.get_folder_mut(current)
                        .child_files
                        .insert(section.to_string(), id);
                    Ok(((), (id, true)))
                };
            } else {
                current = if let Some(id) = self.get_folder(current).child_folders.get(section) {
                    *id
                } else {
                    self.folders.push(CodeFolder {
                        parent: current,
                        child_folders: Default::default(),
                        child_files: Default::default(),
                        current: section.to_string(),
                    });
                    let id = FolderID(self.folders.len() - 1);
                    self.get_folder_mut(current)
                        .child_folders
                        .insert(section.to_string(), id);
                    id
                };
            }
        }

        panic!()
    }
}
