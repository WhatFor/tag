use crate::prelude::*;
use bevy::prelude::*;

use crate::ui::interaction::image_tint::ImageTint;

#[derive(Component)]
pub struct Panel {
    pub title: String,
    pub height: Val,
    pub width: Val,
}

#[derive(EntityEvent)]
pub struct DespawnPanel {
    pub entity: Entity,
}

const CLOSE_BUTTON_ICON_SIZE: f32 = 32.;

impl Panel {
    pub fn new(title: String, height: Val, width: Val) -> Self {
        Self {
            title,
            height,
            width,
        }
    }

    pub fn default(title: String) -> Self {
        Self {
            title,
            height: Val::Percent(DEFAULT_SIZE),
            width: Val::Percent(DEFAULT_SIZE),
        }
    }
}

pub struct PanelWidgetPlugin;

impl Plugin for PanelWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init.run_if(not(in_state(GameState::Initialising))));
        app.add_observer(on_despawn_panel);
    }
}

const BG_COLOUR: Color = Color::srgb(0.15, 0.15, 0.15);
const BACKDROP_COLOUR: Color = Color::srgba(0.05, 0.05, 0.05, 0.45);
const BORDER_COLOUR: Color = Color::srgb(1., 1., 1.);
const BORDER_SIZE: f32 = 4.;

const DEFAULT_SIZE: f32 = 60.;

pub fn init(
    mut commands: Commands,
    query: Query<(Entity, &Panel), Added<Panel>>,
    fonts: Res<FontAssets>,
    icon_store: Res<IconAssets>,
    audio_store: Res<AudioAssets>,
) {
    let Some(click_sfx) = audio_store.sfx.get("click") else {
        warn!("Failed to find click SFX. See: {:?}", audio_store.sfx);
        return;
    };

    let Some(x_icon) = icon_store.icons.get("x").cloned() else {
        warn!("Failed to find x icon. See: {:?}", icon_store.icons);
        return;
    };

    for (panel_entity, panel) in &query {
        let backdrop = commands
            .spawn((
                Name::new("Panel Backdrop"),
                BackgroundColor(BACKDROP_COLOUR),
                GlobalZIndex(LAYER_MENU - 1),
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
            ))
            // TODO: This triggers on click of content too
            // .observe(move |_: On<Pointer<Click>>, mut commands: Commands| {
            //     commands
            //         .entity(panel_entity)
            //         .trigger(|p| DespawnPanel { entity: p });
            // })
            .id();

        let header = commands
            .spawn(Node {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|h| {
                h.spawn((
                    Node {
                        padding: UiRect {
                            left: Val::Px(20.),
                            right: Val::Px(20.),
                            top: Val::Px(10.),
                            bottom: Val::Px(10.),
                        },
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![(
                        Name::new("Panel Title"),
                        Text::new(panel.title.clone()),
                        fonts.ui_color,
                        fonts.ui_font.clone(),
                    )],
                ));

                h.spawn((
                    Button,
                    ImageTint::darken(Color::srgb(1., 1., 1.)),
                    ClickSfx::from(click_sfx.clone()),
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
                    children![(
                        Name::new("Panel Close Button Icon"),
                        Node {
                            width: Val::Px(CLOSE_BUTTON_ICON_SIZE),
                            height: Val::Px(CLOSE_BUTTON_ICON_SIZE),
                            ..default()
                        },
                        Pickable::IGNORE,
                        ImageNode::new(x_icon.clone()),
                    )],
                ))
                .observe(move |_: On<Pointer<Click>>, mut commands: Commands| {
                    commands
                        .entity(panel_entity)
                        .trigger(|p| DespawnPanel { entity: p });
                });
            })
            .id();

        commands
            .entity(panel_entity)
            .insert((
                ChildOf(backdrop),
                Node {
                    width: panel.width,
                    height: panel.height,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(BORDER_SIZE)),
                    ..default()
                },
                BackgroundColor(BG_COLOUR),
                BorderColor::all(BORDER_COLOUR),
                GlobalZIndex(LAYER_MENU),
                Pickable {
                    is_hoverable: true,
                    should_block_lower: true,
                },
            ))
            .insert_child(0, header);
    }
}

fn on_despawn_panel(trigger: On<DespawnPanel>, parents: Query<&ChildOf>, mut commands: Commands) {
    let panel = trigger.event_target();
    let backdrop_parent = parents.get(panel).map(|p| p.parent()).unwrap_or(panel);

    commands.entity(backdrop_parent).try_despawn();
}
