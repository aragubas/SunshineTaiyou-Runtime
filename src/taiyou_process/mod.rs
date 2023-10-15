use std::{collections::HashMap, path::Path, io::Read};
use crate::taiyou_package::TaiyouPackage;
use rhai::{Engine, Scope};
use serde_json::{Result, Value};

pub struct PackageProcess {
  owner_package: TaiyouPackage,
  process_id: u16
}


impl PackageProcess {
  pub fn new(package: TaiyouPackage, manager: &mut ProcessManager) -> PackageProcess {
    manager.process_counter += 1;
    PackageProcess { owner_package: package, process_id: manager.process_counter }
  }
}

pub struct ProcessManager {
  // Number of process currently active
  pub process_counter: u16,
  pub processes: HashMap<u16, PackageProcess>
}

impl ProcessManager {
  // Creates new instance of ProcessManager
  pub fn new() -> ProcessManager {
    ProcessManager { 
      process_counter: 0,
      processes: HashMap::new()
    }
  }

  pub fn create_process(&mut self, package_path: String) {
    // Check if Taiyou Package exists
    let root_folder = Path::new(&package_path);
    let runtime_folder = root_folder.join(Path::new("runtime"));
    let metadata_file_path = root_folder.join(Path::new("metadata.json"));
    
    // TODO: Handle path exists errors
    
    let mut metadata_file = match std::fs::File::open(metadata_file_path) {
      Ok(file) => file,
      Err(error) => { 
        eprintln!("Error while opening metadata file: {}", error); 
        return; 
      }
    };  

    let mut metadata_file_contents = String::new();

    match metadata_file.read_to_string(&mut metadata_file_contents) {
      Ok(_) => { },
      Err(error) => {
        eprint!("Could not read metadata file: {}", error);
        return;
      }
    }

    let deserialized_metadata: TaiyouPackage = match serde_json::from_str(metadata_file_contents.as_str()) {
      Ok(result) => result,
      Err(error) => {
        eprint!("Could not serialize metadata file: {}", error);
        return;
      }
    };

    println!("[TaiyouProcess] Loading package \"{}\"", deserialized_metadata.id);

    let engine = Engine::new();
    let main_module = match engine.compile_file(Path::new(&runtime_folder).join(Path::new("main.rhai"))) {
      Ok(module) => module,
      Err(error) => {
        eprintln!("Could not compile main module: {}", error);
        return;
      }
    };
    let mut scope = Scope::new();

    match engine.call_fn::<()>(&mut scope, &main_module, "main", ()) {
      Ok(_) => (),
      Err(error) => {
        eprintln!("Could not call main method: {}", error);
        return;
      }
    }


  }
}