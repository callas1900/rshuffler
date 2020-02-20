use structopt::StructOpt;
use rand::seq::SliceRandom;

#[derive(Debug, StructOpt)]
#[structopt(name = "options", about = "member shuffler")]
struct Opt {
    #[structopt(help = "input member name, such as 'ryo bruce alex'")]
    v: Vec<String>,
}
fn main() {
    let mut opt = Opt::from_args();
    let v = &mut opt.v[..];
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);

    for e in v.iter() {
        println!("{}",e);
    }
}
