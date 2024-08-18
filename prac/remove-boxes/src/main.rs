use std::collections::HashMap;

fn create_map(boxes: Vec<i32>, mut check: HashMap<i32, (i32, usize, usize)>) {
    // let mut check = std::collections::HashMap::new();
    for (i, color) in boxes.iter().enumerate() {
        match check.get_mut(color) {
            None => {
                check.insert(color.clone(), (1, 0, i));
            }
            Some(v) => {
                let distance = v.1 + (i - v.2 - 1);
                let p = (v.0 + 1, distance, i);
                check.insert(color.clone(), p);
            }
        }
    }
    println!("Check map is {:?}", check);
}

fn remove_boxes(boxes: Vec<i32>) -> i32 {
    // color -> (count, distance, last_index);
    let check = std::collections::HashMap::new();
    let _map = create_map(boxes, check);
    56
}

fn main() {
    println!("Hello, world!");
    assert_eq!(remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]), 56);
}
