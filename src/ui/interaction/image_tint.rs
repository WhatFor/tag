use bevy::prelude::*;

#[derive(Component)]
#[require(Button)]
pub struct ImageTint {
    pub normal: Color,
    pub hover: Color,
    pub pressed: Color,
}

impl ImageTint {
    /// Takes a base colour and provides a hover colour at 50% darker and a
    /// pressed colour at 70% darker.
    /// Requires a ImageNode child to act on.
    pub fn darken(base: Color) -> Self {
        Self {
            normal: base,
            hover: base.darker(0.5),
            pressed: base.darker(0.7),
        }
    }
}

pub struct ImageTintInteractionPlugin;

impl Plugin for ImageTintInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, image_tint);
    }
}

fn image_tint(
    buttons: Query<(&Interaction, &ImageTint, &Children), Changed<Interaction>>,
    mut images: Query<&mut ImageNode>,
) {
    for (interaction, tint, children) in &buttons {
        let color = match interaction {
            Interaction::Pressed => tint.pressed,
            Interaction::Hovered => tint.hover,
            Interaction::None => tint.normal,
        };

        for &child in children {
            if let Ok(mut image) = images.get_mut(child) {
                image.color = color;
            }
        }
    }
}
