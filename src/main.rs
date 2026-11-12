use bevy::{
    animation::{AnimatedBy, AnimationEvent, AnimationTargetId, animated_field},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_observer(|_: On<AnimEvent>| {
            info!("Event received in observer");
        })
        .add_systems(Startup, setup)
        .run();
}

#[derive(AnimationEvent, Clone)]
struct AnimEvent;

fn setup(
    mut commands: Commands,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<BlendGraph>>,
) {
    commands.spawn((Camera2d, Msaa::Off));

    let planet = Name::new("planet");
    let planet_target_id = AnimationTargetId::from_name(&planet);

    let other = Name::new("other");
    let other_target_id = AnimationTargetId::from_names([&planet, &other].into_iter());
    info!("init other_target_id={other_target_id:?}");

    let mut animation = AnimationClip::default();

    animation.add_curve_to_target(
        planet_target_id,
        BlendableCurve::new(
            animated_field!(Transform::translation),
            UnevenSampleAutoCurve::new([0.0, 2.0, 4.0].into_iter().zip([
                Vec3::new(100.0, 0.0, 0.0),
                Vec3::new(-100.0, 0.0, 0.0),
                Vec3::new(100.0, 0.0, 0.0),
            ]))
            .expect("should be able to build translation curve because we pass in valid samples"),
        ),
    );

    // Working
    //animation.add_event(2.0, AnimEvent);
    //animation.add_event_fn(2.0, |_, _, _, _| {
    //    info!("Event received in fn");
    //});

    // Bug is here! VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV
    animation.add_event_to_target(other_target_id, 2.0, AnimEvent);
    animation.add_event_fn_to_target(other_target_id, 2.0, |_, _, _, _| {
        info!("Event received in fn");
    });
    // Bug is here! ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    let (graph, animation_index) = BlendGraph::from_clip(animations.add(animation));

    let mut player = AnimationPlayer::default();
    player.play(animation_index).repeat();

    let planet_entity = commands
        .spawn((
            Sprite::sized(Vec2::splat(21.0)),
            planet,
            BlendGraphHandle(graphs.add(graph)),
            player,
        ))
        .id();
    commands
        .entity(planet_entity)
        .insert((planet_target_id, AnimatedBy(planet_entity)))
        .with_child((other_target_id, AnimatedBy(planet_entity)));
}
