use super::{MetaBuilder, common};

pub struct RoomCorridors;

impl RoomCorridors {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl MetaBuilder for RoomCorridors {
    fn build(&mut self, rng: &mut crate::random::PRng, build_data: &mut super::BuildData) {
        let rooms = {
            if let Some(ref existing_rooms) = build_data.rects {
                existing_rooms.clone()
            } else {
                panic!(
                    "{} requires that BuildData rects is not None {build_data:?}",
                    std::any::type_name::<RoomCorridors>()
                );
            }
        };

        // Consider the first room as connected (it should always exist).
        let mut connected: Vec<usize> = vec![0];

        // All other rooms start out disconnected.
        let mut disconnected: Vec<usize> = Vec::from_iter(1..rooms.len());
        let mut corridors: Vec<Vec<usize>> = Vec::new();

        while !disconnected.is_empty() {

            // Find closest match between connected and disconnected.
            let (closest_connected, closest_disconnected) = connected
                .iter()
                .enumerate()
                .flat_map(|c| std::iter::repeat(c).zip(disconnected.iter().enumerate()))
                .min_by_key(|&((_, &c_room), (_, &d_room))| {
                    let c_center = rooms[c_room].center();
                    let d_center = rooms[d_room].center();
                    c_center.dist_manhattan(d_center)
                })
                .map(|((ci, _), (di, _))| (ci, di))
                .unwrap();

            // Connected the closest connected and disconnected rooms.
            let closest_connected_center = rooms[connected[closest_connected]].center();
            let closest_disconnected_center = rooms[disconnected[closest_disconnected]].center();

            corridors.push(common::draw_corridor(
                &mut build_data.board,
                closest_connected_center,
                closest_disconnected_center,
                rng.gen::<bool>(),
            ));

            // Transfer newly-connected room index from disconnected to connected.
            connected.push(disconnected.remove(closest_disconnected));

            build_data.take_snapshot();
        }

        build_data.corridors = Some(corridors);
    }
}
