use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;
use std::vec;

#[derive(Debug, Clone)]
pub enum GameEvent {
    EnemySpotted { distance: f32 },
    DamageTaken { amount: i32 },
    ItemFound { item: String },

    AllClear,
    Tick,
}

#[derive(Debug)]
pub enum AIAction {
    Idle,
    Patrol,
    Attack { target_distance: f32 },
    Flee,
    PickupItem { item: String },
    Heal,
    Speak(String),
}

pub fn ai_controller_coroutine() -> impl Coroutine<GameEvent, Yield = Vec<AIAction>, Return = String>
{
    #[coroutine]
    |mut event: GameEvent| {
        let mut hp = 100;
        let mut items_collected = 0;

        //this will consume first event and just say halo, no matter what it was
        event = yield vec![AIAction::Speak("hello moto :)".to_string())];

        loop {
            match event {
                GameEvent::Tick => {
                    if hp < 30 {
                        event = yield vec![AIAction::Heal];
                        hp = (hp + 20).min(100);
                    } else {
                        event = yield vec![AIAction::Patrol];
                    }
                }

                GameEvent::EnemySpotted { distance } => {
                    if hp < 30 {
                        event =
                            yield vec![AIAction::Speak("woop, c y 👋".to_string()), AIAction::Flee];
                    } else if distance < 5.0 {
                        event = yield vec![
                            AIAction::Speak("gotcha ⚔️".to_string()),
                            AIAction::Attack {
                                target_distance: distance,
                            },
                        ];
                    } else {
                        event = yield vec![AIAction::Patrol];
                    }
                }

                GameEvent::DamageTaken { amount } => {
                    hp = (hp - amount).max(0);

                    if hp <= 0 {
                        yield vec![AIAction::Speak("Defeated! 🪦".to_string())];
                        return "Bye, Bye!".to_string();
                    } else {
                        event = yield vec![AIAction::Speak(format!("Ouch! HP: {}", hp))];
                    }
                }

                // todo change so it returns collection of actions instead of single action
                GameEvent::ItemFound { item } => {
                    items_collected += 1;
                    event = yield vec![
                        AIAction::PickupItem { item: item.clone() },
                        AIAction::Speak(format!(
                            "Found {}! Total items: {}",
                            item, items_collected
                        )),
                    ];
                }

                GameEvent::AllClear => {
                    event = yield vec![AIAction::Speak("Area secure".to_string()), AIAction::Idle];
                }
            }
        }
    }
}

#[test]
fn demo() {
    let events = vec![
        GameEvent::Tick,
        GameEvent::Tick,
        GameEvent::EnemySpotted { distance: 10.0 },
        GameEvent::Tick,
        GameEvent::EnemySpotted { distance: 3.0 },
        GameEvent::DamageTaken { amount: 40 },
        GameEvent::ItemFound {
            item: "Health Potion".to_string(),
        },
        GameEvent::DamageTaken { amount: 35 },
        GameEvent::EnemySpotted { distance: 2.0 },
        GameEvent::AllClear,
        GameEvent::DamageTaken { amount: 50 },
    ];

    let mut ai = ai_controller_coroutine();
    let mut event_iter = events.into_iter();
    let mut current_event = event_iter.next().unwrap();

    loop {
        match Pin::new(&mut ai).resume(current_event.clone()) {
            CoroutineState::Yielded(action) => {
                println!("Event: {:?}", current_event);
                println!("-> Action: {:?}\n", action);

                //additional check to break the loop if there is no more events
                current_event = match event_iter.next() {
                    Some(e) => e,
                    None => {
                        println!("[No more events]");
                        break;
                    }
                };
            }
            CoroutineState::Complete(result) => {
                println!("[AI sim ended: {}]", result);
                break;
            }
        }
    }
}
