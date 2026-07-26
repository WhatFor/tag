use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::interaction::image_tint::ImageTint;
use bevy::ecs::relationship::RelatedSpawner;
use bevy::ecs::spawn::SpawnableList;

const CLOSE_BUTTON_ICON_SIZE: f32 = 32.;

const BG_COLOUR: Color = Color::srgb(0.15, 0.15, 0.15);
const BACKDROP_COLOUR: Color = Color::srgba(0.05, 0.05, 0.05, 0.45);
const FRAME_COLOUR: Color = Color::srgb(0.8, 0.8, 0.8);

const DEFAULT_SIZE: f32 = 60.;

pub struct PanelWidgetPlugin;

impl Plugin for PanelWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_despawn_panel);
        app.add_observer(on_backdrop_click);
    }
}

#[derive(Component)]
pub struct Panel {
    can_close: bool,
}

#[derive(Component)]
pub struct PanelFrame;

pub struct PanelProps {
    pub title: String,
    pub width: Val,
    pub height: Val,
    pub can_close: bool,
}

#[derive(EntityEvent)]
pub struct DespawnPanel {
    pub entity: Entity,
}

impl PanelProps {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            height: Val::Percent(DEFAULT_SIZE),
            width: Val::Percent(DEFAULT_SIZE),
            title: title.into(),
            can_close: true,
        }
    }

    pub fn unclosable(mut self) -> Self {
        self.can_close = false;
        self
    }

    pub fn sized(mut self, width: Val, height: Val) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

pub fn panel<C>(props: PanelProps, content: C) -> impl Bundle
where
    C: SpawnableList<ChildOf> + Send + Sync + 'static,
{
    (
        Panel {
            can_close: props.can_close,
        },
        Name::new(format!("{} Panel", props.title)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.),
            top: Val::Px(0.),
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(BACKDROP_COLOUR),
        GlobalZIndex(LAYER_MENU),
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            let panel = parent.target_entity();

            let (ui_font, ui_color, click_sfx, x_icon, border_texture) = {
                let world = parent.world();

                let fonts = world
                    .get_resource::<FontAssets>()
                    .expect("Font Assets not found");

                let audio = world
                    .get_resource::<AudioAssets>()
                    .expect("Audio Assets not found");

                let icons = world
                    .get_resource::<UiIconAssets>()
                    .expect("UI Icon Assets not found");

                let textures = world
                    .get_resource::<TextureAssets>()
                    .expect("Texture Assets not found");

                (
                    fonts.ui_font.clone(),
                    fonts.ui_color,
                    audio.sfx.get("click").cloned(),
                    icons.icons.get("x").cloned(),
                    textures.textures.get("panel_border").cloned(),
                )
            };

            let border_slicer = TextureSlicer {
                border: BorderRect::all(32.),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 1.,
            };

            let mut frame = parent.spawn((
                PanelFrame,
                Node {
                    width: props.width,
                    height: props.height,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(BG_COLOUR),
                Pickable {
                    is_hoverable: true,
                    should_block_lower: true,
                },
                Children::spawn((
                    Spawn((
                        Node {
                            width: Val::Percent(100.),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Children::spawn(SpawnWith(move |header: &mut RelatedSpawner<ChildOf>| {
                            header.spawn((
                                Name::new("Panel Title"),
                                Text::new(props.title),
                                ui_color,
                                ui_font,
                            ));

                            if props.can_close {
                                let mut button = header.spawn((
                                    Button,
                                    ImageTint::darken(Color::srgb(1., 1., 1.)),
                                    Name::new("Panel Close Button"),
                                    Node {
                                        padding: UiRect {
                                            left: Val::Px(20.),
                                            right: Val::Px(20.),
                                            top: Val::Px(10.),
                                            bottom: Val::Px(10.),
                                        },
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                ));

                                if let Some(x) = x_icon {
                                    button.with_child((
                                        Name::new("Panel Close Button Icon"),
                                        Node {
                                            width: Val::Px(CLOSE_BUTTON_ICON_SIZE),
                                            height: Val::Px(CLOSE_BUTTON_ICON_SIZE),
                                            ..default()
                                        },
                                        Pickable::IGNORE,
                                        ImageNode::new(x),
                                    ));
                                } else {
                                    warn!("Unable to find X icon in panel.");
                                }

                                if let Some(sfx) = click_sfx {
                                    button.insert(ClickSfx::from(sfx));
                                } else {
                                    warn!("Unable to find Click SFX in panel.");
                                }

                                button.observe(
                                    move |_: On<Pointer<Click>>, mut commands: Commands| {
                                        commands
                                            .entity(panel)
                                            .trigger(|p| DespawnPanel { entity: p });
                                    },
                                );
                            }
                        })),
                    )),
                    content,
                )),
            ));

            if let Some(border) = border_texture {
                frame.insert(ImageNode {
                    image: border,
                    color: FRAME_COLOUR,
                    image_mode: NodeImageMode::Sliced(border_slicer),
                    ..default()
                });
            } else {
                warn!("Unable to find border texture for panel.");
            }
        })),
    )
}

fn on_despawn_panel(trigger: On<DespawnPanel>, mut commands: Commands) {
    commands.entity(trigger.event_target()).try_despawn();
}

fn on_backdrop_click(trigger: On<Pointer<Click>>, panels: Query<&Panel>, mut commands: Commands) {
    if trigger.original_event_target() != trigger.event_target() {
        return;
    }

    let target = trigger.event_target();

    let Ok(panel) = panels.get(target) else {
        return;
    };

    if !panel.can_close {
        return;
    }

    commands
        .entity(target)
        .trigger(|e| DespawnPanel { entity: e });
}
