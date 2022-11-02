extern crate dvos3binding;
use nalgebra::{Isometry3};
use cxx::{let_cxx_string, CxxVector};

use opencv::core::{CV_8UC2, CV_8UC1};
use opencv::prelude::*;
use opencv::types::VectorOfKeyPoint;

use std::pin::Pin;

fn main() {
    let mut matcher = dvos3binding::ffi::new_orb_matcher(48, 48, 0.0, 0.0, 600.0, 600.0,0.1,true);
    
    let mut kps_cv1 = opencv::types::VectorOfKeyPoint::default();
    let mut kps_cv2 = opencv::types::VectorOfKeyPoint::default();

    let mut matches_cv = opencv::types::VectorOfi32::default();

    let mut grid_cv = opencv::types::VectorOfi32::default();

    let mut prevMat_cv = opencv::types::VectorOfPoint2f::default();

    for i in 1..10
    {
        kps_cv1.push(opencv::core::KeyPoint::new_coords(i as f32 *5.0, 1.0/i as f32 *3.0, 5.0, 1.4, 1.1, 3, 0).unwrap());

        kps_cv2.push(opencv::core::KeyPoint::new_coords(i as f32 *5.0, 1.0/i as f32 *3.0, 5.0, 1.4, 1.1, 3, 0).unwrap());

        matches_cv.push(i);
    }

    //let desc1 = opencv::core::Mat::eye(4,4,CV_8UC1).unwrap().to_mat().unwrap();
    //let desc2 = opencv::core::Mat::eye(4,4,CV_8UC1).unwrap().to_mat().unwrap();
    
    let desc1 =opencv::core::Mat::zeros(10, 32, CV_8UC1).unwrap().to_mat().unwrap();
    let desc2 =opencv::core::Mat::ones(10, 32, CV_8UC1).unwrap().to_mat().unwrap();
    
    unsafe{

        let  kps1cv = kps_cv1.into_raw() as *const CxxVector<dvos3binding::ffi::DVKeyPoint>;
        let  kps2cv = kps_cv2.into_raw() as *const CxxVector<dvos3binding::ffi::DVKeyPoint>;


        let  desc1cv = desc1.into_raw() as *const dvos3binding::ffi::DVMat;
        let  desc2cv = desc2.into_raw() as *const dvos3binding::ffi::DVMat;


        let grid_f2 = grid_cv.into_raw() as *const dvos3binding::ffi::DVGrid;
        

        let mut prevMatch  = dvos3binding::ffi::VectorOfDVPoint2f{vec:Vec::new() };
        let mut matches_new = dvos3binding::ffi::VectorOfDVi32{vec:Vec::new() };


        matcher.pin_mut().SearchForInitialization_1(
            &*kps1cv,
            &*kps2cv,
            &*desc1cv,
            &*desc2cv,
            &*grid_f2,
            &mut prevMatch,
            &mut matches_new, 
            100
            );

            // F1_mvKeysUn : &CxxVector<DVKeyPoint>, 
            // F2_mvKeysUn: &CxxVector<DVKeyPoint>, 
            // F1_mDescriptors: &DVMat,
            // F2_mDescriptors: &DVMat ,
            // F2_grid: &DVGrid,
            //  vbPrevMatched : &mut VectorOfDVPoint2f, 
            // vnMatches12 :  &mut VectorOfDVi32, 
            // windowSize: i32

        //print!("{:?}", &(*vP3D).vec.len());
        println!("DVPoint3f");
        // for val in vP3D.vec
        // {
        //     print!("{:?}", val)
        // }
        // println!("vbTriangulated");
        // for val in vbTriangulated.vec
        // {
        //     print!("{:?}", val)
        // }
        
        // print!("\nPose{:?}", pose.pose)
    }


}
