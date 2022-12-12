use std::fs;

struct State {
    address: String,
    contents: Vec<String>
}

fn merge_states(ancestor: State, ours: State, theirs: State) -> (Vec<String>, Vec<String>){
    //find similar elements between ancestor and ours, theirs

    let tup = find_same_diff(ancestor.contents.clone(), ours.contents.clone());
    let edited_elements_ours = tup.0;
    let deleted_ours = tup.1;

    let tup = find_same_diff(ancestor.contents.clone(), theirs.contents.clone());
    let edited_elements_theirs = tup.0;
    let deleted_theirs = tup.1;

    //find elements in diff_ours_ancestor that are not in ancestor but not in ours
    //elements 
    let added_ours = find_added(ancestor.contents.clone(), ours.contents.clone());
    let added_theirs = find_added(ancestor.contents.clone(), theirs.contents.clone());

    if deleted_ours != deleted_theirs {
        //conflict
    }

    let mut to_be_added = Vec::new();
    for i in added_theirs {
        if !added_ours.contains(&i) {
            to_be_added.push(i);
        }
    }

    let mut to_be_merged = Vec::new();
    for i in edited_elements_ours {
        if edited_elements_theirs.contains(&i) {
            to_be_merged.push(i);
        }
    }

    return (to_be_added, to_be_merged);

}

fn merge_repos(ancestor_path: String, ours_path: String, theirs_path: String){

    let ancestor_contents: Vec<String> = get_contents(&ancestor_path);
    let ours_contents: Vec<String> = get_contents(&ours_path);
    let theirs_contents: Vec<String> = get_contents(&theirs_path);

    let ancestor = State{address: ancestor_path, contents: ancestor_contents};
    let ours = State{address: ours_path, contents: ours_contents};
    let theirs = State{address: theirs_path, contents: theirs_contents};

    let tup = merge_states(ancestor, ours, theirs);
    let to_be_added = tup.0;
    let to_be_merged = tup.1;

    for i in to_be_merged {

    }


}

fn get_contents(path: &String) -> Vec<String> {
    let mut contents: Vec<String> = Vec::new();

    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        contents.push(path.unwrap().path().display().to_string());
    }

    return contents;
}

//function that returns the elements that are the same between two vectors
fn find_same_diff(a: Vec<String>, b:Vec<String>) -> (Vec<String>, Vec<String>) {
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

fn find_added(a: Vec<String>, b:Vec<String>) -> Vec<String> {
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
    use super::*;

    use crate::merge_states;

    #[test]
    fn it_works() {
        let ancestor: State = State{address: "a".to_string(), contents: vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()]};
        let ours: State = State{address: "b".to_string(), contents: vec!["a".to_string(), "b".to_string(), "c".to_string(), "e".to_string()]};
        let theirs: State = State{address: "c".to_string(), contents: vec!["a".to_string(), "b".to_string(), "c".to_string(), "f".to_string()]};

        assert_eq!(merge_states(ancestor, ours, theirs), (vec!["e".to_string()], vec!["f".to_string()]));
    }
}
