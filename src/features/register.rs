use std::{fs::{self}, path::{self, PathBuf}};
use mlua::{Lua, Value};
use super::celltype::Celltype;
fn take(file: PathBuf, lua: &Lua, cells: &mut Vec<Celltype>) {
    // let file = file.unwrap().path();
    println!("{}", &file.display());
    let content = fs::read_to_string(&file).unwrap();
    println!("{content}");
    let _ = lua.load(content).exec();
    let celltype = Celltype {
        name: lua.globals().get::<String>("name").unwrap_or("ERROR - FAILED TO LOAD THE NAME".to_string()),
        id: lua.globals().get::<String>("id").unwrap_or("ERROR - FAILED TO LOAD THE ID".to_string()),
        texture: lua.globals().get::<String>("texture").unwrap_or("ERROR - FAILED TO LOAD THE TEXTURE".to_string()),
        author: lua.globals().get::<Vec<String>>("author").unwrap_or(vec!["ERROR - FAILED TO LOAD THE AUTHOR".to_string()]),
        category: lua.globals().get::<Vec<String>>("category").unwrap_or(vec!["ERROR - FAILED TO LOAD THE CATEGORY".to_string()]),
        reference: file.display().to_string(),
    };
    let _ = lua.globals().set("name", Value::Nil);
    let _ = lua.globals().set("id", Value::Nil);
    let _ = lua.globals().set("texture", Value::Nil);
    let _ = lua.globals().set("author", Value::Nil);
    let _ = lua.globals().set("category", Value::Nil);
    println!("{}", celltype.reference);
    cells.push(celltype);
}
pub fn main() -> Vec<Celltype> {
    let mut cells: Vec<Celltype> = Vec::new();
    let lua = Lua::new();
    
    // let celltype: Celltype = Celltype {id: "new".to_string(), name: "new".to_string(), category: vec!["a", "a"].iter().map(|x| x.to_string()).collect::<Vec<String>>()};
    // lua.globals().set("celltype", celltype).unwrap();

    // let f = lua.create_function(|_, ()| -> Result<()> {
    //     println!("hello from rust");
    //     // push(1, 1);
    //     Ok(())
    // }).unwrap();
    // lua.globals().set("a", f).unwrap();
    
    if path::Path::new("./schemapacks/").exists() == false {
        match fs::create_dir("./schemapacks/") {
            Ok(()) => (),
            Err(_) => panic!("failed to make a ./schemapacks/")
        };
    }
    let packs: fs::ReadDir = fs::read_dir("./schemapacks/").unwrap();
    for p in packs {
        let pack: PathBuf = p.unwrap().path();
        // println!("{}", pack.display());
        if pack.is_dir() {
            let contents: fs::ReadDir = fs::read_dir(&pack).unwrap();
            for p in contents {
                let path: PathBuf = p.unwrap().path();
                // println!("{}", format!(r#"{}\cells"#, &pack.display()));
                println!("{}", &pack.display());
                if path.to_string_lossy().trim() != format!(r#"{}\lua_injector"#, &pack.display()) && path.is_dir() {
                    // println!("{}", &path.to_string_lossy());
                    let lua_files = fs::read_dir(&path).unwrap()
                    .filter(|x| 
                        match x.as_ref().unwrap().path().extension() {
                            Some(file) => {file.to_str()==Some("lua")},
                            None => false
                        });
                    let directories = fs::read_dir(&path).unwrap()
                    .filter(|x| x.as_ref().unwrap().path().is_dir());
                    for file in lua_files {
                        let file = file.unwrap().path();
                        take(file, &lua, &mut cells);
                    }
                    for dir in directories {
                        let lua_files = fs::read_dir(&dir.unwrap().path()).unwrap()
                        .filter(|x| 
                        match x.as_ref().unwrap().path().extension() {
                            Some(file) => {file.to_str()==Some("lua")},
                            None => false
                        });
                        for file in lua_files {
                            let file = file.unwrap().path();
                            take(file, &lua, &mut cells);
                        }
                    }
                }
            }
        }
        // println!("{}", p.unwrap().path().display())
    }
    return cells;
    
}