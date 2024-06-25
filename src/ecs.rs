use std::any::{Any, TypeId};

pub struct Registry {
    registry_map: Vec<(TypeId, Box<dyn Any>)>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            registry_map: Vec::<(TypeId, Box<dyn Any>)>::new(),
        }
    }

    pub fn add_component<Component: 'static>(&mut self, entity_id: i32, component: Component) {
        self.registry_map
            .iter()
            .find_map(|(type_id, compo_vec)| {
                (TypeId::of::<Component>() == *type_id).then_some(compo_vec)
            })
            .is_none()
            .then(|| {
                self.registry_map.push((
                    TypeId::of::<Component>(),
                    Box::<Vec<(i32, Component)>>::default(),
                ));
            });

        let it = self
            .registry_map
            .iter_mut()
            .find_map(|(type_id, compo_vec)| {
                (TypeId::of::<Component>() == *type_id).then_some(compo_vec)
            })
            .unwrap()
            .downcast_mut::<Vec<(i32, Component)>>()
            .unwrap();

        it.iter()
            .find_map(|(entity, compo_vec)| (entity_id == *entity).then_some(compo_vec))
            .is_none()
            .then(|| {
                it.push((entity_id, component));
            });
    }

    pub fn get_component<Component: 'static>(&self, entity_id: i32) -> &Component {
        let it = self
            .registry_map
            .iter()
            .find_map(|(type_id, compo_vec)| {
                (TypeId::of::<Component>() == *type_id).then_some(compo_vec)
            })
            .unwrap()
            .downcast_ref::<Vec<(i32, Component)>>()
            .unwrap();

        it.iter()
            .find_map(|(entity, compo_vec)| (entity_id == *entity).then_some(compo_vec))
            .unwrap()
    }

    pub fn get_component_mut<Component: 'static>(&mut self, entity_id: i32) -> &mut Component {
        let it = self
            .registry_map
            .iter_mut()
            .find_map(|(type_id, compo_vec)| {
                (TypeId::of::<Component>() == *type_id).then_some(compo_vec)
            })
            .unwrap()
            .downcast_mut::<Vec<(i32, Component)>>()
            .unwrap();

        it.iter_mut()
            .find_map(|(entity, compo_vec)| (entity_id == *entity).then_some(compo_vec))
            .unwrap()
    }
}
