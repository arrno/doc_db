use crate::trie::document::Document;
use crate::trie::util::NodeError;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Serialize, Clone)]
pub struct Node<T>
where
    T: Document,
{
    children: HashMap<String, Node<T>>,
    value: Option<T>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct NodeInfo<'a, T>
where
    T: Document,
{
    pub label: String,
    pub value: Option<&'a T>,
}

impl<T: Document> Node<T> {
    pub fn new(value: Option<T>) -> Self {
        Node {
            value: value,
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, path: &[&str], val: Option<T>) {
        if path.len() == 0 {
            return;
        }
        let child = self
            .children
            .entry(path[0].to_string())
            .or_insert(Self::new(None));
        if path.len() == 1 {
            child.value = val;
        } else {
            Self::insert(child, &path[1..], val);
        }
    }

    pub fn find(&self, path: &[&str]) -> Option<&Self> {
        if path.len() == 0 {
            return None;
        }
        let child = self.children.get(path[0]);
        if path.len() == 1 {
            return child;
        }
        match child {
            Some(node) => Self::find(node, &path[1..]),
            None => None,
        }
    }

    pub fn find_mut(&mut self, path: &[&str]) -> Option<&mut Self> {
        if path.len() == 0 {
            return None;
        }
        let child = self.children.get_mut(path[0]);
        if path.len() == 1 {
            return child;
        }
        match child {
            Some(node) => Self::find_mut(node, &path[1..]),
            None => None,
        }
    }

    pub fn check(&self, path: &[&str]) -> Option<NodeInfo<T>> {
        if path.len() == 0 {
            return None;
        }
        if let Some(node) = self.children.get(path[0]) {
            if path.len() == 1 {
                return Some(NodeInfo {
                    label: path[0].to_string(),
                    value: match &node.value {
                        Some(x) => Some(x),
                        None => None,
                    },
                });
            }
            return Self::check(node, &path[1..]);
        }
        None
    }

    pub fn delete(&mut self, path: &[&str]) -> Result<(), Box<dyn Error>> {
        if path.len() == 0 {
            return Err(NodeError::NotFound.into());
        } else if path.len() == 1 {
            return match self.children.remove(path[0]) {
                Some(_) => Ok(()),
                None => Err(NodeError::NotFound.into()),
            };
        }
        match self.children.get_mut(path[0]) {
            Some(node) => node.delete(&path[1..]),
            None => Err(NodeError::NotFound.into()),
        }
    }

    pub fn query(&self, path: &[&str], filter: impl Fn(&Option<T>) -> bool) -> Vec<NodeInfo<T>> {
        let parent = Self::find(&self, path);
        let mut results: Vec<NodeInfo<T>> = Vec::new();
        if let Some(node) = parent {
            node.children.iter().for_each(|(k, v)| {
                if filter(&v.value) {
                    results.push(NodeInfo {
                        label: k.to_string(),
                        value: match &v.value {
                            Some(x) => Some(x),
                            None => None,
                        },
                    })
                }
            });
        }
        results
    }

    pub fn update(
        &mut self,
        path: &[&str],
        update: <T as Document>::U,
    ) -> Result<&T, Box<dyn Error>> {
        let child = match self.find_mut(path) {
            Some(val) => val,
            None => return Err(NodeError::NotFound.into()),
        };
        if let Some(val) = &mut child.value {
            return val.update(update);
        }
        Err(NodeError::IsNull.into())
    }
}

impl<T> Node<T>
where
    T: Serialize + Document,
{
    pub fn display(&self) {
        println!("{}", serde_json::to_string_pretty(&self).unwrap());
    }
}
