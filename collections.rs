use crate::value::Value;
use crate::symbol_table::SymbolTable;

/// Registra funções de coleções (map, filter, reduce, etc) na stdlib
pub fn register(globals: &mut SymbolTable) {
    globals.define_native_function("map", |args| {
        if args.len() != 2 { return Err("map espera 2 argumentos".to_string()); }
        
        match (&args[0], &args[1]) {
            (Value::List(list), Value::Function(_func_decl)) => {
                // TODO: Implementar chamada de função para cada elemento
                // Por enquanto, retorna a lista original
                Ok(Value::List(list.clone()))
            },
            _ => Err("map espera uma lista e uma função".to_string()),
        }
    });

    globals.define_native_function("filter", |args| {
        if args.len() != 2 { return Err("filter espera 2 argumentos".to_string()); }
        
        match (&args[0], &args[1]) {
            (Value::List(list), Value::Function(_func_decl)) => {
                // TODO: Implementar filtragem com função
                Ok(Value::List(list.clone()))
            },
            _ => Err("filter espera uma lista e uma função".to_string()),
        }
    });

    globals.define_native_function("reduce", |args| {
        if args.len() < 2 { return Err("reduce espera pelo menos 2 argumentos".to_string()); }
        
        match (&args[0], &args[1]) {
            (Value::List(_list), Value::Function(_func_decl)) => {
                // TODO: Implementar reduce com função
                Ok(Value::Nil)
            },
            _ => Err("reduce espera uma lista e uma função".to_string()),
        }
    });

    globals.define_native_function("find", |args| {
        if args.len() != 2 { return Err("find espera 2 argumentos".to_string()); }
        
        match (&args[0], &args[1]) {
            (Value::List(_list), Value::Function(_func_decl)) => {
                // TODO: Implementar find com função
                Ok(Value::Nil)
            },
            _ => Err("find espera uma lista e uma função".to_string()),
        }
    });

    globals.define_native_function("any", |args| {
        if args.len() != 2 { return Err("any espera 2 argumentos".to_string()); }
        
        match (&args[0], &args[1]) {
            (Value::List(_list), Value::Function(_func_decl)) => {
                // TODO: Implementar any com função
                Ok(Value::Boolean(false))
            },
            _ => Err("any espera uma lista e uma função".to_string()),
        }
    });

    globals.define_native_function("all", |args| {
        if args.len() != 2 { return Err("all espera 2 argumentos".to_string()); }
        
        match (&args[0], &args[1]) {
            (Value::List(_list), Value::Function(_func_decl)) => {
                // TODO: Implementar all com função
                Ok(Value::Boolean(true))
            },
            _ => Err("all espera uma lista e uma função".to_string()),
        }
    });

    globals.define_native_function("reverse", |args| {
        if args.len() != 1 { return Err("reverse espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::List(list) => {
                let mut reversed = list.clone();
                reversed.reverse();
                Ok(Value::List(reversed))
            },
            _ => Err("reverse espera uma lista".to_string()),
        }
    });

    globals.define_native_function("sort", |args| {
        if args.len() != 1 { return Err("sort espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::List(list) => {
                let mut sorted = list.clone();
                sorted.sort_by(|a, b| {
                    match (a, b) {
                        (Value::Number(n1), Value::Number(n2)) => {
                            n1.partial_cmp(n2).unwrap_or(std::cmp::Ordering::Equal)
                        },
                        (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
                        _ => std::cmp::Ordering::Equal,
                    }
                });
                Ok(Value::List(sorted))
            },
            _ => Err("sort espera uma lista".to_string()),
        }
    });

    globals.define_native_function("unique", |args| {
        if args.len() != 1 { return Err("unique espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::List(list) => {
                let mut unique_list = Vec::new();
                for item in list {
                    if !unique_list.contains(item) {
                        unique_list.push(item.clone());
                    }
                }
                Ok(Value::List(unique_list))
            },
            _ => Err("unique espera uma lista".to_string()),
        }
    });

    globals.define_native_function("flatten", |args| {
        if args.len() != 1 { return Err("flatten espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::List(list) => {
                let mut flattened = Vec::new();
                for item in list {
                    match item {
                        Value::List(inner_list) => {
                            flattened.extend(inner_list.clone());
                        },
                        other => flattened.push(other.clone()),
                    }
                }
                Ok(Value::List(flattened))
            },
            _ => Err("flatten espera uma lista".to_string()),
        }
    });

    globals.define_native_function("range", |args| {
        if args.len() < 1 || args.len() > 3 {
            return Err("range espera 1, 2 ou 3 argumentos".to_string());
        }

        let (start, end, step) = match args.len() {
            1 => {
                match &args[0] {
                    Value::Number(n) => (0.0, *n, 1.0),
                    _ => return Err("range espera números".to_string()),
                }
            },
            2 => {
                match (&args[0], &args[1]) {
                    (Value::Number(s), Value::Number(e)) => (*s, *e, 1.0),
                    _ => return Err("range espera números".to_string()),
                }
            },
            3 => {
                match (&args[0], &args[1], &args[2]) {
                    (Value::Number(s), Value::Number(e), Value::Number(st)) => (*s, *e, *st),
                    _ => return Err("range espera números".to_string()),
                }
            },
            _ => unreachable!(),
        };

        if step == 0.0 {
            return Err("step não pode ser zero".to_string());
        }

        let mut result = Vec::new();
        let mut current = start;

        if step > 0.0 {
            while current < end {
                result.push(Value::Number(current));
                current += step;
            }
        } else {
            while current > end {
                result.push(Value::Number(current));
                current += step;
            }
        }

        Ok(Value::List(result))
    });
}
