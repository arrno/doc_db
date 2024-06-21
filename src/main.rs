use crate::tree::node;

mod test;
mod tree;
mod web_handler;

fn main() {
    let root = node::Node::new(Some("root".to_string()));
    root.display();
    web_handler::serve();
}
