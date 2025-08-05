use serde::{Deserialize, Serialize}; // Para serializar/deserializar datos en formato JSON
use std::fs::File;                   // Para abrir y escribir archivos
use std::io::{self, BufReader};      // Para entrada/salida y lectura eficiente
use std::path::Path;                 // Para verificar si un archivo existe
use std::collections::BTreeMap;      // Para ordenar contactos por nombre

// Estructura que representa a un contacto
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Contacto {
    nombre: String,
    cumple: String,   // Fecha en formato "dd/mm"
    telefono: String,
    correo: String,
}

// Carga la lista de contactos desde un archivo JSON si existe, o retorna una lista vacía
fn cargar_contactos() -> Vec<Contacto> {
    if !Path::new("contactos.json").exists() {
        return vec![]; // Si el archivo no existe, se retorna una lista vacía
    }
    let file = File::open("contactos.json").unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| vec![]) // Se deserializa el contenido
}

// Guarda la lista de contactos en un archivo JSON con formato legible
fn guardar_contactos(contactos: &[Contacto]) {
    let file = File::create("contactos.json").unwrap();
    serde_json::to_writer_pretty(file, &contactos).unwrap();
}

// Solicita al usuario un número y valida que sea entero
fn pedir_numero(mensaje: &str) -> u32 {
    loop {
        let mut entrada = String::new();
        println!("{}", mensaje);
        io::stdin().read_line(&mut entrada).unwrap();
        if let Ok(num) = entrada.trim().parse::<u32>() {
            return num;
        } else {
            println!("Entrada inválida. Intenta de nuevo.");
        }
    }
}

// Verifica si el día ingresado es válido para el mes correspondiente
fn validar_dia(dia: u32, mes: u32) -> bool {
    match mes {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => dia >= 1 && dia <= 31,
        4 | 6 | 9 | 11 => dia >= 1 && dia <= 30,
        2 => dia >= 1 && dia <= 29, // Aceptamos hasta 29 días en febrero
        _ => false,
    }
}

// Función para agregar un nuevo contacto a la lista
fn agregar_contacto(contactos: &mut Vec<Contacto>) {
    let mut entrada = String::new();

    println!("Nombre:");
    io::stdin().read_line(&mut entrada).unwrap();
    let nombre = entrada.trim().to_string();
    entrada.clear();

    // Solicita y valida el mes del cumpleaños
    let mes = loop {
        let m = pedir_numero("Mes de cumpleaños (1-12):");
        if m >= 1 && m <= 12 {
            break m;
        } else {
            println!("Mes inválido. Intenta de nuevo.");
        }
    };

    // Solicita y valida el día del cumpleaños
    let dia = loop {
        let d = pedir_numero("Día del cumpleaños:");
        if validar_dia(d, mes) {
            break d;
        } else {
            println!("Día inválido para el mes seleccionado. Intenta de nuevo.");
        }
    };

    // Formatea la fecha como "dd/mm"
    let cumple = format!("{:02}/{:02}", dia, mes);

    println!("Teléfono:");
    io::stdin().read_line(&mut entrada).unwrap();
    let telefono = entrada.trim().to_string();
    entrada.clear();

    println!("Correo:");
    io::stdin().read_line(&mut entrada).unwrap();
    let correo = entrada.trim().to_string();

    // Se crea un nuevo contacto y se agrega a la lista en memoria
    contactos.push(Contacto {
        nombre,
        cumple,
        telefono,
        correo,
    });

    // Se guarda la lista completa en el archivo
    guardar_contactos(contactos);
    println!("Contacto agregado y guardado.");
}

// Busca un contacto por nombre (sin importar mayúsculas o minúsculas)
fn buscar_contacto(contactos: &[Contacto], nombre: &str) {
    for c in contactos {
        if c.nombre.eq_ignore_ascii_case(nombre) {
            println!("{:?}", c);
            return;
        }
    }
    println!("No se encontró el contacto.");
}

// Lista todos los contactos que cumplen años en un mes específico
fn listar_por_mes(contactos: &[Contacto], mes: &str) {
    let mes_num: u32 = match mes.trim().parse() {
        Ok(m) if m >= 1 && m <= 12 => m,
        _ => {
            println!("Mes inválido.");
            return;
        }
    };

    let mes_formateado = format!("{:02}", mes_num);
    let mut encontrados = false;

    // Filtramos y mostramos los contactos cuyo cumpleaños coincidan con el mes ingresado
    for c in contactos {
        if let Some(m) = c.cumple.split('/').nth(1) {
            if m == mes_formateado {
                println!("{:?}", c);
                encontrados = true;
            }
        }
    }

    if !encontrados {
        println!("No hay contactos con cumpleaños en ese mes.");
    }
}

// Lista todos los contactos ordenados alfabéticamente por su nombre
fn listar_ordenados(contactos: &[Contacto]) {
    let mut mapa = BTreeMap::new(); // Mapa ordenado por nombre
    for c in contactos {
        mapa.insert(c.nombre.clone(), c.clone());
    }
    for (_, contacto) in mapa {
        println!("{:?}", contacto);
    }
}

// Elimina un contacto de la lista por nombre (las mayúsculas no importan)
fn eliminar_contacto(contactos: &mut Vec<Contacto>) {
    let mut entrada = String::new();
    println!("Nombre del contacto a eliminar:");
    io::stdin().read_line(&mut entrada).unwrap();
    let nombre = entrada.trim().to_lowercase();

    let original_len = contactos.len();
    // Se retienen solamente los contactos cuyo nombre no coincida
    contactos.retain(|c| c.nombre.to_lowercase() != nombre);

    if contactos.len() < original_len {
        guardar_contactos(contactos);
        println!("Contacto eliminado correctamente.");
    } else {
        println!("No se encontró ningún contacto con ese nombre.");
    }
}


fn main() {
    let mut lista_contactos = cargar_contactos(); // Lista en memoria cargada desde el archivo

    loop {
        println!("\nMenú:");
        println!("1) Agregar contacto");
        println!("2) Consultar contacto por nombre");
        println!("3) Listar cumpleaños por mes");
        println!("4) Listar todos los contactos ordenados");
        println!("5) Eliminar contacto");
        println!("6) Salir");

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).unwrap();

        match opcion.trim() {
            "1" => agregar_contacto(&mut lista_contactos),
            "2" => {
                let mut nombre = String::new();
                println!("Nombre del contacto:");
                io::stdin().read_line(&mut nombre).unwrap();
                buscar_contacto(&lista_contactos, nombre.trim());
            }
            "3" => {
                let mut mes = String::new();
                println!("Número del mes (1-12):");
                io::stdin().read_line(&mut mes).unwrap();
                listar_por_mes(&lista_contactos, mes.trim());
            }
            "4" => listar_ordenados(&lista_contactos),
            "5" => eliminar_contacto(&mut lista_contactos),
            "6" => {
                guardar_contactos(&lista_contactos);
                println!("Contactos guardados correctamente. Saliendo...");
                break;
            }
            _ => println!("Opción no válida"),
        }
    }
}
