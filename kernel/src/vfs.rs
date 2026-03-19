extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
}

#[derive(Clone)]
pub struct FsNode {
    pub name: String,
    pub node_type: FileType,
    pub content: Vec<u8>,
    pub children: Vec<FsNode>,
    pub size: u64,
}

impl FsNode {
    fn new_directory(name: &str) -> Self {
        Self {
            name: String::from(name),
            node_type: FileType::Directory,
            content: Vec::new(),
            children: Vec::new(),
            size: 0,
        }
    }

    fn new_file(name: &str, content: &[u8]) -> Self {
        Self {
            name: String::from(name),
            node_type: FileType::File,
            content: Vec::from(content),
            children: Vec::new(),
            size: content.len() as u64,
        }
    }
}

pub struct Vfs {
    pub root: FsNode,
}

impl Vfs {
    pub fn new() -> Self {
        Self {
            root: FsNode::new_directory(""),
        }
    }

    pub fn create_file(&mut self, path: &str, content: &[u8]) -> bool {
        let segments = Self::path_segments(path);
        let Some((name, parent_segments)) = segments.split_last() else {
            return false;
        };

        let parent_path = Self::build_path(parent_segments);
        if !parent_segments.is_empty()
            && self.resolve_node(&parent_path).is_none()
            && !self.create_dir(&parent_path)
        {
            return false;
        }

        let Some(parent) = self.resolve_node_mut(&parent_path) else {
            return false;
        };

        if parent.node_type != FileType::Directory {
            return false;
        }

        if let Some(index) = parent.children.iter().position(|child| child.name == *name) {
            if parent.children[index].node_type != FileType::File {
                return false;
            }

            parent.children[index].content = Vec::from(content);
            parent.children[index].size = content.len() as u64;
            return true;
        }

        parent.children.push(FsNode::new_file(name, content));
        true
    }

    pub fn create_dir(&mut self, path: &str) -> bool {
        let segments = Self::path_segments(path);
        if segments.is_empty() {
            return false;
        }

        let mut current = &mut self.root;
        let mut created = false;

        for segment in segments {
            if current.node_type != FileType::Directory {
                return false;
            }

            if let Some(index) = current
                .children
                .iter()
                .position(|child| child.name == segment)
            {
                if current.children[index].node_type != FileType::Directory {
                    return false;
                }
                current = &mut current.children[index];
                continue;
            }

            current.children.push(FsNode::new_directory(segment));
            let last_index = current.children.len() - 1;
            current = &mut current.children[last_index];
            created = true;
        }

        created
    }

    pub fn read_file(&self, path: &str) -> Option<&Vec<u8>> {
        let node = self.resolve_node(path)?;
        if node.node_type != FileType::File {
            return None;
        }
        Some(&node.content)
    }

    pub fn list_dir(&self, path: &str) -> Option<Vec<String>> {
        let node = self.resolve_node(path)?;
        if node.node_type != FileType::Directory {
            return None;
        }

        let mut names = Vec::with_capacity(node.children.len());
        for child in &node.children {
            names.push(child.name.clone());
        }
        Some(names)
    }

    pub fn exists(&self, path: &str) -> bool {
        self.resolve_node(path).is_some()
    }

    pub fn delete(&mut self, path: &str) -> bool {
        let segments = Self::path_segments(path);
        let Some((name, parent_segments)) = segments.split_last() else {
            return false;
        };

        let parent_path = Self::build_path(parent_segments);
        let Some(parent) = self.resolve_node_mut(&parent_path) else {
            return false;
        };

        if parent.node_type != FileType::Directory {
            return false;
        }

        let Some(index) = parent.children.iter().position(|child| child.name == *name) else {
            return false;
        };

        parent.children.remove(index);
        true
    }

    fn resolve_node(&self, path: &str) -> Option<&FsNode> {
        let segments = Self::path_segments(path);
        Self::resolve_node_from(&self.root, &segments)
    }

    fn resolve_node_mut(&mut self, path: &str) -> Option<&mut FsNode> {
        let segments = Self::path_segments(path);
        Self::resolve_node_from_mut(&mut self.root, &segments)
    }

    fn resolve_node_from<'a>(node: &'a FsNode, segments: &[&str]) -> Option<&'a FsNode> {
        if segments.is_empty() {
            return Some(node);
        }

        let child = node
            .children
            .iter()
            .find(|child| child.name == segments[0])?;
        Self::resolve_node_from(child, &segments[1..])
    }

    fn resolve_node_from_mut<'a>(
        node: &'a mut FsNode,
        segments: &[&str],
    ) -> Option<&'a mut FsNode> {
        if segments.is_empty() {
            return Some(node);
        }

        let child = node
            .children
            .iter_mut()
            .find(|child| child.name == segments[0])?;
        Self::resolve_node_from_mut(child, &segments[1..])
    }

    fn path_segments(path: &str) -> Vec<&str> {
        path.split('/')
            .filter(|segment| !segment.is_empty())
            .collect()
    }

    fn build_path(segments: &[&str]) -> String {
        if segments.is_empty() {
            return String::from("/");
        }

        let mut path = String::new();
        for segment in segments {
            path.push('/');
            path.push_str(segment);
        }
        path
    }
}

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}
