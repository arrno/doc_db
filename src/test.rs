use crate::trie::node;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        // Init
        let mut root = node::Node::new(Some("root".to_string()));

        let hello = String::from("Hello");
        let world = String::from("world");

        let path = vec!["a", "b", "c"];
        let path_two = vec!["a", "2", "3", "4"];
        let path_three = vec!["collection"];
        let bad_path = vec!["!"];

        // Insert
        root.insert(&path[..], Some(hello.clone()));
        let found = root.check(&path[..]).unwrap().value.unwrap();
        assert_eq!(*found, hello.clone());

        root.insert(&path_two[..], Some(world.clone()));
        let found = root.check(&path_two[..]).unwrap().value.unwrap();
        assert_eq!(*found, world.clone());

        let found = root.check(&path[..path.len() - 1]).unwrap().value;
        assert_eq!(found, None);

        let found = root.check(&bad_path[..]);
        assert_eq!(found, None);

        // Put / Patch
        root.insert(&path[..], Some("Hola".to_string()));
        let found = root.check(&path[..]).unwrap().value.unwrap();
        assert_eq!(*found, "Hola".to_string());

        let result = root
            .update(&path[..], ", Senora!".to_string().into())
            .unwrap();
        assert_eq!(result, "Hola, Senora!");

        // Query
        root.insert(&vec!["collection", "1"], Some("Alaska".to_string()));
        root.insert(&vec!["collection", "2"], Some("Alabama".to_string()));
        root.insert(&vec!["collection", "3"], Some("Maryland".to_string()));
        root.insert(&vec!["collection", "4"], Some("Arkansas".to_string()));
        root.insert(&vec!["collection", "5"], Some("Virginia".to_string()));
        root.insert(&vec!["collection", "6"], None);
        let filtered: Vec<String> = root
            .query(&path_three, |opt| match opt {
                Some(state) => state.starts_with("Al"),
                None => false,
            })
            .into_iter()
            .map(|x| x.value.unwrap().clone())
            .collect();
        assert_eq!(filtered.len(), 2);
        assert!(filtered.contains(&"Alaska".to_string()));
        assert!(filtered.contains(&"Alabama".to_string()));

        // Delete
        root.delete(&path_three[..]).unwrap();
        let collection = root.find(&path_three[..]);
        assert!(match collection {
            Some(_) => false,
            None => true,
        })
    }
}
