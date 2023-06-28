export const initWorld = `fn default() -> Self {
        Self {
            id: WorldId::new().expect("More \`bevy\` \`World\`s have been created than is supported"),
            entities: Entities::new(),
            components: Default::default(),
            archetypes: Archetypes::new(),
            storages: Default::default(),
            bundles: Default::default(),
            removed_components: Default::default(),
            archetype_component_access: Default::default(),
            // Default value is \`1\`, and \`last_change_tick\`s default to \`0\`, such that changes
            // are detected on first system runs and for direct world queries.
            change_tick: AtomicU32::new(1),
            last_change_tick: 0,
            last_check_tick: 0,
        }
}`
export const initWorldResources = `pub fn init_resource<R: Resource + FromWorld>(&mut self) -> ComponentId {
        let component_id = self.components.init_resource::<R>();
        if self
            .storages
            .resources
            .get(component_id)
            .map_or(true, |data| !data.is_present())
        {
            let value = R::from_world(self);
            OwningPtr::make(value, |ptr| {
                // SAFETY: component_id was just initialized and corresponds to resource of type R.
                unsafe {
                    self.insert_resource_by_id(component_id, ptr);
                }
            });
        }
        component_id
    }`
