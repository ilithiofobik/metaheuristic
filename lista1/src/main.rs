mod tsp;
use tsp::gen;
use tsp::geo;
use tsp::io;

fn main() -> std::io::Result<()> {
    let coords = gen::create_tsp(100);
    for (_key, _value) in &coords {
        //println!("{}: {:?}", _key, _value);
    }

    let _p1 = geo::Point { x: 0.0, y: 0.0 };
    let _p2 = geo::Point { x: 1.0, y: 1.0 };
    //println!("{}", _p1.distance(&_p2));

    return io::read_tsp("gr666.tsp");
}
