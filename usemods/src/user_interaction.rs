extern crate clap; 
use clap::Parser; 
#[derive(Parser, Debug)]

pub struct Repository {
        #[clap(short, long)]   
        repo: String,

}
    
pub fn pull(){
        
        let arguments: Repository = Repository::parse();
        println!("pulling from => {:?}", arguments);
        

} 