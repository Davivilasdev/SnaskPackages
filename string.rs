use crate::value::Value;
use crate::symbol_table::SymbolTable;

/// Registra funções de manipulação de strings na stdlib
pub fn register(globals: &mut SymbolTable) {
    globals.define_native_function("len", |args| {
        if args.len() != 1 { return Err("len espera 1 argumento".to_string()); }
        match &args[0] {
            Value::String(s) => Ok(Value::Number(s.len() as f64)),
            Value::List(l) => Ok(Value::Number(l.len() as f64)),
            _ => Err("len espera uma string ou lista".to_string()),
        }
    });

    globals.define_native_function("upper", |args| {
        if args.len() != 1 { return Err("upper espera 1 argumento".to_string()); }
        match &args[0] {
            Value::String(s) => Ok(Value::String(s.to_uppercase())),
            _ => Err("upper espera uma string".to_string()),
        }
    });

    globals.define_native_function("lower", |args| {
        if args.len() != 1 { return Err("lower espera 1 argumento".to_string()); }
        match &args[0] {
            Value::String(s) => Ok(Value::String(s.to_lowercase())),
            _ => Err("lower espera uma string".to_string()),
        }
    });

    globals.define_native_function("trim", |args| {
        if args.len() != 1 { return Err("trim espera 1 argumento".to_string()); }
        match &args[0] {
            Value::String(s) => Ok(Value::String(s.trim().to_string())),
            _ => Err("trim espera uma string".to_string()),
        }
    });

    globals.define_native_function("split", |args| {
        if args.len() != 2 { return Err("split espera 2 argumentos".to_string()); }
        match (&args[0], &args[1]) {
            (Value::String(s), Value::String(delimiter)) => {
                let parts: Vec<Value> = s.split(delimiter.as_str())
                    .map(|part| Value::String(part.to_string()))
                    .collect();
                Ok(Value::List(parts))
            },
            _ => Err("split espera duas strings".to_string()),
        }
    });

    globals.define_native_function("join", |args| {
        if args.len() != 2 { return Err("join espera 2 argumentos".to_string()); }
        match (&args[0], &args[1]) {
            (Value::List(list), Value::String(separator)) => {
                let strings: Result<Vec<String>, String> = list.iter().map(|v| {
                    match v {
                        Value::String(s) => Ok(s.clone()),
                        _ => Err("join espera uma lista de strings".to_string()),
                    }
                }).collect();

                match strings {
                    Ok(strs) => Ok(Value::String(strs.join(separator))),
                    Err(e) => Err(e),
                }
            },
            _ => Err("join espera uma lista e uma string".to_string()),
        }
    });

    globals.define_native_function("replace", |args| {
        if args.len() != 3 { return Err("replace espera 3 argumentos".to_string()); }
        match (&args[0], &args[1], &args[2]) {
            (Value::String(s), Value::String(from), Value::String(to)) => {
                Ok(Value::String(s.replace(from.as_str(), to.as_str())))
            },
            _ => Err("replace espera três strings".to_string()),
        }
    });

    globals.define_native_function("contains", |args| {
        if args.len() != 2 { return Err("contains espera 2 argumentos".to_string()); }
        match (&args[0], &args[1]) {
            (Value::String(s), Value::String(substr)) => {
                Ok(Value::Boolean(s.contains(substr.as_str())))
            },
            _ => Err("contains espera duas strings".to_string()),
        }
    });

    globals.define_native_function("starts_with", |args| {
        if args.len() != 2 { return Err("starts_with espera 2 argumentos".to_string()); }
        match (&args[0], &args[1]) {
            (Value::String(s), Value::String(prefix)) => {
                Ok(Value::Boolean(s.starts_with(prefix.as_str())))
            },
            _ => Err("starts_with espera duas strings".to_string()),
        }
    });

    globals.define_native_function("ends_with", |args| {
        if args.len() != 2 { return Err("ends_with espera 2 argumentos".to_string()); }
        match (&args[0], &args[1]) {
            (Value::String(s), Value::String(suffix)) => {
                Ok(Value::Boolean(s.ends_with(suffix.as_str())))
            },
            _ => Err("ends_with espera duas strings".to_string()),
        }
    });

    globals.define_native_function("chars", |args| {
        if args.len() != 1 { return Err("chars espera 1 argumento".to_string()); }
        match &args[0] {
            Value::String(s) => {
                let chars: Vec<Value> = s.chars()
                    .map(|c| Value::String(c.to_string()))
                    .collect();
                Ok(Value::List(chars))
            },
            _ => Err("chars espera uma string".to_string()),
        }
    });

    globals.define_native_function("substring", |args| {
        if args.len() != 3 { return Err("substring espera 3 argumentos".to_string()); }
        match (&args[0], &args[1], &args[2]) {
            (Value::String(s), Value::Number(start), Value::Number(end)) => {
                let start_idx = (*start as usize).min(s.len());
                let end_idx = (*end as usize).min(s.len());
                
                if start_idx > end_idx {
                    return Err("índice inicial maior que índice final".to_string());
                }

                let substr: String = s.chars()
                    .skip(start_idx)
                    .take(end_idx - start_idx)
                    .collect();
                
                Ok(Value::String(substr))
            },
            _ => Err("substring espera uma string e dois números".to_string()),
        }
    });

    globals.define_native_function("format", |args| {
        if args.is_empty() { return Err("format espera pelo menos 1 argumento".to_string()); }
        
        match &args[0] {
            Value::String(template) => {
                let mut result = template.clone();
                
                for (i, arg) in args[1..].iter().enumerate() {
                    let placeholder = format!("{{{}}}", i);
                    let value_str = match arg {
                        Value::Number(n) => n.to_string(),
                        Value::String(s) => s.clone(),
                        Value::Boolean(b) => b.to_string(),
                        Value::List(_) => "[lista]".to_string(),
                        Value::Dict(_) => "{dict}".to_string(),
                        Value::Nil => "nil".to_string(),
                        _ => "?".to_string(),
                    };
                    result = result.replace(&placeholder, &value_str);
                }
                
                Ok(Value::String(result))
            },
            _ => Err("format espera uma string como primeiro argumento".to_string()),
        }
    });
}
