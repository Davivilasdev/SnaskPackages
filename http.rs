use crate::value::Value;
use crate::symbol_table::SymbolTable;
use std::collections::HashMap;

/// Registra funções de HTTP na stdlib
pub fn register(globals: &mut SymbolTable) {
    globals.define_native_function("http_get", |args| {
        if args.len() != 1 { return Err("http_get espera 1 argumento".to_string()); }
        
        match &args[0] {
            Value::String(url) => {
                #[cfg(feature = "http")]
                {
                    match reqwest::blocking::get(url) {
                        Ok(response) => {
                            let mut result = HashMap::new();
                            
                            // Status code
                            result.insert(
                                Value::String("status".to_string()),
                                Value::Number(response.status().as_u16() as f64)
                            );

                            // Body
                            match response.text() {
                                Ok(body) => {
                                    result.insert(
                                        Value::String("body".to_string()),
                                        Value::String(body)
                                    );
                                },
                                Err(e) => {
                                    return Err(format!("Erro ao ler resposta: {}", e));
                                }
                            }

                            Ok(Value::Dict(result))
                        },
                        Err(e) => Err(format!("Erro na requisição HTTP: {}", e)),
                    }
                }
                
                #[cfg(not(feature = "http"))]
                {
                    Err("HTTP não está habilitado nesta build".to_string())
                }
            },
            _ => Err("http_get espera uma string (URL)".to_string()),
        }
    });

    globals.define_native_function("http_post", |args| {
        if args.len() != 2 { return Err("http_post espera 2 argumentos".to_string()); }
        
        match (&args[0], &args[1]) {
            (Value::String(url), Value::String(body)) => {
                #[cfg(feature = "http")]
                {
                    let client = reqwest::blocking::Client::new();
                    match client.post(url).body(body.clone()).send() {
                        Ok(response) => {
                            let mut result = HashMap::new();
                            
                            result.insert(
                                Value::String("status".to_string()),
                                Value::Number(response.status().as_u16() as f64)
                            );

                            match response.text() {
                                Ok(response_body) => {
                                    result.insert(
                                        Value::String("body".to_string()),
                                        Value::String(response_body)
                                    );
                                },
                                Err(e) => {
                                    return Err(format!("Erro ao ler resposta: {}", e));
                                }
                            }

                            Ok(Value::Dict(result))
                        },
                        Err(e) => Err(format!("Erro na requisição HTTP: {}", e)),
                    }
                }
                
                #[cfg(not(feature = "http"))]
                {
                    Err("HTTP não está habilitado nesta build".to_string())
                }
            },
            _ => Err("http_post espera duas strings (URL e body)".to_string()),
        }
    });
}
