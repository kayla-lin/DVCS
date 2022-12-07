mod user_interaction;
mod user_feedback;
fn main(){
    user_interaction::pull();
    

    let mut errors = Vec::new();
    for i in 0..10 {
        errors.push(format!("Error {}", i));
    }

    //user_feedback::format_error_alt(errors);


    
    
}