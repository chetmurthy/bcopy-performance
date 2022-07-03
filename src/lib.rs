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

    pub fn append_rc_slot0(lhs : &mut RowContents, colv : &[u64], datav : &[Complex64]) {
	colv.iter().for_each(|v| lhs.0.push(*v)) ;
    }

    pub fn append_rc_slot1(lhs : &mut RowContents, colv : &[u64], datav : &[Complex64]) {
	datav.iter().for_each(|v| lhs.1.push(*v)) ;
    }

    pub fn append_rc(lhs : &mut RowContents, colv : &[u64], datav : &[Complex64]) {
	append_rc_slot0(lhs, colv, datav);
	append_rc_slot1(lhs, colv, datav);
    }

    pub fn append_rc1(lhs : &mut RowContents, colv : &[u64], datav : &[Complex64]) {
	lhs.0.copy_from_slice(colv) ;
	lhs.1.copy_from_slice(datav) ;
    }

pub fn hack_make_row(size : usize, rowind : u64)
		-> RowContents
{
    let recoeff = f64::value_from(rowind).unwrap() ;
    let coeff = Complex64::new(recoeff, 0.0) ;
    let mut colw = vec![rowind; size] ;
    let mut dataw = vec![coeff; size] ;
    (colw, dataw)
}


pub fn hack_chunks(count : usize,
		   size : usize,
) -> () {

    let debug = true ;
    let timings = true ;

    let now = Instant::now();

    if timings { println!("START hack_chunks({}, {}): {}",
			  count, size,
			  seconds(now.elapsed().as_secs_f64())) ; }

    let mut nnz = 0 as usize ;
    let rc : RowContents = hack_make_row(size, count as u64) ;
    let mut indices : Vec<u64> = Vec::with_capacity(size) ;
    let mut data : Vec<Complex64> = Vec::with_capacity(size) ;
    let mut dst_rc = (indices, data) ;
    for i in 0..count {
	nnz += size ;
	append_rc(&mut dst_rc, &rc.0[..], &rc.1[..]) ;
	dst_rc.0.clear() ;
	dst_rc.1.clear() ;
    }

    if timings { println!("AFTER CHUNKS hack_chunks: nnz={} {}",
			  number_(f64::value_from(nnz).unwrap()),
			  seconds(now.elapsed().as_secs_f64())) ; }
}
