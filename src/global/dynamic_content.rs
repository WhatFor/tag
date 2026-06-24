use crate::prelude::*;
use bevy::prelude::*;

use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct DynamicContent<'w, 's> {
    player: Query<'w, 's, &'static Health, With<Player>>,
}

impl DynamicContent<'_, '_> {
    fn lookup(&self, key: &str) -> Option<String> {
        match key {
            "player_health" => Some(self.player.single().ok()?.0.to_string()),
            _ => None,
        }
    }

    pub fn resolve(&self, template: &str) -> String {
        substitute(template, |k| self.lookup(k))
    }
}

fn substitute(template: &str, lookup: impl Fn(&str) -> Option<String>) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;

    while let Some(start) = rest.find('{') {
        out.push_str(&rest[..start]);

        match rest[start + 1..].find('}') {
            Some(end) => {
                let key = &rest[start + 1..start + 1 + end];

                match lookup(key) {
                    Some(v) => out.push_str(&v),
                    None => {
                        out.push('{');
                        out.push_str(key);
                        out.push('}');
                    }
                }

                rest = &rest[start + 1 + end + 1..];
            }
            None => {
                out.push_str(&rest[start..]);
                break;
            }
        }
    }

    out.push_str(rest);

    out
}
