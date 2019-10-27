extern crate actix_files;
extern crate actix_web;
extern crate bincode;
extern crate serde;
extern crate serde_json;

mod graph;

use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use bincode::deserialize_from;
use graph::Graph;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct Way {
    source: usize,
    target: usize,
    weight: usize,
    kind: usize,
}

#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
pub struct Node {
    latitude: f32,
    longitude: f32,
}

#[derive(Deserialize, Debug)]
struct Input {
    ways: Vec<Way>,
    nodes: Vec<Node>,
    offset: Vec<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Query {
    start: Node,
    end: Node,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    path: Vec<Node>,
}

fn query(request: web::Json<Query>, dijkstra: web::Data<Graph>) -> web::Json<Response> {
    // extract points
    let start: &Node = &request.start;
    let end: &Node = &request.end;
    println!("Start: {},{}", start.latitude, start.longitude);
    println!("End: {},{}", end.latitude, end.longitude);
    // search for clicked points
    let start_id: usize = dijkstra.get_point_id(start.latitude, start.longitude);
    let end_id: usize = dijkstra.get_point_id(end.latitude, end.longitude);
    println!("Node IDs: {},{}", start_id, end_id);
    let (path, cost) = dijkstra.find_path(start_id, end_id, 1, false).unwrap();
    // TODO start dijkstra with start
    // TODO start dijkstra with target

    for i in path.iter() {
        print!(" -> {}", *i);
    }
    println!(" :  {}", cost);
    let result: Vec<Node> = dijkstra.get_coordinates(path);
    return web::Json(Response { path: result });
}

fn main() {
    // check if arguments are right
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} pbf.fmi_file", args[0]);
        return;
    }

    // check if file is right
    let filename = std::env::args_os().nth(1).unwrap();
    if !Path::new(&filename).exists() {
        println!("{} not found", filename.into_string().unwrap());
        std::process::exit(1);
    }

    // read file
    let mut f = BufReader::new(File::open(filename).unwrap());
    let input: Input = deserialize_from(&mut f).unwrap();
    let d = Graph::new(input.nodes, input.ways, input.offset);

    let graph = web::Data::new(d);

    // check for static-html folder
    if !Path::new("./static").exists() {
        eprintln!("./static/ directory not found");
        std::process::exit(1);
    }

    // start webserver
    println!("webserver started on http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(1024))
            .register_data(graph.clone())
            .service(web::resource("/dijkstra").route(web::post().to(query)))
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind("localhost:8080")
    .unwrap()
    .run()
    .unwrap();
}
