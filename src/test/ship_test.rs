/**
    Testing of the Ship object.
*/

#[cfg(test)]

mod tests {

    #[test]
    fn ship_constructs() {
        use ship::*;
        use tile::*;

        let test_tiles =
        vec![
        vec![0,0,1,0,0],
        vec![0,1,1,1,0],
        vec![0,1,2,1,0],
        vec![1,1,1,1,1],
        ];

        let test_ship = Ship::new(test_tiles);

        // Check ship's width, height, and layout.
        assert_eq!(test_ship.width, 5.0);
        assert_eq!(test_ship.height, 4.0);
        // Flipped 90 degrees left.
        assert_eq!(test_ship.tiles[2][0].material, TileMaterial::Wood);
        assert_eq!(test_ship.tiles[0][0].tile_type, TileType::Special);

        drop(test_ship);
    }

}
