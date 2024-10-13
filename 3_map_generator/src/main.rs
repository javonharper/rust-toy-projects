use bevy::{
    prelude::*,
    render::{
        camera::{Camera, PerspectiveProjection},
        mesh::Indices,
        pipeline::PrimitiveTopology,
    },
};
use voronoice::*;

fn main() {
    // voronoi sites
    let sites = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 0.0, y: 1.0 },
    ];

    // builds a voronoi diagram from the set of sites above, bounded by a square of size 4
    let my_voronoi = VoronoiBuilder::default()
        .set_sites(sites)
        .set_bounding_box(BoundingBox::new_centered_square(4.0))
        .set_lloyd_relaxation_iterations(5)
        .build()
        .unwrap();

    // inspect cells through iterators
    my_voronoi.iter_cells().for_each(|cell| {
        println!(
            "Vertices of cell: {:?}",
            cell.iter_vertices().collect::<Vec<&Point>>()
        )
    });

    // or probe cells individually
    let my_cell = my_voronoi.cell(1);
    println!(
        "Second cell has site {:?}, voronoi vertices {:?} and delaunay triangles {:?}",
        my_cell.site_position(),
        my_cell.iter_vertices().collect::<Vec<&Point>>(),
        my_cell.triangles().iter().collect::<Vec<&usize>>()
    );

    // or, for graphical applications, that benefit from index buffers
    // you can access the raw, indexed data
    let all_voronoi_cell_vertices = my_voronoi.vertices();
    let indexed_voronoi_cells = my_voronoi.cells();
    println!(
        "The first vertex position for the first voronoi cell is at {:?}",
        all_voronoi_cell_vertices[indexed_voronoi_cells[0][0]]
    );
}
