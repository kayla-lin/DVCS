pub struct State {
    address: String,
    contents: Vec<String>,
}

pub fn merge_states(ancestor: State, ours: State, theirs: State) -> (Vec<String>, Vec<String>) {
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

//function that returns the elements that are the same between two vectors
fn find_same_diff(a: Vec<String>, b: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut same = Vec::new();
    let mut diff = Vec::new();
    for i in a {
        if b.contains(&i) {
            same.push(i);
        } else {
            diff.push(i);
        }
    }
    return (same, diff);
}

fn find_added(a: Vec<String>, b: Vec<String>) -> Vec<String> {
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
        let ancestor: State = State {
            address: "a".to_string(),
            contents: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
            ],
        };
        let ours: State = State {
            address: "b".to_string(),
            contents: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "e".to_string(),
            ],
        };
        let theirs: State = State {
            address: "c".to_string(),
            contents: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "f".to_string(),
            ],
        };

        assert_eq!(
            merge_states(ancestor, ours, theirs),
            (
                vec!["f".to_string()],
                vec!["a".to_string(), "b".to_string(), "c".to_string()]
            )
        );
    }
}
