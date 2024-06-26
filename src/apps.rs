pub mod booktore;
pub mod mas;

use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    usize,
};

#[derive(Debug, Clone)]
pub struct Change {
    pub from: String,
    pub to: String,
}

#[derive(Clone)]
pub struct Rename {
    name: String,
    file_name: String,
    pub file_path: PathBuf,
    pub comments: Vec<String>,
    pub changes: Vec<Change>,
}
impl Rename {
    fn new() -> Self {
        Self {
            name: String::new(),
            file_name: String::new(),
            comments: Vec::new(),
            changes: Vec::new(),
            file_path: PathBuf::new(),
        }
    }
    fn setup(mut self, config: &Config, _sphere: &Sphere) -> Self {
        self.name = "rename".to_string();
        self.file_name = format!("{}.on", self.name);
        self.file_path = config.dir.join(&self.file_name);
        ReadParseFile::new(&self.file_path).split_rename(self)
    }
}
impl Config {
    fn new() -> Self {
        Self {
            dir: PathBuf::new(),
            folders: Folders::new(),
            types: Types::new(),
            rename: Rename::new(),
        }
    }

    fn setup(mut self, sphere: &Sphere) -> Self {
        self.dir = sphere.current_dir.join("config");
        self.folders = self.folders.clone().setup(&self);
        self.types = self.types.clone().setup(&self);
        self.rename = self.rename.clone().setup(&self, sphere);
        self
    }
}
#[derive(Clone)]
pub struct Config {
    dir: PathBuf,
    pub folders: Folders,
    pub rename: Rename,
    pub types: Types,
}

#[derive(Clone)]
pub struct Folders {
    name: String,
    file_name: String,
    file_path: PathBuf,
    pub list: Vec<String>,
}
impl Folders {
    fn readparse(&self) -> Vec<String> {
        ReadParseFile::new(&self.file_path).split_whitespace()
    }
    fn new() -> Self {
        Self {
            name: String::new(),
            file_name: String::new(),
            file_path: PathBuf::new(),
            list: Vec::new(),
        }
    }
    fn setup(mut self, config: &Config) -> Self {
        self.name = "folders".to_string();
        self.file_name = format!("{}.on", self.name);
        self.file_path = config.dir.join(&self.file_name);
        self.list = self.readparse();
        self
    }
}

#[derive(Clone)]
pub struct Types {
    name: String,
    file_name: String,
    file_path: PathBuf,
    pub list: Vec<String>,
}
impl Types {
    fn new() -> Self {
        Self {
            name: String::new(),
            file_name: String::new(),
            file_path: PathBuf::new(),
            list: Vec::new(),
        }
    }
    fn setup(mut self, config: &Config) -> Self {
        self.name = "types".to_string();
        self.file_name = format!("{}.on", self.name);
        self.file_path = config.dir.join(&self.file_name);
        self.list = ReadParseFile::new(&self.file_path).split_whitespace();
        self
    }
}

pub struct ReadParseFile {
    pub content: String,
}

impl ReadParseFile {
    pub fn new<T: AsRef<Path>>(name: T) -> Self {
        let content = fs::read_to_string(name).unwrap();
        Self { content }
    }

    pub fn split_mas(self, mas: Mas) -> Vec<Mas> {
        self.content
            .split("\n")
            .enumerate()
            .map(|(index, word)| {
                let mut new_mas = mas.clone();
                new_mas.line = index + 1;
                new_mas.word = word.trim().to_owned();
                new_mas
            })
            .filter(|mas| mas.word.len() > 0)
            .collect()
    }

    pub fn split_whitespace(&self) -> Vec<String> {
        self.content
            .split_whitespace()
            .map(|line| line.trim())
            .filter(|line| line.len() > 0)
            .map(|line| line.to_owned())
            .collect()
    }

    pub fn split_rename(self, mut rename: Rename) -> Rename {
        let list: Vec<_> = self
            .content
            .split("\n")
            .map(|line| line.trim())
            .filter(|line| line.len() > 0)
            .collect();
        let comments: Vec<String> = list
            .iter()
            .take_while(|line| &line[0..2] == "//")
            .map(|line| line.to_string())
            .collect();
        let rest: Vec<_> = list
            .iter()
            .skip(comments.len())
            .map(|line| line.to_owned())
            .collect();

        let changes: Vec<_> = rest
            .iter()
            .map(|line| {
                let inner: Vec<_> = line.split("to:").collect();
                Change {
                    from: inner[0][5..].trim().to_owned(),
                    to: inner[1].trim().to_owned(),
                }
            })
            .collect();

        rename.comments = comments;
        rename.changes = changes;
        rename
    }
}

#[derive(Clone, Debug)]
pub struct Mas {
    pub line: usize,
    pub word: String,
    pub tipo: String,
    pub folder: String,
}

impl Mas {
    pub fn new() -> Self {
        Self {
            line: 0,
            word: String::new(),
            tipo: String::new(),
            folder: String::new(),
        }
    }

    fn setup(mut self, folder: &String, tipo: &String) -> Self {
        self.folder = folder.to_owned();
        self.tipo = tipo.to_owned();
        self
    }
}

#[derive(Clone)]
pub struct File {
    pub ext: String,
    pub name: String,
    pub full_name: String,
    pub path: PathBuf,
}
impl File {
    fn new() -> Self {
        Self {
            ext: String::new(),
            name: String::new(),
            full_name: String::new(),
            path: PathBuf::new(),
        }
    }
    fn setup(mut self, name: &String, carrier: &Carrier) -> Self {
        self.name = name.to_owned();
        self.ext = "on".to_owned();
        self.full_name = format!("{}.{}", &self.name, &self.ext);
        self.path = carrier.dir.join(&self.full_name);
        self
    }
}
#[derive(Clone)]
pub struct Carrier {
    pub dir: PathBuf,
    pub folder: String,
    pub paths: Vec<File>,
}
impl Carrier {
    fn new() -> Self {
        Self {
            dir: PathBuf::new(),
            folder: String::new(),
            paths: Vec::new(),
        }
    }
    fn get_data_paths(&self, types: &Vec<String>) -> Vec<File> {
        let mut acc = Vec::new();
        for tipo in types {
            acc.push(File::new().clone().setup(tipo, &self));
        }
        acc
    }
    fn setup(mut self, voc: &Vocabulary, types: &Vec<String>, folder: &String) -> Self {
        self.folder = folder.to_owned();
        self.dir = voc.dir.join(folder);
        self.paths = self.get_data_paths(types);
        self
    }
}

#[derive(Clone)]
pub struct Vocabulary {
    pub name: String,
    pub dir: PathBuf,
    pub carrier: Vec<Carrier>,
    pub data: HashMap<String, Vec<Mas>>,
    pub data_all: Vec<Mas>,
    pub core_data: HashMap<String, HashMap<String, Vec<Mas>>>,
}
impl Vocabulary {
    fn new() -> Self {
        Self {
            name: String::new(),
            carrier: Vec::new(),
            data: HashMap::new(),
            dir: PathBuf::new(),
            data_all: Vec::new(),
            core_data: HashMap::new(),
        }
    }

    fn read_files_data(
        &self,
    ) -> (
        HashMap<String, Vec<Mas>>,
        HashMap<String, HashMap<String, Vec<Mas>>>,
    ) {
        let mut acc = HashMap::new();
        let mut layout_data: HashMap<String, HashMap<String, Vec<Mas>>> = HashMap::new();
        for carrier in &self.carrier {
            let a = layout_data
                .entry(carrier.folder.to_owned())
                .or_insert_with(|| HashMap::new());

            for file in &carrier.paths {
                let b = a.entry(file.name.clone()).or_insert_with(|| Vec::new());

                let mas = Mas::new().setup(&carrier.folder, &file.name);
                for mas in ReadParseFile::new(&file.path).split_mas(mas) {
                    let inner = acc.entry(mas.tipo.to_owned()).or_insert(Vec::new());
                    b.push(mas.clone());
                    inner.push(mas);
                }
            }
        }
        (acc, layout_data)
    }

    fn create_folders_file_no_exists(&self) {
        for carrier in &self.carrier {
            if !carrier.dir.exists() {
                fs::create_dir(&carrier.dir).unwrap();
            }
            for file in &carrier.paths {
                if !file.path.exists() {
                    fs::File::create(&file.path).unwrap();
                }
            }
        }
    }

    fn get_carrier(&self, sphere: &Sphere) -> Vec<Carrier> {
        let mut acc = vec![];
        for folder in &sphere.config.folders.list {
            let types = &sphere.config.types.list;
            let carrier = Carrier::new().setup(&self, &types, folder);
            acc.push(carrier);
        }
        acc
    }

    fn copy_data_into_array(&self) -> Vec<Mas> {
        let mut acc = vec![];
        for (_, data) in &self.data {
            for mas in data {
                acc.push(mas.clone());
            }
        }
        acc
    }
    fn get_name(&self, name: &str) -> String {
        let new_name = if name == "aparter" {
            "aparter/vocabulary"
        } else {
            "vocabulary"
        };

        format!("{}", new_name)
    }

    fn setup(mut self, sphere: &Sphere, name: &str) -> Self {
        // let compose = format!("{}", name);

        self.name = self.get_name(name);
        self.dir = sphere.current_dir.join(&self.name);
        self.carrier = self.get_carrier(&sphere);
        self.create_folders_file_no_exists();
        let together = self.read_files_data();
        self.data = together.0;
        self.core_data = together.1;
        self.data_all = self.copy_data_into_array();
        self
    }
}

pub fn extract_forbid_words(sphere: &Sphere) -> Vec<String> {
    sphere
        .vocabulary
        .data_all
        .iter()
        .map(|mas| mas.word.to_string())
        .collect()
}

#[derive(Clone)]
pub struct Sphere {
    pub current_dir: PathBuf,
    pub config: Config,
    pub vocabulary: Vocabulary,
    pub name: String,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            current_dir: PathBuf::new(),
            config: Config::new(),
            vocabulary: Vocabulary::new(),
            name: String::new(),
        }
    }
    pub fn rename_paths(&self) -> Vec<(PathBuf, PathBuf)> {
        let mut acc = Vec::new();
        for folder in &self.config.folders.list {
            let folder_dir = self.vocabulary.dir.join(folder);
            for change in &self.config.rename.changes {
                let old_path = folder_dir.join(format!("{}.on", change.from));
                let new_path = folder_dir.join(format!("{}.on", change.to));
                acc.push((old_path, new_path));
            }
        }
        acc
    }

    pub fn rename_files(&self) {
        for (old_path, new_path) in self.rename_paths() {
            fs::rename(&old_path, &new_path).unwrap();
        }
        let comments = self.config.rename.comments.join("\n");
        fs::write(&self.config.rename.file_path, comments).unwrap();
    }

    pub fn mas(self) -> Self {
        for (tipo, list) in &self.vocabulary.data {
            println!("{}: {} ", tipo, list.len());
        }
        let acc = extract_forbid_words(&self);
        println!("Vocabulary: {:?}", &acc.len());
        mas::Mas::new().setup(&self.name).start(acc);
        self
    }

    pub fn setup(mut self, name: &str) -> Self {
        println!("Vocabulary Running...\n");
        self.current_dir = env::current_dir().unwrap();
        self.config = self.config.clone().setup(&self);
        self.vocabulary = self.vocabulary.clone().setup(&self, name);
        self.name = name.to_string();
        self.rename_files();

        for name in ["release", "build"] {
            let release_dir = Path::new(name);
            if release_dir.is_dir() {
                fs::remove_dir_all(release_dir).unwrap();
            }
            fs::create_dir(release_dir).unwrap();
        }
        self.mas()
    }
}
