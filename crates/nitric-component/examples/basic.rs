use nitric_component::{
    error::OomError,
    id::CheckedId,
    impls::{FlatAllocator, FlatUsize},
    prelude::*,
};

#[derive(Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

fn create_positions() -> Storage<FlatUsize, Position> {
    Storage::new()
}

#[derive(Debug)]
pub struct Rotation {
    axis: [f32; 3],
    angle: f32,
}

fn create_rotations() -> Storage<FlatUsize, Rotation> {
    Storage::new()
}

fn create_id_set<'merger>(
    allocator: &mut FlatAllocator,
    merger: &'merger Merger<FlatAllocator>,
) -> Result<Vec<CheckedId<'merger, FlatUsize>>, OomError> {
    (0..100).map(|_| allocator.create_checked(merger)).collect()
}

fn main() {
    let (mut allocator, mut merger) = FlatAllocator::new();

    let mut positions = create_positions();
    let mut rotations = create_rotations();

    let id_set = match create_id_set(&mut allocator, &merger) {
        Ok(id_set) => id_set,
        Err(_) => {
            eprintln!("Out of Memory!");
            return;
        }
    };

    for id in id_set.clone() {
        positions.insert(id, Position { x: 2, y: -5 });

        rotations.insert(
            id,
            Rotation {
                axis: [0.0, 1.0, 0.0],
                angle: 1.14,
            },
        );
    }

    println!("Position at third ID: {:?}", positions.get(&id_set[3]));
    println!("Rotation at last ID: {:?}", rotations.get(&id_set[99]));

    // Let's delete half of the IDs again
    id_set.iter().skip(50).for_each(|id| allocator.delete(id));

    // Delete all IDs that were flagged
    allocator.merge_deleted(&mut merger);

    // Now, `id_set` cannot be used anymore; using it would not compile
    // TODO: allow to iterate all valid IDs + show how to still use the first half
}
