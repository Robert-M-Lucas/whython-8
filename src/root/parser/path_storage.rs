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

/// ID corresponding to a file
#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub struct FileID(usize);

impl FileID {
    pub const MAIN_FILE: FileID = FileID(0);
}

/// ID representing a folder
#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub struct FolderID(usize);

impl FolderID {
    pub const ROOT_FOLDER: FolderID = FolderID(0);
}

/// Node representing a folder, its files, and subfolders
#[derive(Getters)]
pub struct CodeFolder {
    parent: FolderID,
    child_folders: HashMap<String, FolderID>,
    child_files: HashMap<String, FileID>,
    current: String,
}

impl CodeFolder {
    /// Creates the root folder
    pub fn root() -> CodeFolder {
        CodeFolder {
            parent: FolderID::ROOT_FOLDER,
            child_folders: Default::default(),
            child_files: Default::default(),
            current: "[ROOT]".to_string(),
        }
    }
}

/// A scope representing imported files, folders, etc.
#[derive(Clone, Default, Getters, new)]
pub struct Scope {
    files_used: Vec<(FileID, Location)>,
    files_imported: Vec<(FileID, Location)>,
    folders_imported: Vec<(FolderID, Location)>,
}

/// Represents a file of code with a scope
#[derive(Getters)]
pub struct CodeFile {
    parent: FolderID,
    current: String,
    scope: Scope,
}

/// Storage struct for the file structure
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

        let mut current = FolderID::ROOT_FOLDER;
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

    /// Returns a `CodeFile` ref by its ID
    #[inline(always)]
    pub fn get_file(&self, file_id: FileID) -> &CodeFile {
        &self.files[file_id.0]
    }


    /// Returns a `CodeFile` mut ref by its ID
    #[allow(dead_code)]
    #[inline(always)]
    pub fn get_file_mut(&mut self, file_id: FileID) -> &mut CodeFile {
        &mut self.files[file_id.0]
    }


    /// Returns a `CodeFolder` ref by its ID
    #[inline(always)]
    pub fn get_folder(&self, folder_id: FolderID) -> &CodeFolder {
        &self.folders[folder_id.0]
    }


    /// Returns a `CodeFolder` mut ref by its ID
    #[inline(always)]
    pub fn get_folder_mut(&mut self, folder_id: FolderID) -> &mut CodeFolder {
        &mut self.folders[folder_id.0]
    }

    /// Reconstructs a file path from its ID
    pub fn reconstruct_file(&self, id: FileID) -> String {
        let mut sb = self.get_file(id).current.clone() + ".why";
        let mut current = self.get_file(id).parent;
        while current.0 != 0 {
            sb = self.get_folder(current).current.clone() + std::path::MAIN_SEPARATOR_STR + &sb;
            current = self.get_folder(current).parent;
        }
        sb
    }

    /// Reconstructs a folder path from its ID
    pub fn reconstruct_folder(&self, id: FolderID) -> String {
        let mut sb = self.get_folder(id).current.to_string();
        let mut current = self.get_folder(id).parent;
        while current.0 != 0 {
            sb = self.get_folder(current).current.clone() + std::path::MAIN_SEPARATOR_STR + &sb;
            current = self.get_folder(current).parent;
        }
        sb
    }

    /// Adds an import and adds it to a file, returning new files 
    /// required to compile the current file
    pub fn get_id_and_add_to_file<'a>(
        &mut self,
        current_file: FileID,
        is_use: bool,
        path_span: Span<'a>,
    ) -> ParseResult<(), Vec<FileID>, ErrorTree<'a>> {
        let mut path_span = path_span;

        // let Some(last) = path_span.chars().last() else {
        //     todo!()
        // };
        // let wildcard = if last == '*' {
        //     let (_, p) = path_rem.take_split(path_rem.len() - 1);
        //     path_rem = p;
        //     true
        // } else {
        //     false
        // };
        
        // Folders must end in '/'
        let is_folder = if let Some(last) = path_span.chars().last() {
            if last == '/' {
                // Remove folder from span
                let (_, p) = path_span.take_split(path_span.len() - 1);
                path_span = p;
                true
            } else {
                false
            }
        } else {
            false
        };
        
        // Gets whether the path is absolute or relative (starts with / means absolute)
        let is_absolute = if let Some(next) = path_span.chars().next() {
            if next == '/' {
                let (p, _) = path_span.take_split(1);
                path_span = p;
                true
            } else {
                false
            }
        } else {
            false
        };

        // Path remaining
        let mut path_rem = path_span;

        // if wildcard && !is_folder {
        //     todo!()
        // }

        // Check path characters
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
            FolderID::ROOT_FOLDER
        } else {
            self.get_file(current_file).parent
        };

        for (is_last, section) in path_span.split_terminator('/').identify_last() {
            if is_last {
                if is_folder {
                    let folder = self.add_folder(section, current);
                    // Import folder if not use
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
                    // Add all subfiles
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
                            // Add files imported by use
                            self.get_file_mut(current_file)
                                .scope
                                .files_imported
                                .push((file, Location::from_span(&path_span)));
                        }
                    }

                    return Ok(((), new_files));
                } else {
                    // Add file
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
                // Add subfolder
                current = self.add_folder(section, current);
            }
        }

        panic!()
    }

    /// Adds a file if it does not exist to a folder
    /// Returns the ID and whether the file is new
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

    /// Adds a folder to a parent folder
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
