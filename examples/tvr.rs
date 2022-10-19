extern crate dvos3binding;
use nalgebra::{Isometry3};
use cxx::{let_cxx_string};


fn main() {
    // let tvr = dvos3binding::ffi::new_two_view_reconstruction(1.0, 1.0, 1.0, 1.0, 1.0, 200);
    // let test = dvos3binding::ffi::test();
    let test2 = dvos3binding::ffi::test2();

    // let pose = Pose::default();
    // let_cxx_string!(vertex_name = "VertexSE3Expmap");

    // g2orust::ffi::create_frame_vertex(
    //     1, &vertex_name, 
    //     pose.t_to_array(), pose.r_quaternion_to_array(), 
    //     optimizer_ptr
    // );

}
