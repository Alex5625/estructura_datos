use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::Path;
use std::u32;
use std::fs::File;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let mut contador_lineas = 0;
    if let Ok(lines) = read_lines("./NOMBRE_DEL_ARCHIVO (CON SU EXTENSION .TXT O .CSV)") {

        for line in lines {

            if let Ok(ip) = line {
                let ip_copy = ip.clone();
                //                 aqui abajo modificar el tipo de split
                let split = ip_copy.split("::");
                //cuenta la columna, como si fuera un arreglo
                let mut contador_columnas:u32 = 0;
                for s in split {
                    //CUERPO DEL EJERCICIO
                }   
            }
            //cuenta en la linea que va, como control de donde va 
            contador_lineas += 1;
        }
    }
}