use bevy::prelude::*;

use crate::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, loading.run_if(in_state(AppState::Loading)));
    }
}

fn loading(
    mut cmd: Commands,
    scene: Option<Res<MainScene>>,
    fx: Option<Res<ParticleEffects>>,
    text_resource: Option<Res<TextResource>>,
    mut ns: ResMut<NextState<AppState>>,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    if scene.is_none() {
        return;
    }
    if text_resource.is_none() {
        return;
    }
    if fx.is_none() {
        return;
    }

    if !loading.get(server) {
        cmd.remove_resource::<AssetsLoading>();
        ns.set(AppState::MainMenu);
    };
}
