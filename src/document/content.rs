use crate::document::object::{Group, GroupElement, Layer};

struct Content {
    content: Vec<GroupElement>,
    name: String,
}

impl Default for Content {
    fn default() -> Self {
        Self {
            content: Vec::new(),
            name: "SceneGroup".to_string(),
        }
    }
}

impl Content {
    fn add(&mut self, element: GroupElement) {
        self.content.push(element);
    }

    fn delete(&mut self, element_id: u16) -> Result<(), String> {
        fn worker(root: &mut Group, del_id: &u16) -> Result<(), String> {
            for index in 0..root.content.len() {
                let value = &mut root.content[index];
                match value {
                    GroupElement::Group(v) => {
                        if v.id == *del_id {
                            root.content.remove(index);
                            return Ok(());
                        } else {
                            match worker(v, del_id) {
                                Ok(_) => return Ok(()),
                                Err(_) => continue,
                            }
                        }
                    }
                    GroupElement::Layer(v) => {
                        if &v.id == del_id {
                            root.content.remove(index);
                            return Ok(());
                        }
                    }
                }
            }
            Err("Not exist".to_string())
        }

        for index in 0..self.content.len() {
            let value = &mut self.content[index];
            match value {
                GroupElement::Group(v) => {
                    if v.id == element_id {
                        self.content.remove(index);
                        return Ok(());
                    } else {
                        match worker(v, &element_id) {
                            Ok(_) => return Ok(()),
                            Err(_) => continue,
                        }
                    }
                }
                GroupElement::Layer(v) => {
                    if v.id == element_id {
                        self.content.remove(index);
                        return Ok(());
                    }
                }
            }
        }
        Err("Id not found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::counter;
    use crate::utils::logs;

    fn delete_by_id() {
        logs::setup_logger().expect("Failed to setup logger");
        let mut scene_collection: Vec<GroupElement> = Vec::new();
        let mut counter = counter::IdCounter::default();
        {
            let el_1: Layer = Layer::new(Vec::new(), counter.next());
            let el_2: Layer = Layer::new(Vec::new(), counter.next());

            let mut el_3: Group = Group::new(Vec::new(), counter.next());

            let el_6: Layer = Layer::new(Vec::new(), counter.next());
            let el_7: Layer = Layer::new(Vec::new(), counter.next());

            el_3.content.push(GroupElement::Layer(el_6));
            el_3.content.push(GroupElement::Layer(el_7));

            let el_4: Layer = Layer::new(Vec::new(), counter.next());
            let el_5: Layer = Layer::new(Vec::new(), counter.next());

            scene_collection.push(GroupElement::Layer(el_1));
            scene_collection.push(GroupElement::Layer(el_2));
            scene_collection.push(GroupElement::Group(el_3));
            scene_collection.push(GroupElement::Layer(el_4));
            scene_collection.push(GroupElement::Layer(el_5));
        }
        log::debug!("scene_collection: {:?}", scene_collection);
    }
}
