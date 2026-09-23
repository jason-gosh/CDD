//checkear que hace este codigo
/*
const COMMAND_CD: &str = "cd";
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Uso: cdi <ruta_o_archivo>");
        return;
    }

    let destino = &args[1];

    // 1. Obtener la ruta absoluta base
    let mut path_final = match env::current_dir() {
        Ok(ruta) => ruta.join(destino),
        Err(e) => {
            eprintln!("¡No se pudo leer el path actual! -> {}", e);
            return;
        }
    };

    println!("[IN] Evaluando: {:?}", path_final);

    // 2. Lógica de "Búsqueda del Padre":
// Si la ruta no existe o no es un directorio, intentamos subir niveles
    while !path_final.exists() || !path_final.is_dir() {
        if let Some(padre) = path_final.parent() {
            path_final = padre.to_path_buf();
        } else {
            // Si llegamos a la raíz y nada es válido, abortamos
            eprintln!("Error: No se encontró ninguna carpeta válida en la ruta proporcionada.");
            return;
        }
    }

    // 3. Escritura del comando
    let mut file_sh = File::create("/tmp/cdi_path.sh").expect("No se pudo crear archivo temporal");

    // Usamos Raw Strings para mayor limpieza estética
    writeln!(file_sh, r#"{COMMAND_CD} "{}""#, path_final.display())
        .expect("No se pudo escribir en el archivo .sh");

    println!("[OUT] Cambiando a carpeta válida: {}", path_final.display());
     */