#![allow(unused_imports)]
use num_complex::Complex64;
use std::cmp::min;
use std::time::Instant;
use std::fs::File;
use std::io;
use std::path::Path;
use rayon::prelude::*;

use si_scale::helpers::{seconds, number_};
use conv::prelude::* ;

    pub type RowContents = (Vec<u64>, Vec<Complex64>) ;

    pub fn append_rc(lhs : &mut RowContents, colv : &[u64], datav : &[Complex64]) {
	colv.iter().for_each(|v| lhs.0.push(*v)) ;
	datav.iter().for_each(|v| lhs.1.push(*v)) ;
    }

    pub fn append_rc1(lhs : &mut RowContents, colv : &[u64], datav : &[Complex64]) {
	lhs.0.copy_from_slice(colv) ;
	lhs.1.copy_from_slice(datav) ;
    }

pub fn hack_make_row(size : u64, rowind : u64)
		-> RowContents
{
    let recoeff = f64::value_from(rowind).unwrap() ;
    let coeff = Complex64::new(recoeff, 0.0) ;
    let mut colw = vec![rowind; 3000 * size as usize] ;
    let mut dataw = vec![coeff; 3000 * size as usize] ;
    (colw, dataw)
}


pub fn hack_chunks(dim : usize,
		   step : usize,
) -> () {

    let debug = true ;
    let timings = true ;

    let now = Instant::now();

    if timings { println!("START hack_chunks({}, {}): {}",
			  dim, step,
			  seconds(now.elapsed().as_secs_f64())) ; }

    let chunks : Vec<(u64, u64)> =
        (0..(dim as u64))
        .into_iter()
        .step_by(step)
        .map(|n| (n,min(n + step as u64, dim as u64)))
        .collect();

    let mut nnz = 0 as usize ;
    chunks.iter()
        .for_each(|(lo,hi)| {
            let rc : RowContents = hack_make_row(*hi - *lo, *lo) ;
	    let chunk_nnz = rc.0.len() ;
	    nnz += chunk_nnz ;
	    let mut indices : Vec<u64> = Vec::with_capacity(chunk_nnz as usize) ;
	    let mut data : Vec<Complex64> = Vec::with_capacity(chunk_nnz as usize) ;
	    let mut dst_rc = (indices, data) ;
	    append_rc(&mut dst_rc, &rc.0[..], &rc.1[..]) ;
        }) ;

    if timings { println!("AFTER CHUNKS hack_chunks: nnz={} {}",
			  number_(f64::value_from(nnz).unwrap()),
			  seconds(now.elapsed().as_secs_f64())) ; }
}
