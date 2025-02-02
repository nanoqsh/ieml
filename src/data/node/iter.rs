use super::super::cell::Data;
use super::BasicNode;
use std::fmt::{Debug, Formatter};

pub struct BasicListIter<'a> {
    data: &'a Data,
    iter: std::slice::Iter<'a, usize>,
}

impl<'a> BasicListIter<'a> {
    pub fn new(data: &'a Data, iter: std::slice::Iter<'a, usize>) -> Self {
        Self { data, iter }
    }
}

impl<'a> Clone for BasicListIter<'a> {
    fn clone(&self) -> Self {
        Self {
            data: self.data,
            iter: self.iter.clone(),
        }
    }
}

impl<'a> Debug for BasicListIter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{current: {:?}}}", self.clone().next())
    }
}

impl<'a> Iterator for BasicListIter<'a> {
    type Item = BasicNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|i| BasicNode::new(self.data.get(*i), self.data))
    }
}

pub struct BasicMapIter<'a> {
    data: &'a Data,
    iter: std::collections::hash_map::Iter<'a, String, usize>,
}

impl<'a> BasicMapIter<'a> {
    pub fn new(data: &'a Data, iter: std::collections::hash_map::Iter<'a, String, usize>) -> Self {
        Self { data, iter }
    }
}

impl<'a> Clone for BasicMapIter<'a> {
    fn clone(&self) -> Self {
        Self {
            data: self.data,
            iter: self.iter.clone(),
        }
    }
}

impl<'a> Debug for BasicMapIter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{current: {:?}}}", self.clone().next())
    }
}

impl<'a> Iterator for BasicMapIter<'a> {
    type Item = (&'a String, BasicNode<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|i| (i.0, BasicNode::new(self.data.get(*i.1), self.data)))
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::{
        cell::{DataCell, MarkedDataCell, TagCell},
        node_type::NodeType,
    };
    use super::super::Node;
    use super::*;
    use std::collections::HashMap;

    type ListIter<'a> = BasicListIter<'a>;
    type MapIter<'a> = BasicMapIter<'a>;

    fn test_data() -> Data {
        Data::new(
            4,
            [
                (
                    0,
                    MarkedDataCell {
                        cell: DataCell::Null,
                        mark: Default::default(),
                    },
                ),
                (
                    1,
                    MarkedDataCell {
                        cell: DataCell::Null,
                        mark: Default::default(),
                    },
                ),
                (
                    2,
                    MarkedDataCell {
                        cell: DataCell::String("hello".into()),
                        mark: Default::default(),
                    },
                ),
                (
                    3,
                    MarkedDataCell {
                        cell: DataCell::Raw("hello".into()),
                        mark: Default::default(),
                    },
                ),
                (
                    4,
                    MarkedDataCell {
                        cell: DataCell::Tag(TagCell {
                            cell_index: 0,
                            tag: "tag".into(),
                        }),
                        mark: Default::default(),
                    },
                ),
            ],
        )
    }

    #[test]
    fn test_list_iter_next() {
        let data = test_data();
        let list = vec![2_usize, 3];
        let mut list_iter = ListIter::new(&data, list.iter());

        let first = list_iter.next().unwrap();
        assert_eq!(first.node_type(), NodeType::String);
        assert_eq!(first.string().unwrap(), "hello".to_string());

        let second = list_iter.next().unwrap();
        assert_eq!(second.node_type(), NodeType::Raw);
        assert_eq!(second.raw().unwrap(), "hello");

        assert!(list_iter.next().is_none());
    }

    #[test]
    fn test_map_iter_next() {
        let data = test_data();
        let map = HashMap::<String, usize>::from([("first".into(), 1), ("second".into(), 4)]);
        let map_iter = MapIter::new(&data, map.iter());
        let mut collected_map = map_iter.collect::<Vec<(&String, Node)>>();
        collected_map.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(collected_map.len(), 2);

        let first = &collected_map[0];
        assert_eq!(*first.0, "first");
        assert_eq!(first.1.node_type(), NodeType::Null);

        let second = &collected_map[1];
        assert_eq!(*second.0, "second");
        assert_eq!(second.1.node_type(), NodeType::Tag);
        assert_eq!(second.1.tag().unwrap(), "tag");
    }
}
