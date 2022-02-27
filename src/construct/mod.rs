use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct URDConstruct {
    #[
        structopt(
            short = "t", long = "type", 
            parse(from_str), name = "URD Type", 
            about = "The type of data-structure", 
            help = "\
                Choose a data-structure (the default is a dynamic-array)\
            ")
    ]
    urd_type: String
}

pub fn construct(urd_type: URDConstruct) { 
    println!("{:?}", urd_type); 
}
