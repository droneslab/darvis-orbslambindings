use opencv::{
    prelude::*, core::*,
    types::{VectorOfKeyPoint, Point3f},
};

#[cxx::bridge(namespace = "orb_slam3")]
pub mod ffi {
    // Shared structs with fields visible to both languages.

    unsafe extern "C++" {
        // Opaque types which both languages can pass around
        // but only C++ can see the fields.
        type TwoViewReconstruction;

        fn new_two_view_reconstruction(
            fx: f32,
            cx: f32,
            fy: f32,
            cy: f32,
            sigma: f32,
            iterations: i32
        ) -> UniquePtr<TwoViewReconstruction>;

        fn Reconstruct(
            self: &TwoViewReconstruction,
            vKeys1: opencv::types::VectorOfKeyPoint,
            vKeys2: opencv::types::VectorOfKeyPoint,
            &vMatches12: Vec<i32>,
            &T21: pose,
            &vP3D: opencv::types::VectorOfPoint3f,
            &vbTriangulated: Vec<bool>
        ) -> bool;
    }
}
