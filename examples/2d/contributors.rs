use bevy::prelude::*;
use rand::{prelude::SliceRandom, Rng};
use std::{
    collections::BTreeSet,
    env::VarError,
    io::{self, BufRead, BufReader},
    process::Stdio,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_contributor_selection)
        .add_startup_system(setup)
        .add_system(velocity_system)
        .add_system(move_system)
        .add_system(collision_system)
        .add_system(select_system)
        .run();
}

type Contributors = BTreeSet<String>;

struct ContributorSelection {
    order: Vec<(String, Entity)>,
    idx: usize,
}

#[derive(Component)]
struct SelectTimer;

#[derive(Component)]
struct ContributorDisplay;

#[derive(Component)]
struct Contributor {
    hue: f32,
}

#[derive(Component)]
struct Velocity {
    translation: Vec3,
    rotation: f32,
}

enum LoadContributorsError {
    IO(io::Error),
    Var(VarError),
    Stdout,
}

const GRAVITY: f32 = -9.821 * 100.0;
const SPRITE_SIZE: f32 = 75.0;

const SATURATION_DESELECTED: f32 = 0.3;
const LIGHTNESS_DESELECTED: f32 = 0.2;
const SATURATION_SELECTED: f32 = 0.9;
const LIGHTNESS_SELECTED: f32 = 0.7;
const ALPHA: f32 = 0.92;

const SHOWCASE_TIMER_SECS: f32 = 3.0;

const CONTRIBUTORS_LIST: &[&str] = &[
    "0x22fe",
    "8bp",
    "Aaron Housh",
    "Aaron Winter",
    "Adamaq01",
    "Adam Bates",
    "adekau",
    "Aevyrie",
    "Agorgianitis Loukas",
    "Alec Deason",
    "Alessandro Re",
    "Alex",
    "Alexander Krivács Schrøder",
    "Alexander Sepity",
    "Alex.F",
    "Alex Hirsch",
    "Alice Cecile",
    "Alister Lee",
    "Al M",
    "aloucks",
    "Amber Kowalski",
    "Anders Rasmussen",
    "andoco",
    "Andreas Weibye",
    "André Heringer",
    "Andre Kuehne",
    "Andre Popovitch",
    "Andrew Hickman",
    "AngelicosPhosphoros",
    "Anselmo Sampietro",
    "Archina",
    "bg",
    "bilsen",
    "BimDav",
    "bjorn3",
    "Boiethios",
    "Boutillier",
    "Boxy",
    "Bram Buurlage",
    "caelunshun",
    "Caleb Boylan",
    "Callum Tolley",
    "Cameron Hart",
    "carter",
    "Carter Anderson",
    "Catherine Gilbert",
    "CGMossa",
    "Charles Giguere",
    "Chris Janaqi",
    "Christopher Durham",
    "Chrs Msln",
    "Claire C",
    "ColonisationCaptain",
    "Corey Farwell",
    "Cory Forsstrom",
    "CrazyRoka",
    "Csányi István",
    "Daniel Borges",
    "Daniel Burrows",
    "Daniel Jordaan",
    "Daniel McNab",
    "Dashiell Elliott",
    "dataphract",
    "David Ackerman",
    "David McClung",
    "davier",
    "Denis Laprise",
    "dependabot[bot]",
    "deprilula28",
    "DGriffin91",
    "Digital Seven",
    "Dimev",
    "Dimitri Belopopsky",
    "Dimitri Bobkov",
    "dinococo",
    "dintho",
    "Downtime",
    "Duncan",
    "Dusty DeWeese",
    "easynam",
    "Elias",
    "Emanuel Lindström",
    "EthanYidong",
    "Fabian Löschner",
    "Fabian Würfl",
    "Federico Rinaldi",
    "Felipe Jorge",
    "figsoda",
    "FlyingRatBull",
    "follower",
    "forbjok",
    "Forest Anderson",
    "François",
    "Freya",
    "Fuyang Liu",
    "Gab Campbell",
    "GabLotus",
    "Garett Cooper",
    "Georg Friedrich Schuppe",
    "Gilbert Röhrbein",
    "giusdp",
    "Grant Moyer",
    "Gray Olson",
    "Grayson Burton",
    "Gregor",
    "Gregory Oakes",
    "Grindv1k",
    "Guillaume DALLENNE",
    "Guim Caballero",
    "Halfwhit",
    "Hans W. Uhlig",
    "Hoidigan",
    "Hugo Lindsay",
    "HyperLightKitsune",
    "Iaiao",
    "ifletsomeclaire",
    "Ilja Kartašov",
    "iMplode nZ",
    "Isaak Eriksson",
    "Ixentus",
    "Jackson Lango",
    "Jacob Gardner",
    "jak6jak",
    "Jake Kerr",
    "Jakob Hellermann",
    "James Higgins",
    "James Leflang",
    "James Liu",
    "James R",
    "Jasen Borisov",
    "Jay Oster",
    "Jeremiah Senkpiel",
    "Jerome Humbert",
    "jngbsn",
    "João Capucho",
    "Joel Nordström",
    "Johan Klokkhammer Helsing",
    "John",
    "John Doneth",
    "John Mitchell",
    "Jonas Matser",
    "Jonathan Behrens",
    "Jonathan Cornaz",
    "Josh Kuhn",
    "Josh Taylor",
    "Joshua Chapman",
    "Joshua J. Bouw",
    "Joshua Ols",
    "julhe",
    "Julian Heinken",
    "Junfeng Liu",
    "kaflu",
    "karroffel",
    "Kenneth Dodrill",
    "Klim Tsoutsman",
    "Kurt Kühnert",
    "Lachlan Sneff",
    "lambdagolem",
    "lee-orr",
    "Léo Gillot-Lamure",
    "Loch Wansbrough",
    "Logan Collins",
    "Logan Magee",
    "Lucas Kent",
    "Lucas Rocha",
    "Lukas Orsvärn",
    "Lukas Wirth",
    "M",
    "Marcel Müller",
    "Marc Parenteau",
    "Marcus Buffett",
    "Marek Fajkus",
    "Marek Legris",
    "marius851000",
    "Mariusz Kryński",
    "Mark",
    "Martin Lavoie",
    "Martín Maita",
    "Martin Svanberg",
    "Mat Hostetter",
    "Matteo Guglielmetti",
    "Matthias Seiffert",
    "Max Bruckner",
    "maxwellodri",
    "memoryruins",
    "mfrancis107",
    "MGlolenstine",
    "Michael Hills",
    "Michael Tang",
    "Mika",
    "Mikail Khan",
    "Mike",
    "Milan Vaško",
    "milkybit",
    "MinerSebas",
    "Minghao Liu",
    "MiniaczQ",
    "Mirko Rainer",
    "Moxinilian",
    "MsK`",
    "multun",
    "Nathan Jeffords",
    "Nathan Stocks",
    "Nathan Ward",
    "Nibor62",
    "Nicholas Rishel",
    "Nick",
    "Nikita Zdanovitch",
    "Niklas Eicker",
    "Noah Callaway",
    "Nolan Darilek",
    "Olivier Pinon",
    "OptimisticPeach",
    "Oscar",
    "Patrick Greene",
    "Patrik Buhring",
    "Paweł Grabarz",
    "Philip Degarmo",
    "Philipp Mildenberger",
    "Piotr Balcer",
    "Plecra",
    "Protowalker",
    "Psychoticpotato",
    "r00ster",
    "Raymond",
    "RedlineTriad",
    "reidbhuntley",
    "Rémi Lauzier",
    "Renato Caldas",
    "Restioson",
    "Richard Tjerngren",
    "RiskLove",
    "rmsthebest",
    "Rob",
    "Robbie Davenport",
    "Robert Swain",
    "Rob Parrett",
    "rod-salazar",
    "Ryan Lee",
    "Ryan Scheel",
    "sapir",
    "sark",
    "Saverio Miroddi",
    "Schell Carl Scivally",
    "sdfgeoff",
    "Sergey Minakov",
    "simens_green",
    "simlay",
    "Simon Guillot",
    "Smite Rust",
    "speak",
    "Spencer Burris",
    "Squirrel",
    "StarArawn",
    "stefee",
    "Stjepan Glavina",
    "SvenTS",
    "szunami",
    "taryn",
    "TehPers",
    "Telzhaak",
    "TEMHOTAOKEAHA",
    "terrarier2111",
    "thebluefish",
    "Theia Vogel",
    "the-notable",
    "Théo Degioanni",
    "TheRawMeatball",
    "therealstork",
    "Thirds",
    "Thomas Heartman",
    "Thomas Herzog",
    "Tiago Ferreira",
    "tiagolam",
    "tigregalis",
    "Tomasz Sterna",
    "Tom Bebb",
    "Toniman20",
    "Toothbrush",
    "TotalKrill",
    "Tristan Pemble",
    "Utkarsh",
    "Valentin",
    "verzuz",
    "Victor \"multun\" Collod",
    "VitalyR",
    "Vladyslav Batyrenko",
    "VVishion",
    "walterpie",
    "Waridley",
    "W. Brian Gourlie",
    "Will Crichton",
    "Will Dixon",
    "Will Hart",
    "William Batista",
    "willolisp",
    "Wojciech Olejnik",
    "Wouter Buckens",
    "Wouter Standaert",
    "wyhaya",
    "Xavientois",
    "Yoh Deadfall",
    "Zach Gotsch",
    "Zaszi",
    "Zhixing Zhang",
    "Zicklag",
    "Zooce",
];

fn setup_contributor_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let contribs = contributors().unwrap_or_else(|_| {
        BTreeSet::from_iter(CONTRIBUTORS_LIST.iter().map(|name| name.to_string()))
    });

    let texture_handle = asset_server.load("branding/icon.png");

    let mut contributor_selection = ContributorSelection {
        order: vec![],
        idx: 0,
    };

    let mut rnd = rand::thread_rng();

    for name in contribs {
        let pos = (rnd.gen_range(-400.0..400.0), rnd.gen_range(0.0..400.0));
        let dir = rnd.gen_range(-1.0..1.0);
        let velocity = Vec3::new(dir * 500.0, 0.0, 0.0);
        let hue = rnd.gen_range(0.0..=360.0);

        // some sprites should be flipped
        let flipped = rnd.gen_bool(0.5);

        let transform = Transform::from_xyz(pos.0, pos.1, 0.0);

        let entity = commands
            .spawn()
            .insert_bundle((
                Contributor { hue },
                Velocity {
                    translation: velocity,
                    rotation: -dir * 5.0,
                },
            ))
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    size: Vec2::new(1.0, 1.0) * SPRITE_SIZE,
                    resize_mode: SpriteResizeMode::Manual,
                    flip_x: flipped,
                    ..Default::default()
                },
                material: materials.add(ColorMaterial {
                    color: Color::hsla(hue, SATURATION_DESELECTED, LIGHTNESS_DESELECTED, ALPHA),
                    texture: Some(texture_handle.clone()),
                }),
                transform,
                ..Default::default()
            })
            .id();

        contributor_selection.order.push((name, entity));
    }

    contributor_selection.order.shuffle(&mut rnd);

    commands.insert_resource(contributor_selection);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle((SelectTimer, Timer::from_seconds(SHOWCASE_TIMER_SECS, true)));

    commands
        .spawn()
        .insert(ContributorDisplay)
        .insert_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Contributor showcase".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        });
}

/// Finds the next contributor to display and selects the entity
fn select_system(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut contributor_selection: ResMut<ContributorSelection>,
    mut text_query: Query<&mut Text, With<ContributorDisplay>>,
    mut timer_query: Query<&mut Timer, With<SelectTimer>>,
    mut query: Query<(&Contributor, &Handle<ColorMaterial>, &mut Transform)>,
    time: Res<Time>,
) {
    let mut timer_fired = false;
    for mut timer in timer_query.iter_mut() {
        if !timer.tick(time.delta()).just_finished() {
            continue;
        }
        timer.reset();
        timer_fired = true;
    }

    if !timer_fired {
        return;
    }

    let prev = contributor_selection.idx;

    if (contributor_selection.idx + 1) < contributor_selection.order.len() {
        contributor_selection.idx += 1;
    } else {
        contributor_selection.idx = 0;
    }

    {
        let (_, entity) = &contributor_selection.order[prev];
        if let Ok((contributor, handle, mut transform)) = query.get_mut(*entity) {
            deselect(
                &mut *materials,
                handle.clone(),
                contributor,
                &mut *transform,
            );
        }
    }

    let (name, entity) = &contributor_selection.order[contributor_selection.idx];

    if let Ok((contributor, handle, mut transform)) = query.get_mut(*entity) {
        if let Some(mut text) = text_query.iter_mut().next() {
            select(
                &mut *materials,
                handle,
                contributor,
                &mut *transform,
                &mut *text,
                name,
            );
        }
    }
}

/// Change the modulate color to the "selected" colour,
/// bring the object to the front and display the name.
fn select(
    materials: &mut Assets<ColorMaterial>,
    material_handle: &Handle<ColorMaterial>,
    contributor: &Contributor,
    transform: &mut Transform,
    text: &mut Text,
    name: &str,
) -> Option<()> {
    let material = materials.get_mut(material_handle)?;
    material.color = Color::hsla(
        contributor.hue,
        SATURATION_SELECTED,
        LIGHTNESS_SELECTED,
        ALPHA,
    );

    transform.translation.z = 100.0;

    text.sections[0].value = "Contributor: ".to_string();
    text.sections[1].value = name.to_string();
    text.sections[1].style.color = material.color;

    Some(())
}

/// Change the modulate color to the "deselected" colour and push
/// the object to the back.
fn deselect(
    materials: &mut Assets<ColorMaterial>,
    material_handle: Handle<ColorMaterial>,
    contributor: &Contributor,
    transform: &mut Transform,
) -> Option<()> {
    let material = materials.get_mut(material_handle)?;
    material.color = Color::hsla(
        contributor.hue,
        SATURATION_DESELECTED,
        LIGHTNESS_DESELECTED,
        ALPHA,
    );

    transform.translation.z = 0.0;

    Some(())
}

/// Applies gravity to all entities with velocity
fn velocity_system(time: Res<Time>, mut velocity_query: Query<&mut Velocity>) {
    let delta = time.delta_seconds();

    for mut velocity in velocity_query.iter_mut() {
        velocity.translation += Vec3::new(0.0, GRAVITY * delta, 0.0);
    }
}

/// Checks for collisions of contributor-birds.
///
/// On collision with left-or-right wall it resets the horizontal
/// velocity. On collision with the ground it applies an upwards
/// force.
fn collision_system(
    windows: Res<Windows>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Contributor>>,
) {
    let mut rnd = rand::thread_rng();

    let window = windows.get_primary().unwrap();

    let ceiling = window.height() / 2.;
    let ground = -(window.height() / 2.);

    let wall_left = -(window.width() / 2.);
    let wall_right = window.width() / 2.;

    for (mut velocity, mut transform) in query.iter_mut() {
        let left = transform.translation.x - SPRITE_SIZE / 2.0;
        let right = transform.translation.x + SPRITE_SIZE / 2.0;
        let top = transform.translation.y + SPRITE_SIZE / 2.0;
        let bottom = transform.translation.y - SPRITE_SIZE / 2.0;

        // clamp the translation to not go out of the bounds
        if bottom < ground {
            transform.translation.y = ground + SPRITE_SIZE / 2.0;
            // apply an impulse upwards
            velocity.translation.y = rnd.gen_range(700.0..1000.0);
        }
        if top > ceiling {
            transform.translation.y = ceiling - SPRITE_SIZE / 2.0;
        }
        // on side walls flip the horizontal velocity
        if left < wall_left {
            transform.translation.x = wall_left + SPRITE_SIZE / 2.0;
            velocity.translation.x *= -1.0;
            velocity.rotation *= -1.0;
        }
        if right > wall_right {
            transform.translation.x = wall_right - SPRITE_SIZE / 2.0;
            velocity.translation.x *= -1.0;
            velocity.rotation *= -1.0;
        }
    }
}

/// Apply velocity to positions and rotations.
fn move_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    let delta = time.delta_seconds();

    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += delta * velocity.translation;
        transform.rotate(Quat::from_rotation_z(velocity.rotation * delta));
    }
}

/// Get the names of all contributors from the git log.
///
/// The names are deduplicated.
/// This function only works if `git` is installed and
/// the program is run through `cargo`.
fn contributors() -> Result<Contributors, LoadContributorsError> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(LoadContributorsError::Var)?;

    let mut cmd = std::process::Command::new("git")
        .args(&["--no-pager", "log", "--pretty=format:%an"])
        .current_dir(manifest_dir)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(LoadContributorsError::IO)?;

    let stdout = cmd.stdout.take().ok_or(LoadContributorsError::Stdout)?;

    let contributors = BufReader::new(stdout)
        .lines()
        .filter_map(|x| x.ok())
        .collect();

    Ok(contributors)
}
