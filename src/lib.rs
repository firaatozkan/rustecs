mod ecs;

#[cfg(test)]
mod tests {
    use crate::ecs::Registry;

    #[test]
    fn basic_component_addition() {
        let mut registry = Registry::new();

        let player = 1;

        registry.add_component(player, "Initial Value".to_owned());

        assert_eq!("Initial Value", registry.get_component::<String>(player));

        let player_string_mutref = registry.get_component_mut::<String>(player);

        *player_string_mutref = "Final Value".to_owned();

        assert_eq!("Final Value", registry.get_component::<String>(player));
    }
}
