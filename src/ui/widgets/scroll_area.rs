use bevy::{
    ecs::relationship::RelatedSpawner,
    input::mouse::MouseScrollUnit,
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{ControlOrientation, CoreScrollbarDragState, CoreScrollbarThumb, Scrollbar},
};

pub const NORMAL: Color = Color::srgb(0.2, 0.2, 0.2);
pub const HOVERED: Color = Color::srgb(0.3, 0.3, 0.3);

pub const SCROLL_WIDTH: f32 = 8.;

pub struct ScrollAreaWidgetPlugin;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScrollArea;

impl Plugin for ScrollAreaWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_scrollbar_thumb);
    }
}

// See: https://bevy.org/examples/ui-user-interface/scrollbars/
pub fn scroll_area<TChildren>(child_spawner: TChildren) -> impl Bundle
where
    TChildren: FnOnce(&mut RelatedSpawner<ChildOf>) + Send + Sync + 'static,
{
    (
        ScrollArea,
        Node {
            display: Display::Grid,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            grid_template_columns: vec![
                RepeatedGridTrack::flex(1, 1.),
                RepeatedGridTrack::px(1, SCROLL_WIDTH),
            ],
            column_gap: px(2),
            ..default()
        },
        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
            let scroll_area_id = parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(px(4)),
                        overflow: Overflow::scroll(),
                        ..default()
                    },
                    ScrollPosition(Vec2::new(0.0, 0.0)),
                    Children::spawn(SpawnWith(child_spawner)),
                ))
                .observe(
                    |target: On<Pointer<Scroll>>, mut query: Query<&mut ScrollPosition>| {
                        let Ok(mut pos) = query.get_mut(target.event_target()) else {
                            return;
                        };

                        let dy = match target.unit {
                            bevy::input::mouse::MouseScrollUnit::Line => {
                                target.y * MouseScrollUnit::SCROLL_UNIT_CONVERSION_FACTOR
                            }
                            bevy::input::mouse::MouseScrollUnit::Pixel => target.y,
                        };

                        pos.0.y = (pos.0.y - dy).max(0.);
                    },
                )
                .id();

            // Y Scrollbar track and thumb
            parent.spawn((
                Node {
                    min_width: px(SCROLL_WIDTH),
                    grid_row: GridPlacement::start(1),
                    grid_column: GridPlacement::start(2),
                    ..default()
                },
                Name::new("Scrollbar Area"),
                Scrollbar {
                    orientation: ControlOrientation::Vertical,
                    target: scroll_area_id,
                    min_thumb_length: SCROLL_WIDTH,
                },
                Children::spawn(Spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        border_radius: BorderRadius::all(px(SCROLL_WIDTH / 2.)),
                        ..default()
                    },
                    Hovered::default(),
                    BackgroundColor(NORMAL),
                    CoreScrollbarThumb,
                    Name::new("T Scrollbar Thumb"),
                ))),
            ));
        })),
    )
}

fn update_scrollbar_thumb(
    mut q_thumb: Query<
        (&mut BackgroundColor, &Hovered, &CoreScrollbarDragState),
        (
            With<CoreScrollbarThumb>,
            Or<(Changed<Hovered>, Changed<CoreScrollbarDragState>)>,
        ),
    >,
) {
    for (mut thumb_bg, Hovered(is_hovering), drag) in q_thumb.iter_mut() {
        let color: Color = if *is_hovering || drag.dragging {
            HOVERED
        } else {
            NORMAL
        }
        .into();

        thumb_bg.0 = color;
    }
}
