struct State {
    address: String,
    contents: Vec<String>
}

fn merge_states(ancestor: State, ours: State, theirs: State){
    //find similar elements between ancestor and ours, theirs
    let tup = find_same_diff(&ancestor.contents, &ours.contents);
    let edited_elements_ours = tup.0;
    let deleted_ours = tup.1;

    let tup = find_same_diff(&ancestor.contents, &theirs.contents);
    let edited_elements_theirs = tup.0;
    let deleted_theirs = tup.1;

    //find elements in diff_ours_ancestor that are not in ancestor but not in ours
    //elements 
    let added_ours = find_added(&ancestor.contents, &ours.contents);
    let added_theirs = find_added(&ancestor.contents, &theirs.contents);

    if deleted_ours != deleted_theirs {
        //conflict
    }


}


//function that returns the elements that are the same between two vectors
fn find_same_diff<'a>(a:&'a Vec<String>, b: &'a Vec<String>) -> (Vec<&'a String>, Vec<&'a String>) {
    let mut same = Vec::new();
    let mut diff = Vec::new();
    for i in a {
        if b.contains(&i) {
            same.push(i);
        }
        else{
            diff.push(i);
        }
    }
    return (same, diff);
}

fn find_added<'a>(a: &'a Vec<String>, b: &'a Vec<String>) -> Vec<&'a String> {
    let mut added = Vec::new();
    for i in b {
        if !a.contains(&i) {
            added.push(i);
        }
    }
    return added;
}


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {

    }
}
