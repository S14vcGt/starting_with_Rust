// Algoritmo Bosque de Arboles - Sebastian Mata C.I:30.547.594
use std::{
    fs::File,
    io::BufReader,
    io::prelude::*,
    io::Result,
};
fn main() -> Result<()>{
    let mut grafo: Vec<String> = vec![];// estructura que representa el bosque
    let mut vertices = String::new();//conjunto de vertices del grafo
    let mut d = 0;
    let archivo = File::open("tree.in")?;
    let buf_reader = BufReader::new(archivo);
    for linea in buf_reader.lines() {// recorro el archivo linea a linea 
        let n = linea.unwrap();
        d = d+1;
        match d {
            1=>{
                vertices = n.replace(",", "");// saco las comas y tengo mi conjunto de vertices
            }
            _=>{
                let vertice= n.chars();
                let mut tup:Vec<char> = vec![];
                for i in vertice{// agrego a una tupla los vertices
                    if vertices.contains(i){// solo si lo que se lee en entrada pertenece al conjunto de vertices, se agrega
                        tup.push(i)
                    }
                }
                let mut aux = grafo.len();
                if tup.len() == 2{
                    if aux == 0{// si no se ha agregado nada al bosque, se agrega y ya
                        grafo.push(tup[0].to_string()+&tup[1].to_string());
                    }else{
                        let mut repetido =0;
                        let mut agregado = false;
                        let mut i = 0;
                        while i < aux{// empiezo a insertar aristas y unir arboles
                            if grafo[i].contains(tup[0]){//compruebo si el primer vertice de la arista pertenece a algun conjunto, y si no se ha agregado ya, se agrega el 2do vertice
                                if agregado == false{
                                    agregado = true;// si no contiene el segundo vertice, se agrega, y si si lo tiene, significa que ya esta agregado
                                    if grafo[i].contains(tup[1])==false{
                                        grafo[i].push(tup[1]);
                                        repetido = i;// guardamos el indice del vector en el que se agrego la arista
                                    }
                                }else {// si es true, significa que hay otro arbol con este vertice, se unen
                                    let otro= grafo[i].replace(tup[0], "");//saco el vertice que se repite del arbol donde se repite
                                    grafo[repetido].push_str(&otro);// conecto los arboles
                                    grafo.remove(i);// elimino el arbol repetido que ya esta dentro de otro
                                    aux-=1;
                                }
                            }else if grafo[i].contains(tup[1]) {// tambien se pregunta si el segundo vertice pertence a algun conjunto,ya sabiendo que el primero no pertenece por lo menos al consultado
                                if agregado == false{
                                    grafo[i].push(tup[0]);//si es asi, se agrega el primer vertice
                                    repetido = i;
                                    agregado = true;
                                }else {
                                    let otro= grafo[i].replace(tup[1], "");
                                    grafo[repetido].push_str(&otro);
                                    grafo.remove(i);
                                    aux-=1;
                                }
                            }
                            i+=1;
                        }
                        if agregado == false{//si la arista no tiene vertices que pertenezcan a un arbol ya existente,se crea uno
                            grafo.push(tup[0].to_string()+&tup[1].to_string());
                        }
                    }
                }
            }
        }
    }
    let arboles = grafo.len();// el numero de arboles sera el numero de los conjuntos de vertices separados
    let mut bellotas = 0;
    let vertex = vertices.chars();// convierto mi string de vertices en una coleccion de caracteres para poder iterar mas facilmente
    for i in vertex{// me permite cambiar el vertice a comprovar 
        let mut pertenece = false;
        for x in 0..arboles{// me permite cambiar el arbol en el que pregunto si esta un determinado vertice
            if grafo[x].contains(i){// si pertenece el vertice a alguno de los arboles, entonces no es bellota
                pertenece = true;
            }
        }
        if pertenece==false{//si ese vertice no pertenece a ningun arbol, entonces es bellota
            bellotas+=1;
        }
    }
    println!("Hay {} árbol(es) y {} bellota(s)", arboles, bellotas);
    Ok(())
}