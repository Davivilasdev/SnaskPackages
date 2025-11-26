use crate::value::Value;
use crate::symbol_table::SymbolTable;
use std::fs;
use std::path::Path;

/// Registra funções de I/O (arquivos) na stdlib
pub fn register(globals: &mut SymbolTable) {
    globals.define_native_function("read_file", |args| {
        if args.len() != 1 { return Err("read_file espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::String(path) => {
                match fs::read_to_string(path) {
                    Ok(content) => Ok(Value::String(content)),
                    Err(e) => Err(format!("Erro ao ler arquivo: {}", e)),
                }
            },
            _ => Err("read_file espera uma string (caminho do arquivo)".to_string()),
        }
    });

    globals.define_native_function("write_file", |args| {
        if args.len() != 2 { return Err("write_file espera 2 argumentos".to_string()); }
        
        match (&args[0], &args[1]) {
            (Value::String(path), Value::String(content)) => {
                match fs::write(path, content) {
                    Ok(_) => Ok(Value::Boolean(true)),
                    Err(e) => Err(format!("Erro ao escrever arquivo: {}", e)),
                }
            },
            _ => Err("write_file espera duas strings (caminho e conteúdo)".to_string()),
        }
    });

    globals.define_native_function("append_file", |args| {
        if args.len() != 2 { return Err("append_file espera 2 argumentos".to_string()); }
        
        match (&args[0], &args[1]) {
            (Value::String(path), Value::String(content)) => {
                use std::fs::OpenOptions;
                use std::io::Write;

                match OpenOptions::new().create(true).append(true).open(path) {
                    Ok(mut file) => {
                        match file.write_all(content.as_bytes()) {
                            Ok(_) => Ok(Value::Boolean(true)),
                            Err(e) => Err(format!("Erro ao adicionar ao arquivo: {}", e)),
                        }
                    },
                    Err(e) => Err(format!("Erro ao abrir arquivo: {}", e)),
                }
            },
            _ => Err("append_file espera duas strings (caminho e conteúdo)".to_string()),
        }
    });

    globals.define_native_function("exists", |args| {
        if args.len() != 1 { return Err("exists espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::String(path) => {
                Ok(Value::Boolean(Path::new(path).exists()))
            },
            _ => Err("exists espera uma string (caminho)".to_string()),
        }
    });

    globals.define_native_function("delete", |args| {
        if args.len() != 1 { return Err("delete espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::String(path) => {
                let path_obj = Path::new(path);
                
                let result = if path_obj.is_dir() {
                    fs::remove_dir_all(path)
                } else {
                    fs::remove_file(path)
                };

                match result {
                    Ok(_) => Ok(Value::Boolean(true)),
                    Err(e) => Err(format!("Erro ao deletar: {}", e)),
                }
            },
            _ => Err("delete espera uma string (caminho)".to_string()),
        }
    });

    globals.define_native_function("read_dir", |args| {
        if args.len() != 1 { return Err("read_dir espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::String(path) => {
                match fs::read_dir(path) {
                    Ok(entries) => {
                        let mut files = Vec::new();
                        for entry in entries {
                            if let Ok(entry) = entry {
                                if let Some(name) = entry.path().file_name() {
                                    if let Some(name_str) = name.to_str() {
                                        files.push(Value::String(name_str.to_string()));
                                    }
                                }
                            }
                        }
                        Ok(Value::List(files))
                    },
                    Err(e) => Err(format!("Erro ao ler diretório: {}", e)),
                }
            },
            _ => Err("read_dir espera uma string (caminho do diretório)".to_string()),
        }
    });

    globals.define_native_function("is_file", |args| {
        if args.len() != 1 { return Err("is_file espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::String(path) => {
                Ok(Value::Boolean(Path::new(path).is_file()))
            },
            _ => Err("is_file espera uma string (caminho)".to_string()),
        }
    });

    globals.define_native_function("is_dir", |args| {
        if args.len() != 1 { return Err("is_dir espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::String(path) => {
                Ok(Value::Boolean(Path::new(path).is_dir()))
            },
            _ => Err("is_dir espera uma string (caminho)".to_string()),
        }
    });

    globals.define_native_function("create_dir", |args| {
        if args.len() != 1 { return Err("create_dir espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::String(path) => {
                match fs::create_dir_all(path) {
                    Ok(_) => Ok(Value::Boolean(true)),
                    Err(e) => Err(format!("Erro ao criar diretório: {}", e)),
                }
            },
            _ => Err("create_dir espera uma string (caminho)".to_string()),
        }
    });
}
