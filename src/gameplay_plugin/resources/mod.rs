use std::cmp::{max, min, Ordering};

use bevy::{
    prelude::{Entity, Resource},
    reflect::Reflect,
};
use radius_and_amount_tiles::{AmountTiles, Radius};

use self::radius_and_amount_tiles::InvalidTileAmount;

use super::components::AxialCoordinates;

pub mod radius_and_amount_tiles;

fn tiles_lower_rows(radius: Radius, lower_rows: u32) -> usize {
    let radius = radius as u32;
    // These calculations derive from the assumption that we can split a hexagonal into two areas, the lower rows (excluding the middle row) and the upper rows (including the middle row).
    // To get the tiles of the lower rows we have the following formula:
    // Row at: -radius     -radius + 1          -radius + lower_rows
    //            |             |                       |
    //            V             v                       v
    //      (radius + 1) + (radius + 2) + ... + (radius + lower_rows) = lower_rows * radius + 1 + 2 + ... + lower_rows = lower_rows * radius + lower_rows * (lower_rows + 1) / 2) (Gaussian formula)
    // This effectively yields a formula where the first part (lower_rows * radius) gives us a rectangle of tiles smaller than the one lower rows we look at. The second part (lower_rows * (lower_rows + 1) / 2)) basically yields the triangle of the missing tiles.
    let tiles_lower_rows = lower_rows * radius + lower_rows * (lower_rows + 1) / 2;
    tiles_lower_rows as usize
}

fn tiles_upper_rows(radius: Radius, upper_rows: u32) -> usize {
    let radius = radius as u32;
    let diameter = 2 * radius + 1;

    // This formula is very similar to the one in [tiles_till_lower_rows] but instead of starting at the row with the fewest tiles we start at the row with the most tiles (thicc_row). This is the row at `r = -1`. This means that we cut out / subtract the none-existent triangle of tiles from the rectangle larger than the upper rows we look at (upper_rows * (diameter + 1)) with the Gaussian formula (upper_rows * (upper_rows + 1) / 2).
    // We have to add one to `thicc_row_tiles` because the Gaussian formula starts at 1 (1 + 2 + ... + n) not at 0 (0 + 1 + 2 + ... + n).
    let tiles_upper_rows = upper_rows * (diameter + 1) - upper_rows * (upper_rows + 1) / 2;
    tiles_upper_rows as usize
}

fn tiles_till_row(radius: Radius, r: i32) -> usize {
    let lower_rows = radius as i32 - i32::max(0, r); // This value will be in range `0..=radius + 1` since the middle row is part of the lower rows.
    let upper_rows = (i32::min(0, r)).unsigned_abs(); // This value will be in range `0..=radius - 1` since the middle row is not part of the upper rows.
    tiles_lower_rows(radius, lower_rows as u32) + tiles_upper_rows(radius, upper_rows)
}

fn coordinates_to_index(radius: Radius, coordinates: AxialCoordinates) -> usize {
    let signed_radius = match coordinates.r().cmp(&0) {
        Ordering::Less | Ordering::Equal => -(radius as i32),
        Ordering::Greater => radius as i32,
    };

    // Normalized in the sense that the first tile in the row is always assigned offset 0 (not say -2 or 5).
    let normalized_offset = (coordinates.q() + signed_radius).unsigned_abs() as usize;

    let current_row_start = tiles_till_row(radius, coordinates.r());

    current_row_start + normalized_offset
}

fn index_to_coordinates(radius: Radius, index: usize) -> AxialCoordinates {
    // Calculate the number of tiles in lower rows
    let all_tiles_lower_rows = tiles_lower_rows(radius, radius as u32);

    let lower_rows_tiles = min(all_tiles_lower_rows, index);

    // Calculate the number of tiles in upper rows
    let upper_rows_tiles = max(all_tiles_lower_rows, index) - all_tiles_lower_rows;

    // Use radius as f64 for floating-point calculations
    let radius_f64 = radius as u8 as f64;

    // Inverse function of `tiles_lower_rows`
    let lower_rows = ((-2.0 * radius_f64 - 1.0
        + (4.0 * radius_f64 * radius_f64 + 4.0 * radius_f64 + 1.0 + 8.0 * lower_rows_tiles as f64)
            .sqrt())
        / 2.0) as u32;

    // Inverse function of `tiles_upper_rows`
    let upper_rows = ((3.0 + 4.0 * radius_f64
        - (9.0 + 24.0 * radius_f64 + 16.0 * radius_f64 * radius_f64
            - 8.0 * upper_rows_tiles as f64)
            .sqrt())
        / 2.0) as u32;

    // Calculate the total number of tiles till the row
    let tiles_till_row =
        tiles_lower_rows(radius, lower_rows) + tiles_upper_rows(radius, upper_rows);

    // Calculate signed radius adjustment
    let positive_if_tile_in_upper_row = index as i32 - all_tiles_lower_rows as i32;

    // Calculate final coordinates
    let offset_from_row_start = index - tiles_till_row;

    let q = offset_from_row_start as i32 - (radius as i32);

    let q = match positive_if_tile_in_upper_row.cmp(&0) {
        Ordering::Less => q,
        Ordering::Greater | Ordering::Equal => -q,
    };

    let r = radius as i32 - lower_rows as i32 - upper_rows as i32;

    AxialCoordinates::new(q, r)
}

pub type TileEntity = Entity;
pub type TileConnectionEntity = Entity;

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    tile_entity: TileEntity,
    tile_connection_right_entity: TileConnectionEntity,
    tile_connection_lower_right_entity: TileConnectionEntity,
    tile_connection_lower_left_entity: TileConnectionEntity,
}

impl Tile {
    pub fn new(
        tile_entity: TileEntity,
        tile_connection_right_entity: TileConnectionEntity,
        tile_connection_lower_right_entity: TileConnectionEntity,
        tile_connection_lower_left_entity: TileConnectionEntity,
    ) -> Self {
        Self {
            tile_entity,
            tile_connection_right_entity,
            tile_connection_lower_right_entity,
            tile_connection_lower_left_entity,
        }
    }

    pub fn tile_connection_right_entity(&self) -> Entity {
        self.tile_connection_right_entity
    }

    pub fn tile_connection_lower_right_entity(&self) -> Entity {
        self.tile_connection_lower_right_entity
    }

    pub fn tile_connection_lower_left_entity(&self) -> Entity {
        self.tile_connection_lower_left_entity
    }
}

/// This data structure represents a hexagonal map made up of hexagons.
/// It allows you to retrieve hex tile entities by their coordinates.
/// Since this is the heart of this game I wanted to design it in a very performant manner (and because it is fun).
/// Otherwise I could have just used a hashmap.
/// Internally this data structure stores tiles in a flat vec without empty gaps.
/// To retrieve tiles it maps to coordinates to unique indices without gaps in constant time complexity, avoiding expensive branches as far as it's possible and so on.
#[derive(Resource, Reflect, Debug, Clone, PartialEq, Eq)]
pub struct HexagonalMap<T> {
    tiles: Vec<T>,
    radius: Radius,
}

impl<T> HexagonalMap<T> {
    /// Retrieve a tile by its axial coordinates.
    /// The time complexity of this function is O(1) and it is additionally somewhat optimized to avoid branching and other expensive operations to the best of my abilities.
    ///
    /// ## Safety
    /// This function does no out-of-bounds checking.
    /// If the distance of the coordinates from the center of the map is greater than its radius ([HexagonalMap::radius()]) it will result in undefined behavior!
    pub unsafe fn get_unchecked(&self, coordinates: AxialCoordinates) -> &T {
        let index = coordinates_to_index(self.radius, coordinates);
        self.tiles.get_unchecked(index)
    }

    /// Retrieve a tile by its axial coordinates.
    /// The time complexity of this function is O(1) and it is additionally somewhat optimized to avoid branching and other expensive operations to the best of my abilities.
    ///
    /// ## Returns
    /// - Some(&T): If the distance of the coordinates from the center of the map is less or equal than its radius ([HexagonalMap::radius()]).
    /// - None: If the distance of the coordinates from the center of the map is greater than its radius ([HexagonalMap::radius()]).
    pub fn get(&self, coordinates: AxialCoordinates) -> Option<&T> {
        if coordinates.distance_to_origin() > self.radius as u32 {
            return None;
        }
        Some(unsafe { self.get_unchecked(coordinates) })
    }

    /// Convert a vec to [HexagonalMap].
    ///
    /// ## Returns
    /// - None: If there where to many or to few tiles for a hexagonal map with a max radius of 255.
    /// - Some: Otherwise
    pub fn from_vec(vec: Vec<T>) -> Result<HexagonalMap<T>, InvalidTileAmount> {
        let amount_tiles: AmountTiles = vec.len().try_into()?;
        Ok(Self {
            tiles: vec,
            radius: amount_tiles.into(),
        })
    }

    pub fn iter_with_coordinates(self) -> impl Iterator<Item = (T, AxialCoordinates)> {
        self.tiles
            .into_iter()
            .enumerate()
            .map(move |(index, tile)| (tile, index_to_coordinates(self.radius, index)))
    }

    pub fn into_vec(self) -> Vec<T> {
        self.tiles
    }

    pub fn tiles(&self) -> &[T] {
        &self.tiles
    }

    pub fn radius(&self) -> Radius {
        self.radius
    }
}

impl<T> TryFrom<Vec<T>> for HexagonalMap<T> {
    type Error = InvalidTileAmount;

    /// Retrieve a tile by its axial coordinates.
    /// The time complexity of this function is O(1) and it is additionally somewhat optimized to avoid branching and other expensive operations to the best of my abilities.
    ///
    /// ## Returns
    /// - Some(&T): If the distance of the coordinates from the center of the map is less or equal than its radius ([HexagonalMap::radius()]).
    /// - None: If the distance of the coordinates from the center of the map is greater than its radius ([HexagonalMap::radius()]).
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Self::from_vec(value)
    }
}

impl<T> From<HexagonalMap<T>> for Vec<T> {
    fn from(val: HexagonalMap<T>) -> Self {
        val.into_vec()
    }
}

#[cfg(test)]
mod tests_hexagonal_map {

    use crate::gameplay_plugin::{
        components::AxialCoordinates,
        resources::{
            coordinates_to_index, index_to_coordinates, radius_and_amount_tiles::InvalidTileAmount,
        },
    };

    use super::{radius_and_amount_tiles::Radius, HexagonalMap};

    // A Vec of radius to coordinates, where each coordinate is at its correct index.
    fn coordinates() -> Vec<(Radius, Vec<AxialCoordinates>)> {
        vec![
            (Radius::Radius0, vec![AxialCoordinates::new(0, 0)]),
            (
                Radius::Radius1,
                vec![
                    // r = 1
                    AxialCoordinates::new(-1, 1),
                    AxialCoordinates::new(0, 1),
                    // r = 0
                    AxialCoordinates::new(1, 0),
                    AxialCoordinates::new(0, 0),
                    AxialCoordinates::new(-1, 0),
                    // r = -1
                    AxialCoordinates::new(1, -1),
                    AxialCoordinates::new(0, -1),
                ],
            ),
            (
                Radius::Radius2,
                vec![
                    // r = 2
                    AxialCoordinates::new(-2, 2),
                    AxialCoordinates::new(-1, 2),
                    AxialCoordinates::new(0, 2),
                    // r = 1
                    AxialCoordinates::new(-2, 1),
                    AxialCoordinates::new(-1, 1),
                    AxialCoordinates::new(0, 1),
                    AxialCoordinates::new(1, 1),
                    // r = 0
                    AxialCoordinates::new(2, 0),
                    AxialCoordinates::new(1, 0),
                    AxialCoordinates::new(0, 0),
                    AxialCoordinates::new(-1, 0),
                    AxialCoordinates::new(-2, 0),
                    // r = -1
                    AxialCoordinates::new(2, -1),
                    AxialCoordinates::new(1, -1),
                    AxialCoordinates::new(0, -1),
                    AxialCoordinates::new(-1, -1),
                    // r = -2
                    AxialCoordinates::new(2, -2),
                    AxialCoordinates::new(1, -2),
                    AxialCoordinates::new(0, -2),
                ],
            ),
            (
                Radius::Radius3,
                vec![
                    // r = 3
                    AxialCoordinates::new(-3, 3),
                    AxialCoordinates::new(-2, 3),
                    AxialCoordinates::new(-1, 3),
                    AxialCoordinates::new(0, 3),
                    // r = 2
                    AxialCoordinates::new(-3, 2),
                    AxialCoordinates::new(-2, 2),
                    AxialCoordinates::new(-1, 2),
                    AxialCoordinates::new(0, 2),
                    AxialCoordinates::new(1, 2),
                    // r = 1
                    AxialCoordinates::new(-3, 1),
                    AxialCoordinates::new(-2, 1),
                    AxialCoordinates::new(-1, 1),
                    AxialCoordinates::new(0, 1),
                    AxialCoordinates::new(1, 1),
                    AxialCoordinates::new(2, 1),
                    // r = 0
                    AxialCoordinates::new(3, 0),
                    AxialCoordinates::new(2, 0),
                    AxialCoordinates::new(1, 0),
                    AxialCoordinates::new(0, 0),
                    AxialCoordinates::new(-1, 0),
                    AxialCoordinates::new(-2, 0),
                    AxialCoordinates::new(-3, 0),
                    // r = -1
                    AxialCoordinates::new(3, -1),
                    AxialCoordinates::new(2, -1),
                    AxialCoordinates::new(1, -1),
                    AxialCoordinates::new(0, -1),
                    AxialCoordinates::new(-1, -1),
                    AxialCoordinates::new(-2, -1),
                    // r = -2
                    AxialCoordinates::new(3, -2),
                    AxialCoordinates::new(2, -2),
                    AxialCoordinates::new(1, -2),
                    AxialCoordinates::new(0, -2),
                    AxialCoordinates::new(-1, -2),
                    // r = -3
                    AxialCoordinates::new(3, -3),
                    AxialCoordinates::new(2, -3),
                    AxialCoordinates::new(1, -3),
                    AxialCoordinates::new(0, -3),
                ],
            ),
        ]
    }

    fn coordinates_out_of_bounds() -> Vec<(Radius, Vec<AxialCoordinates>)> {
        vec![
            (
                Radius::Radius0,
                vec![
                    // r = 1
                    AxialCoordinates::new(-1, 1),
                    AxialCoordinates::new(0, 1),
                    // r = 0
                    AxialCoordinates::new(1, 0),
                    // AxialCoordinates::new( 0,0), // I comment out every in bounds coordinates, but leave them in for reference.
                    AxialCoordinates::new(-1, 0),
                    // r = -1
                    AxialCoordinates::new(1, -1),
                    AxialCoordinates::new(0, -1),
                ],
            ),
            (
                Radius::Radius1,
                vec![
                    // r = 2
                    AxialCoordinates::new(-2, 2),
                    AxialCoordinates::new(-1, 2),
                    AxialCoordinates::new(0, 2),
                    // r = 1
                    AxialCoordinates::new(-2, 1),
                    // AxialCoordinates::new( -1,1),
                    // AxialCoordinates::new( 0,1),
                    AxialCoordinates::new(1, 1),
                    // r = 0
                    AxialCoordinates::new(2, 0),
                    // AxialCoordinates::new( 1,0),
                    // AxialCoordinates::new( 0,0),
                    // AxialCoordinates::new( -1,0),
                    AxialCoordinates::new(-2, 0),
                    // r = -1
                    AxialCoordinates::new(2, -1),
                    // AxialCoordinates::new( 1,-1),
                    // AxialCoordinates::new( 0,-1),
                    AxialCoordinates::new(-1, -1),
                    // r = -2
                    AxialCoordinates::new(2, -2),
                    AxialCoordinates::new(1, -2),
                    AxialCoordinates::new(0, -2),
                ],
            ),
            (
                Radius::Radius2,
                vec![
                    // r = 3
                    AxialCoordinates::new(-3, 3),
                    AxialCoordinates::new(-2, 3),
                    AxialCoordinates::new(-1, 3),
                    AxialCoordinates::new(0, 3),
                    // r = 2
                    AxialCoordinates::new(-3, 2),
                    // AxialCoordinates::new( -2,2),
                    // AxialCoordinates::new( -1,2),
                    // AxialCoordinates::new( 0,2),
                    AxialCoordinates::new(1, 2),
                    // r = 1
                    AxialCoordinates::new(-3, 1),
                    // AxialCoordinates::new( -2,1),
                    // AxialCoordinates::new( -1,1),
                    // AxialCoordinates::new( 0,1),
                    // AxialCoordinates::new( 1,1),
                    AxialCoordinates::new(2, 1),
                    // r = 0
                    AxialCoordinates::new(3, 0),
                    // AxialCoordinates::new( 2,0),
                    // AxialCoordinates::new( 1,0),
                    // AxialCoordinates::new( 0,0),
                    // AxialCoordinates::new( -1,0),
                    // AxialCoordinates::new( -2,0),
                    AxialCoordinates::new(-3, 0),
                    // r = -1
                    AxialCoordinates::new(3, -1),
                    // AxialCoordinates::new( 2,-1),
                    // AxialCoordinates::new( 1,-1),
                    // AxialCoordinates::new( 0,-1),
                    // AxialCoordinates::new( -1,-1),
                    AxialCoordinates::new(-2, -1),
                    // r = -2
                    AxialCoordinates::new(3, -2),
                    // AxialCoordinates::new( 2,-2),
                    // AxialCoordinates::new( 1,-2),
                    // AxialCoordinates::new( 0,-2),
                    AxialCoordinates::new(-1, -2),
                    // r = -3
                    AxialCoordinates::new(3, -3),
                    AxialCoordinates::new(2, -3),
                    AxialCoordinates::new(1, -3),
                    AxialCoordinates::new(0, -3),
                ],
            ),
            (
                Radius::Radius3,
                vec![
                    // r = 4
                    AxialCoordinates::new(-4, 4),
                    AxialCoordinates::new(-3, 4),
                    AxialCoordinates::new(-2, 4),
                    AxialCoordinates::new(-1, 4),
                    AxialCoordinates::new(0, 4),
                    // r = 3
                    AxialCoordinates::new(-4, 3),
                    // AxialCoordinates::new( -3,3),
                    // AxialCoordinates::new( -2,3),
                    // AxialCoordinates::new( -1,3),
                    // AxialCoordinates::new( 0,3),
                    AxialCoordinates::new(1, 3),
                    // r = 2
                    AxialCoordinates::new(-4, 2),
                    // AxialCoordinates::new( -3,2),
                    // AxialCoordinates::new( -2,2),
                    // AxialCoordinates::new( -1,2),
                    // AxialCoordinates::new( 0,2),
                    // AxialCoordinates::new( 1,2),
                    AxialCoordinates::new(2, 2),
                    // r = 1
                    AxialCoordinates::new(-4, 1),
                    // AxialCoordinates::new( -3,1),
                    // AxialCoordinates::new( -2,1),
                    // AxialCoordinates::new( -1,1),
                    // AxialCoordinates::new( 0,1),
                    // AxialCoordinates::new( 1,1),
                    // AxialCoordinates::new( 2,1),
                    AxialCoordinates::new(3, 1),
                    // r = 0
                    AxialCoordinates::new(4, 0),
                    // AxialCoordinates::new( 3,0),
                    // AxialCoordinates::new( 2,0),
                    // AxialCoordinates::new( 1,0),
                    // AxialCoordinates::new( 0,0),
                    // AxialCoordinates::new( -1,0),
                    // AxialCoordinates::new( -2,0),
                    // AxialCoordinates::new( -3,0),
                    AxialCoordinates::new(-4, 0),
                    // r = -1
                    AxialCoordinates::new(4, -1),
                    // AxialCoordinates::new( 3,-1),
                    // AxialCoordinates::new( 2,-1),
                    // AxialCoordinates::new( 1,-1),
                    // AxialCoordinates::new( 0,-1),
                    // AxialCoordinates::new( -1,-1),
                    // AxialCoordinates::new( -2,-1),
                    AxialCoordinates::new(-3, -1),
                    // r = -2
                    AxialCoordinates::new(4, -2),
                    // AxialCoordinates::new( 3,-2),
                    // AxialCoordinates::new( 2,-2),
                    // AxialCoordinates::new( 1,-2),
                    // AxialCoordinates::new( 0,-2),
                    // AxialCoordinates::new( -1,-2),
                    AxialCoordinates::new(-2, -2),
                    // r = -3
                    AxialCoordinates::new(4, -3),
                    // AxialCoordinates::new( 3,-3),
                    // AxialCoordinates::new( 2,-3),
                    // AxialCoordinates::new( 1,-3),
                    // AxialCoordinates::new( 0,-3),
                    AxialCoordinates::new(-1, -3),
                    // r = -4
                    AxialCoordinates::new(4, -4),
                    AxialCoordinates::new(3, -4),
                    AxialCoordinates::new(2, -4),
                    AxialCoordinates::new(1, -4),
                    AxialCoordinates::new(0, -4),
                ],
            ),
        ]
    }

    #[test]
    fn test_coordinates_to_index() {
        for (radius, coordinates) in coordinates() {
            for (index, coordinates) in coordinates.into_iter().enumerate() {
                let actual_index = coordinates_to_index(radius, coordinates);
                assert_eq!(actual_index, index, "{coordinates:?}");
            }
        }
    }

    #[test]
    fn test_get_unchecked() {
        for (radius, coordinates) in coordinates() {
            let hexagonal_map = HexagonalMap {
                tiles: coordinates.clone(),
                radius,
            };
            for coordinates in coordinates {
                assert_eq!(
                    unsafe { hexagonal_map.get_unchecked(coordinates) },
                    &coordinates
                );
            }
        }
    }

    #[test]
    fn test_get() {
        for ((radius, coordinates), (_, out_of_bounds_coordinates)) in
            coordinates().into_iter().zip(coordinates_out_of_bounds())
        {
            let hexagonal_map = HexagonalMap {
                tiles: coordinates.clone(),
                radius,
            };

            for coordinates in coordinates {
                assert_eq!(hexagonal_map.get(coordinates), Some(&coordinates));
            }

            for out_of_bounds_coordinates in out_of_bounds_coordinates {
                assert_eq!(hexagonal_map.get(out_of_bounds_coordinates), None);
            }
        }
    }

    #[test]
    fn test_index_to_coordinates() {
        for (radius, coordinates) in coordinates() {
            for (index, coordinates) in coordinates.into_iter().enumerate() {
                let actual_coordinates = index_to_coordinates(radius, index);
                assert_eq!(actual_coordinates, coordinates, "Index: {index:?}");
            }
        }
    }

    #[test]
    fn test_from_vec() {
        for (radius, coordinates) in coordinates() {
            assert_eq!(
                HexagonalMap::try_from(coordinates.clone()),
                Ok(HexagonalMap {
                    tiles: coordinates,
                    radius
                })
            );

            let next_radius = Radius::from(radius as u8 + 1);
            for amount_tiles in
                (radius.into_amount_tiles() as usize + 1)..next_radius.into_amount_tiles() as usize
            {
                assert_eq!(
                    HexagonalMap::try_from(vec![0; amount_tiles]),
                    Err(InvalidTileAmount(amount_tiles))
                );
            }
        }
    }
}
