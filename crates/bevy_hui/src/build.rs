use crate::{
    animation::{ActiveAnimation, AnimationDirection}, data::{AttrTokens, Attribute, HtmlTemplate, NodeType, XNode}, prelude::ComponentBindings, styles::{HoverTimer, HtmlStyle, PressedTimer}, util::SlotId
};
use bevy::{prelude::*, utils::HashMap};
use nom::{
    bytes::complete::{is_not, tag, take_until}, character::complete::multispace0, multi::many0, sequence::{delimited, preceded, tuple}, IResult
};
use std::time::Duration;

/// holds a parsed template
/// can be build as UI.
pub struct BuildPlugin;
impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (hotreload, spawn_ui).chain())
            .register_type::<TemplatePropertySubscriber>()
            .register_type::<TemplateExpresions>()
            .register_type::<TemplateProperties>()
            .register_type::<TemplateScope>()
            .register_type::<Tags>()
            .register_type::<OnUiExit>()
            .register_type::<OnUiEnter>()
            .register_type::<OnUiPress>()
            .register_type::<OnUiSpawn>()
            .register_type::<OnUiChange>()
            .register_type::<UiTarget>()
            .register_type::<UiId>()
            .register_type::<SlotPlaceholder>()
            .register_type::<UnslotedChildren>()
            .register_type::<HtmlNode>()
            .register_type::<super::data::XNode>()
            .register_type::<super::data::HtmlTemplate>()
            .register_type::<super::data::StyleAttr>()
            .register_type::<super::data::Action>();
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct TemplateContent(pub String);

/// Holds the reference to the template root entity,
/// which owns the template properties
#[derive(Component, Clone, Deref, Debug, DerefMut, Copy, Reflect)]
#[reflect]
pub struct TemplateScope(Entity);

/// The property definition of a template,
/// this component can be found on the template root
/// entity, use `TemplateScope` (exists on all nodes, part of a template)
/// to get access to the root.
#[derive(Component, Debug, Clone, Default, Reflect, Deref, DerefMut)]
#[reflect]
pub struct TemplateProperties(HashMap<String, String>);

impl TemplateProperties {
    pub fn with(mut self, key: &str, value: &str) -> Self {
        self.insert(key.to_string(), value.to_string());
        self
    }
}

impl From<HashMap<String, String>> for TemplateProperties {
    fn from(map: HashMap<String, String>) -> Self {
        TemplateProperties(map) // No clone or copy here, direct move
    }
}

/// Entites that need to be notified, when the
/// template properties change.
#[derive(Component, Clone, Default, Debug, Deref, DerefMut, Reflect)]
#[reflect]
pub struct TemplatePropertySubscriber(pub Vec<Entity>);

#[derive(Component)]
pub struct InsideSlot {
    owner: Entity,
}

#[derive(Component, Reflect, Debug)]
#[reflect]
pub struct SlotPlaceholder {
    owner: Entity,
}

/// ref to unresolved nodes that
/// need to move to the `<slot/>`
/// when the template is loaded.
#[derive(Component, Reflect, Debug)]
#[reflect]
pub struct UnslotedChildren(Entity);

/// entities subscribed to the owners interaction
/// component
#[derive(Component, DerefMut, Debug, Deref)]
pub struct InteractionObverser(Vec<Entity>);

/// unresolved expresssions that can be compiled
/// to a solid attribute
#[derive(Component, Reflect, Deref, Debug, DerefMut)]
#[reflect]
pub struct TemplateExpresions(Vec<AttrTokens>);

/// Any attribute prefixed with `tag:my_tag="my_value"`
/// will be availble here.
#[derive(Component, Deref, DerefMut, Debug, Default, Reflect)]
#[reflect]
pub struct Tags(HashMap<String, String>);

/// holds ref to the raw uncompiled text content
#[derive(Component, Deref, DerefMut)]
pub struct ContentId(SlotId);

/// the entities owned uid hashed as u64
#[derive(Component, Debug, Default, Hash, Deref, DerefMut, Reflect)]
#[reflect]
pub struct UiId(String);

/// the entity behind `id` in `target="id"`
#[derive(Component, Debug, DerefMut, Deref, Reflect)]
#[reflect]
pub struct UiTarget(pub Entity);

/// watch interaction of another entity
#[derive(Component, Debug, DerefMut, Deref, Reflect)]
#[reflect]
pub struct UiWatch(pub Entity);

#[derive(Component, Default)]
pub struct FullyBuild;

/// Eventlistener interaction transition to Hover
#[derive(Component, Debug, Deref, DerefMut, Reflect)]
#[reflect]
pub struct OnUiPress(pub Vec<String>);

/// Eventlistener on spawning node
#[derive(Component, Debug, DerefMut, Deref, Reflect)]
#[reflect]
pub struct OnUiSpawn(pub Vec<String>);

/// Eventlistener for interaction transition to Hover
#[derive(Component, Debug, DerefMut, Deref, Reflect)]
#[reflect]
pub struct OnUiEnter(pub Vec<String>);

/// Eventlistener for interaction transition to None
#[derive(Component, Debug, Deref, DerefMut, Reflect)]
#[reflect]
pub struct OnUiExit(pub Vec<String>);

/// Eventlistener for a user triggered Change Event
/// This can be when building a widgets
#[derive(Component, Debug, Deref, DerefMut, Reflect)]
#[reflect]
pub struct OnUiChange(pub Vec<String>);

/// Html Ui Node
/// pass it a handle, it will spawn an UI.
#[derive(Component, Debug, Default, Deref, DerefMut, Reflect)]
#[require(Node, TemplateProperties)]
#[reflect]
pub struct HtmlNode(pub Handle<HtmlTemplate>);

fn hotreload(
    mut cmd: Commands,
    mut events: EventReader<AssetEvent<HtmlTemplate>>,
    templates: Query<(Entity, &HtmlNode)>,
    sloted_nodes: Query<(Entity, &InsideSlot)>,
) {
    events.read().for_each(|ev| {
        let id = match ev {
            AssetEvent::Modified { id } => id,
            _ => {
                return;
            }
        };

        templates
            .iter()
            .filter(|(_, html)| html.id() == *id)
            .for_each(|(entity, _)| {
                let slots = sloted_nodes
                    .iter()
                    .flat_map(|(slot_entity, slot)| (slot.owner == entity).then_some(slot_entity))
                    .collect::<Vec<_>>();

                if slots.len() > 0 {
                    let slot_holder = cmd.spawn_empty().add_children(&slots).id();
                    cmd.entity(entity).insert(UnslotedChildren(slot_holder));
                }

                cmd.entity(entity)
                    .despawn_descendants()
                    .retain::<KeepComps>();
            });
    });
}

#[derive(Bundle)]
struct KeepComps {
    pub parent: Parent,
    pub children: Children,
    pub ui: HtmlNode,
    pub unsloed: UnslotedChildren,
    pub slot: SlotPlaceholder,
    pub inside: InsideSlot,
    pub scope: TemplateScope,
}

fn spawn_ui(
    mut cmd: Commands,
    mut unbuild: Query<(Entity, &HtmlNode, &mut TemplateProperties), Without<FullyBuild>>,
    assets: Res<Assets<HtmlTemplate>>,
    server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    custom_comps: Res<ComponentBindings>,
) {
    unbuild
        .iter_mut()
        .for_each(|(root_entity, handle, mut properties)| {
            let Some(template) = assets.get(&**handle) else {
                return;
            };

            template.properties.iter().for_each(|(key, val)| {
                _ = properties.try_insert(key.to_owned(), val.clone());
            });

            let mut builder = TemplateBuilder::new(
                cmd.reborrow(),
                &assets,
                &server,
                &mut texture_atlases,
                &custom_comps,
            );

            if let Some(node) = template.root.first() {
                let mut state = TemplateBuilderState::new(root_entity, template, &node.children, &properties);

                builder.build_tree(node, &mut state);
            } else {
                warn!("template has no root node!");
            }

            if template.root.len() > 1 {
                warn!("templates currently only support one root node, ignoring the rest");
            }
        });
}

fn calculate_starting_frame(start: usize, end: usize, direction: &AnimationDirection) -> usize {
    match direction {
        AnimationDirection::Forward => start,
        AnimationDirection::Reverse => end,
        AnimationDirection::AlternateForward => start,
        AnimationDirection::AlternateReverse => end,
    }
}

fn build_animation(style: &HtmlStyle) -> Option<ActiveAnimation> {
    if style.computed.atlas.is_none() {
        return None
    }

    let starting_frame = if !style.computed.frames.is_empty() {
        calculate_starting_frame(style.computed.frames[0] as usize, style.computed.frames[style.computed.frames.len() - 1] as usize, &style.computed.direction)
    } else {
        let atlas = style.computed.atlas.as_ref().unwrap();

        calculate_starting_frame(0, (atlas.rows * atlas.columns) as usize - 1, &style.computed.direction)
    };

    let starting_direction = match style.computed.direction {
        AnimationDirection::AlternateForward => AnimationDirection::Forward,
        AnimationDirection::AlternateReverse => AnimationDirection::Reverse,
        _ => style.computed.direction.clone(),
    };

    Some(ActiveAnimation {
        timer: Timer::new(Duration::from_secs_f32(1.0 / style.computed.fps as f32), TimerMode::Repeating),
        direction: starting_direction,
        frame: starting_frame,
        iterations: style.computed.iterations,
        duration: style.computed.duration / 1000.0,
    })
}

struct TemplateBuilder<'w, 's> {
    cmd: Commands<'w, 's>,
    assets: &'w Assets<HtmlTemplate>,
    server: &'w AssetServer,
    texture_atlases: &'w mut Assets<TextureAtlasLayout>,
    comps: &'w ComponentBindings,
    subscriber: TemplatePropertySubscriber,
}

struct TemplateBuilderState<'w> {
    scope: Entity,
    template: &'w HtmlTemplate,
    ids: HashMap<String, Entity>,
    targets: HashMap<Entity, String>,
    watch: HashMap<String, Vec<Entity>>,
    slottable_contents: Option<&'w Vec<XNode>>,
    properties: &'w TemplateProperties,
}

impl<'w> TemplateBuilderState<'w> {
    pub fn new(scope: Entity, template: &'w HtmlTemplate, slottable_contents: &'w Vec<XNode>, properties: &'w TemplateProperties) -> Self {
        let contents = if slottable_contents.len() > 0 {
            Some(slottable_contents)
        } else {
            None
        };

        Self {
            scope,
            template,
            ids: Default::default(),
            targets: Default::default(),
            watch: Default::default(),
            slottable_contents: contents,
            properties,
        }
    }
}

impl<'w, 's> TemplateBuilder<'w, 's> {
    pub fn new(
        cmd: Commands<'w, 's>,
        assets: &'w Assets<HtmlTemplate>,
        server: &'w AssetServer,
        texture_atlases: &'w mut Assets<TextureAtlasLayout>,
        comps: &'w ComponentBindings,
    ) -> Self {
        Self {
            cmd,
            assets,
            server,
            texture_atlases,
            comps,
            subscriber: Default::default(),
        }
    }

    pub fn finalize_relations(&mut self, state: &TemplateBuilderState) {
        state.ids.iter().for_each(|(id_string, entity)| {
            self.cmd.entity(*entity).insert(UiId(id_string.clone()));
        });

        state.targets
            .iter()
            .for_each(|(entity, target_id)| match state.ids.get(target_id) {
                Some(tar) => {
                    self.cmd.entity(*entity).insert(UiTarget(*tar));
                }
                None => warn!("target `{target_id}` not found for entity {entity}"),
            });

            state.watch
            .iter()
            .for_each(|(target_str, obs_list)| match state.ids.get(target_str) {
                Some(to_observe) => {
                    self.cmd
                        .entity(*to_observe)
                        .insert(InteractionObverser(obs_list.clone()));
                }
                None => warn!("undefined watch target `{target_str}`"),
            });

        self.cmd
            .entity(state.scope)
            .insert((std::mem::take(&mut self.subscriber), FullyBuild));
    }

    pub fn build_tree(&mut self, root: &XNode, state: &mut TemplateBuilderState) {
        self.build_node(state.scope, root, state);
        self.finalize_relations(&state);
    }

    fn fill_attr_tokens(&mut self, entity: Entity, node: &XNode, properties: &TemplateProperties, style: &mut HtmlStyle, tags: &mut Tags) -> Option<String> {
        let mut path: Option<String> = None;

        for attr in node.uncompiled.iter() {
            if let Some(attr) = attr.compile(properties) {
                match attr {
                    Attribute::Style(style_attr) => {
                        style.add_style_attr(style_attr);
                    }
                    Attribute::Action(action) => {
                        action.self_insert(self.cmd.entity(entity))
                    }
                    Attribute::Path(p) => {
                        path = Some(p);
                    }
                    Attribute::Tag(key, value) => {
                        tags.insert(key, value);
                    }
                    rest => {
                        warn!("attribute of this kind cannot be dynamic `{:?}`", rest);
                    }
                }
            }
        }

        return path;
    }

    fn build_basic_node(&mut self, entity: Entity, node: &XNode, state: &TemplateBuilderState) {
        let mut styles = HtmlStyle::from(node.styles.clone());
        let mut tags = Tags(node.tags.clone());

        let path = self.fill_attr_tokens(entity, node, state.properties, &mut styles, &mut tags);

        let mut bundle = self.cmd.entity(entity);

        //insert base node for styling
        bundle.insert(styles.computed.node.clone());

        // ----------------------
        // timers
        //todo: these should be optional and only set if set in the template
        bundle
            .insert(PressedTimer::new(Duration::from_secs_f32(
                styles.computed.delay.max(0.01),
            )))
            .insert(HoverTimer::new(Duration::from_secs_f32(
                styles.computed.delay.max(0.01),
            )));

        //todo: do we still need this?
        if entity != state.scope {
            bundle.insert(TemplateScope(state.scope));
        }

        // ----------------------
        //tags
        if tags.0.len() > 0 {
            bundle.insert(tags);
        }

        match &node.node_type {
            NodeType::Image => {
                let animation_option = build_animation(&styles);
                let mut starting_frame = 0;

                if animation_option.is_some() {
                    let animation = animation_option.unwrap();
                    starting_frame = animation.frame;

                    bundle.insert(animation);
                }

                let mut img = Handle::<Image>::default();

                if let Some(src) = &node.src {
                    img = self.server.load(src);
                } else if let Some(src) = path {
                    img = self.server.load(src);
                }

                bundle.insert(
                    ImageNode {
                        image: img,
                        image_mode: styles
                            .computed
                            .image_mode
                            .as_ref()
                            .cloned()
                            .unwrap_or_default(),
                        rect: styles.computed.image_region.clone(),
                        texture_atlas: styles
                            .computed
                            .atlas
                            .as_ref()
                            .map(|atlas| {
                                let atlas_layout = TextureAtlasLayout::from_grid(atlas.size, atlas.columns, atlas.rows, atlas.padding, atlas.offset);
                                let atlas_handle = self.texture_atlases.add(atlas_layout);

                                TextureAtlas {
                                    layout: atlas_handle,
                                    index: starting_frame,
                                }
                            }),
                        ..default()
                    }
                );
            }
            NodeType::Text => {
                match &node.content {
                    Some(content) => {
                        let (processed, count) = replace_placeholders(content.as_str(), state.properties);

                        bundle.insert(Text(processed));

                        if count > 0 {
                            self.subscriber.push(entity);
                        }
                    }
                    None => {
                        bundle.insert(Text::default());
                    }
                }
            }
            NodeType::Button => {
                bundle.insert(Button);
            }
            _ => {}
        };

        // ---------------------
        // shadow
        if let Some(shadow) = styles.computed.shadow {
            bundle.insert(shadow.clone());
        }

        // ----------------------
        // dirty outline
        if let Some(outline) = styles.computed.outline.as_ref() {
            bundle.insert(outline.clone());
        }

        //apply initial styling
        bundle.insert((
            TextFont {
                font: styles.computed.font.clone(),
                font_size: styles.computed.font_size,
                ..Default::default()
            },
            TextColor(styles.computed.font_color),
            BackgroundColor(styles.computed.background),
            BorderRadius {
                top_left: styles.computed.border_radius.top,
                top_right: styles.computed.border_radius.right,
                bottom_right: styles.computed.border_radius.bottom,
                bottom_left: styles.computed.border_radius.left,
            },
            BorderColor(styles.computed.border_color),
            styles,
        ));
    }

    fn build_custom_node(&mut self, entity: Entity, custom: &str, node: &XNode) {
        let Some(comp) = self.comps.get(custom) else {
            error!("trying to spawn unregistered custom node `{custom}`");
            return;
        };

        let Some(template) = self.assets.get(comp.asset_id) else {
            error!("unable to find template for custom node `{custom}`");
            return;
        };

        let mut properties = TemplateProperties::from(node.defs.clone());
        
        template.properties.iter().for_each(|(key, val)| {
            _ = properties.try_insert(key.to_owned(), val.clone());
        });

        let mut state = TemplateBuilderState::new(entity, template, &node.children, &properties);

        let Some(root) = template.root.first() else {
            error!("custom node `{custom}` has no root node");
            return;
        };
        
        self.build_node(entity, root, &mut state);
        self.finalize_relations(&state);
    }

    fn build_node_child(&mut self, entity: Entity, node: &XNode, state: &mut TemplateBuilderState) {
        match node.node_type {
            NodeType::Slot => {
                match state.slottable_contents {
                    Some(contents) => {
                        contents.iter().for_each(|child| {
                            self.build_node_child(entity, child, state);
                        });
                        
                        state.slottable_contents = None;
                    }
                    None => {}
                }
            }
            _ => {
                let child_entity = self.cmd.spawn_empty().id();
                self.build_node(child_entity, node, state);
                self.cmd.entity(entity).add_child(child_entity);
            }
        }
    }

    fn build_node(&mut self, entity: Entity, node: &XNode, state: &mut TemplateBuilderState) {
        match node.node_type {
            NodeType::Custom(ref custom) => {
                self.build_custom_node(entity, custom, node);
            }
            NodeType::Slot => {
                warn!("slot node should not be present in the root of a template");
            }
            _ => {
                //ADDRESS THIS STUFF!!!!!!!!!!
        
                // ----------------------
                //register prop listner
                if node.uncompiled.len() > 0 {
                    self.cmd.entity(entity).insert(TemplateExpresions(
                        node.uncompiled.iter().cloned().collect(),
                    ));
                    self.subscriber.push(entity);
                }
        
                // ----------------------
                // connections
                if let Some(id) = &node.id {
                    state.ids.insert(id.clone(), entity);
                }
                if let Some(target) = &node.target {
                    state.targets.insert(entity, target.clone());
                }
        
                if let Some(watch) = &node.watch {
                    match state.watch.get_mut(watch) {
                        Some(list) => {
                            list.push(entity);
                        }
                        None => {
                            state.watch.insert(watch.clone(), vec![entity]);
                        }
                    };
                }

                self.build_basic_node(entity, node, state);

                for child in node.children.iter() {
                    self.build_node_child(entity, child, state);
                }
                
                // ----------------------
                // events
                node.event_listener.iter().for_each(|listener| {
                    listener.clone().self_insert(self.cmd.entity(entity));
                });
            }
        }
    }
}

fn extract_all_placeholders(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many0(
        tuple((
            take_until("{"),
            delimited(tag("{"), preceded(multispace0, is_not("}")), tag("}")),
        ))
    )(input)
}

fn replace_placeholders(input: &str, replacements: &HashMap<String, String>) -> (String, u8) {
    let mut result = String::new();
    let mut remaining = input;
    let mut count: u8 = 0;

    match extract_all_placeholders(input) {
        Ok((rest, matches)) => {
            count = matches.len() as u8;

            for (before, key) in matches {
                result.push_str(before);
                
                if let Some(value) = replacements.get(key) {
                    result.push_str(value);
                } else {
                    result.push_str(&format!("{{{}}}", key));
                }
                remaining = rest;
            }
        }
        Err(_) => {
            // If parsing fails, return the original string
            result.push_str(input);
        }
    }

    // Append any remaining text after the last match
    result.push_str(remaining);

    (result.trim().to_string(), count)
}