/* use std::io;
    use std::io::Write;
    use std::str::FromStr;

    //get user input
    pub fn get_user_input() -> String {
        let mut input = String::new();
        io::stdout().flush().unwrap(); //
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
    //get user input from command line
    fn get_input_function(commands: Vec<String>) -> String {
        let mut input = String::new();
        io::stdout().flush().unwrap(); //
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn apply_method(command: String){
        let mut coms: Vec<&str> = command.split_whitespace().collect();
        let method = coms[0];
        let mut args: Vec<String> = Vec::new();
        for i in 1..coms.len(){
            args.push(coms[i].to_string());
        }
        
        match method {
            'pull' => pull(args[1]),
            'push' => push(args[1]),
            'commit' => commit(args[1]),
            'branch' => branch(args[1]),
            'checkout' => checkout(args[1]),
            'merge' => merge(args[1]),
            'status' => status(),
            'log' => log(),
            'init' => init(),
            'add' => add(args[1]),
            'rm' => rm(args[1]),
            'reset' => reset(args[1]),
            'diff' => diff(args[1]),
            'catfile' => cat_file(args[1]),
            'head' => head(),
            

        }

    } */

pub mod repo_working_hiding{
    
}