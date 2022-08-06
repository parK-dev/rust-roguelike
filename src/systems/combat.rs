use crate::prelude::*;
// TODO: Remove println statements
#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    for (message, victim) in &victims {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
            println!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    }
}
