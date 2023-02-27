// Transformation operations
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Ops {
    Insert(String), // insert with what string to insert
    Delete(usize),  // delete with how many characters to delete
    Skip(usize),    // skip with how many characters to skip
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Transformation {
    ops: Vec<Ops>, // each transformation allows a few operations
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Doc {
    text: String,  // the text of the document
    cursor: usize, // the cursor position
}

// check if transformation on Doc is valid
// take old doc, new doc, and transformation
// return true if valid, false if not
// each time an operation is verified, stale will be updated to
// reflect the state of the document with that operation applied
#[allow(dead_code)]
fn is_valid(stale: &mut Doc, latest: &Doc, trans: &Transformation) -> bool {
    for t in trans.ops.iter() {
        match t {
            Ops::Insert(s) => {
                // make sure after inserting text, stale text matches
                let mut stale_clone = stale.text.clone();
                stale_clone.insert_str(stale.cursor, s);
                // update stale for the next operation
                stale.text = stale_clone;
                stale.cursor += s.len();
            }
            Ops::Skip(count) => {
                // make sure cursor is not past the end
                if stale.cursor + count > stale.text.len() {
                    return false;
                }

                // update stale for the next operation
                stale.cursor += count;
            }
            Ops::Delete(count) => {
                // make sure text can be deleted within the length of stale
                if stale.cursor + count > stale.text.len() {
                    return false;
                }

                // make sure after deleting from stale, text matches in both docs
                // if it does, update stale for the next operation
                let mut stale_clone = stale.text.clone();
                stale_clone.replace_range(stale.cursor..stale.cursor + count, "");
                stale.text = stale_clone;
            }
        }
    }

    // if all operations are valid, check stale against latest
    stale.text == latest.text && stale.cursor == latest.cursor
}

#[test]
fn test_is_valid() {
    let tests = vec![
        (
            Doc {
                text: "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.".to_string(),
                cursor: 0,
            }, Doc {
                text: "Repl.it uses operational transformations.".to_string(),
                cursor: 40,
            }, 
            vec![
                Ops::Skip(40),
                Ops::Delete(47),
            ], 
            true,
        ),
        (
            Doc {
                text: "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.".to_string(),
                cursor: 0,
            }, Doc {
                text: "Repl.it uses operational transformations.".to_string(),
                cursor: 40,
            }, 
            vec![
                Ops::Skip(45),
                Ops::Delete(47),
            ],
            false,
        ),
        (
            Doc {
                text: "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.".to_string(),
                cursor: 0,
            }, Doc {
                text: "Repl.it uses operational transformations.".to_string(),
                cursor: 40,
            }, 
            vec![
                Ops::Skip(40),
                Ops::Delete(47),
                Ops::Skip(2),
            ],
            false
        ),
        (
            Doc {
                text: "".to_string(),
                cursor: 0,
            }, Doc {
                text: "Howdy".to_string(),
                cursor: 5,
            }, 
            vec![
                Ops::Insert("Howdy".to_string()),
            ],
            true
        ),
        (
            Doc {
                text: "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.".to_string(),
                cursor: 0,
            }, Doc {
                text: "We use operational transformations to keep everyone in a multiplayer repl in sync.".to_string(),
                cursor: 6,
            }, 
            vec![
                Ops::Delete(7),
                Ops::Insert("We".to_string()),
                Ops::Skip(4),
                Ops::Delete(1),
            ],
            true
        ),
    ];

    // run tests
    let mut num = 1;
    for test in tests {
        assert_eq!(is_valid(&mut test.0.clone(), &test.1, &Transformation { ops: test.2 }), test.3, "Test failed: {num}");
        num += 1;
    }    
}

fn main() {
}
