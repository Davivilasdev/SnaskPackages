use crate::value::Value;
use crate::symbol_table::SymbolTable;
use std::time::{SystemTime, UNIX_EPOCH};

/// Registra funções de sistema na stdlib
pub fn register(globals: &mut SymbolTable) {
    globals.define_native_function("time", |_args| {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Ok(Value::Number(since_the_epoch.as_secs_f64()))
    });

    globals.define_native_function("sleep", |args| {
        if args.len() != 1 { return Err("sleep espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::Number(ms) => {
                let duration = std::time::Duration::from_millis(*ms as u64);
                std::thread::sleep(duration);
                Ok(Value::Nil)
            },
            _ => Err("sleep espera um número (milissegundos)".to_string()),
        }
    });

    globals.define_native_function("exit", |args| {
        let code = if args.is_empty() {
            0
        } else {
            match &args[0] {
                Value::Number(n) => *n as i32,
                _ => 0,
            }
        };
        
        std::process::exit(code);
    });

    globals.define_native_function("args", |_args| {
        let args: Vec<Value> = std::env::args()
            .skip(1) // Pula o nome do executável
            .map(|arg| Value::String(arg))
            .collect();
        Ok(Value::List(args))
    });

    globals.define_native_function("env", |args| {
        if args.len() != 1 { return Err("env espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::String(key) => {
                match std::env::var(key) {
                    Ok(value) => Ok(Value::String(value)),
                    Err(_) => Ok(Value::Nil),
                }
            },
            _ => Err("env espera uma string (nome da variável)".to_string()),
        }
    });

    globals.define_native_function("set_env", |args| {
        if args.len() != 2 { return Err("set_env espera 2 argumentos".to_string()); }
        
        match (&args[0], &args[1]) {
            (Value::String(key), Value::String(value)) => {
                std::env::set_var(key, value);
                Ok(Value::Nil)
            },
            _ => Err("set_env espera duas strings".to_string()),
        }
    });

    globals.define_native_function("cwd", |_args| {
        match std::env::current_dir() {
            Ok(path) => {
                if let Some(path_str) = path.to_str() {
                    Ok(Value::String(path_str.to_string()))
                } else {
                    Err("Erro ao converter caminho".to_string())
                }
            },
            Err(e) => Err(format!("Erro ao obter diretório atual: {}", e)),
        }
    });

    globals.define_native_function("platform", |_args| {
        Ok(Value::String(std::env::consts::OS.to_string()))
    });

    globals.define_native_function("arch", |_args| {
        Ok(Value::String(std::env::consts::ARCH.to_string()))
    });
}
