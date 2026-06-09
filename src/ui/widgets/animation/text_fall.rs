use bevy::prelude::*;

#[derive(Component)]
#[require(Node)]
pub struct AnimateTextFall {
    pub lines: Vec<String>,
    pub font: TextFont,
    pub color: TextColor,
}

#[derive(Component)]
struct LastInAnimation(pub Entity);

#[derive(Component)]
struct AnimationComplete;

#[derive(EntityEvent)]
pub struct AnimateTextFallComplete {
    entity: Entity,
}

pub struct TextFallAnimationPlugin;

impl Plugin for TextFallAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, build_text_fall);
    }
}

fn build_text_fall(
    mut commands: Commands,
    query: Query<(Entity, &AnimateTextFall), Added<AnimateTextFall>>,
) {
    for (entity, animation) in &query {}
}

fn on_char_done(mut commands: Commands, query: Query<&LastInAnimation, Added<AnimationComplete>>) {
    for entity in &query {
        commands.trigger(AnimateTextFallComplete { entity: entity.0 });
    }
}
