use std::collections::HashMap;

fn find_keys_for_value(map: &HashMap<String, i32>, value: i32) -> Vec<&String> {
    map.iter()
        .filter_map(|(key, &val)| if val == value { Some(key) } else { None })
        .collect()
}

pub fn encode(text: String, mut key: i32) -> String {
    let char_map: HashMap<String, i32> = HashMap::from([ ("a".to_string(), 1), ("b".to_string(), 2), ("c".to_string(), 3), ("d".to_string(), 4), ("e".to_string(), 5), ("f".to_string(), 6), ("g".to_string(), 7), ("h".to_string(), 8), ("i".to_string(), 9), ("j".to_string(), 10), ("k".to_string(), 11), ("l".to_string(), 12), ("m".to_string(), 13), ("n".to_string(), 14), ("o".to_string(), 15), ("p".to_string(), 16), ("q".to_string(), 17), ("r".to_string(), 18), ("s".to_string(), 19), ("t".to_string(), 20), ("u".to_string(), 21), ("v".to_string(), 22), ("w".to_string(), 23), ("x".to_string(), 24), ("y".to_string(), 25), ("z".to_string(), 26), (" ".to_string(), 0), ]);

    let mut encoded_text = String::new();
    
  while key > 26{
      key-=26;
    }

    for letter in text.chars() {
        let letter_value = char_map
            .get(&letter.to_string().to_lowercase())
            .expect("Failed to get value")
            + key;
        let normalized_value = if letter_value > 26 {
            letter_value - 26
        } else {
            letter_value
        };

        let letter_key = find_keys_for_value(&char_map, normalized_value);
        encoded_text.push_str(letter_key[0]);
    }

    encoded_text
}
