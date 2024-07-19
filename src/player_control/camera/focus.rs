use crate::util::{single, single_mut};
use crate::{
    level_instantiation::on_spawn::{player, Player},
    player_control::camera::IngameCamera,
    world_interaction::dialog::CurrentDialogTarget,
};
use bevy::prelude::*;
use bevy_yarnspinner::events::DialogueCompleteEvent;

pub(super) fn set_camera_focus(
    mut camera_query: Query<&mut IngameCamera>,
    player_query: Query<&Transform, With<Player>>,
    dialog_targets: Query<&Transform, Without<Player>>,
    dialog_target: Res<CurrentDialogTarget>,
    mut dialogue_complete_event: EventReader<DialogueCompleteEvent>,
) {
    let mut camera = single_mut!(camera_query);
    let player_transform = single!(player_query);
    if let Some(dialog_target) = dialog_target.0 {
        let dialog_target_transform = dialog_targets.get(dialog_target).unwrap();
        camera.secondary_target = Some(dialog_target_transform.translation);
    }
    camera.target = player_transform.translation + Vec3::Y * player::HEIGHT / 2.;

    for _event in dialogue_complete_event.read() {
        for mut camera in camera_query.iter_mut() {
            camera.secondary_target = None;
        }
    }
}
