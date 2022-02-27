use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Unix Runtime Data", about = "Construction, Management, & Access to Unix Runtime Data from the Shell Runtime")]
struct URDArguments {
    #[structopt(subcommand)]
    subcmd: URDCommand
}

#[derive(Debug, StructOpt)]
enum URDCommand {
    Display,
    Construct(URD::construct::URDConstruct),
    Destroy,
    Select,
}

fn main() {
    let args = URDArguments::from_args();
    match args.subcmd { 
        URDCommand::Construct(urd_type) =>{ URD::construct::construct(urd_type); },
        _ => {}
    }
}
