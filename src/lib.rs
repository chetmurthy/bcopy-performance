#![allow(unused_imports)]
use std::mem::size_of ;
use num_complex::Complex64;
use std::cmp::min;
use std::time::Instant;
use std::fs::File;
use std::io;
use std::path::Path;
use rayon::prelude::*;

use si_scale::helpers::{seconds, number_, bibytes1};
use conv::prelude::* ;

pub struct RowContents {
    pub indices : Vec<u64>,
    pub data : Vec<Complex64>,
}

impl RowContents {
    pub fn capacity(size : usize) -> RowContents {
	let mut indices : Vec<u64> = Vec::with_capacity(size) ;
	let mut data : Vec<Complex64> = Vec::with_capacity(size) ;
	RowContents { indices, data }
    }

    pub fn new(size : usize, colval : u64, dataval : Complex64) -> RowContents {
	let mut indices : Vec<u64> = vec![colval; size] ;
	let mut data : Vec<Complex64> = vec![dataval; size] ;
	RowContents { indices, data }
    }

    pub fn append(&mut self, other : &RowContents) -> () {
	other.indices.iter().for_each(|v| self.indices.push(*v)) ;
	other.data.iter().for_each(|v| self.data.push(*v)) ;
    }

    pub fn clear(&mut self) -> () {
	self.indices.clear() ;
	self.data.clear() ;
    }

/*
pub fn append_rc1(lhs : &mut RowContents, colv : &[u64], datav : &[Complex64]) {
    lhs.0.copy_from_slice(colv) ;
    lhs.1.copy_from_slice(datav) ;
}
*/
}

pub fn hack_make_row(size : usize, rowind : u64)
		-> RowContents
{
    let recoeff = f64::value_from(rowind).unwrap() ;
    let coeff = Complex64::new(recoeff, 0.0) ;
    RowContents::new(size, rowind, coeff)
}


pub fn hack_chunks(iterations : usize,
		   nrows : usize,
		   size : usize,
) -> () {

    let debug = true ;
    let timings = true ;

    if timings { println!("START hack_chunks(iterations={}, nrows={}, size={})",
			  iterations, nrows, size) ; }

    let v_rc = {
	let setup = Instant::now();
	let v_rc : Vec<RowContents> =
	    (0..nrows).into_par_iter()
	    .map(|i| hack_make_row(size, i as u64))
	    .collect() ;
	let elapsed = setup.elapsed().as_secs_f64() ;
	let nnz = nrows * size ;
	let nbytes = nnz * (size_of::<u64>() + size_of::<Complex64>()) ;
	let f_nbytes = f64::value_from(nbytes).unwrap() ;
	let bytespersec = f_nbytes / elapsed ;
	if timings { println!("AFTER SETUP hack_chunks: nnz={}, {} bytes/sec {}",
			      number_(f64::value_from(nnz).unwrap()),
			      bibytes1(bytespersec),
			      seconds(elapsed)) ; }
	v_rc
    } ;


    let mut v_dst_rc : Vec<RowContents> =
	(0..nrows).into_iter()
	.map(|i| RowContents::capacity(size))
	.collect() ;

    {
	let copying = Instant::now();
	v_rc.par_iter()
	    .zip(v_dst_rc.par_iter_mut())
	    .for_each(|(rc,dst_rc)| {
		for i in 0..iterations {
		    dst_rc.append(&rc) ;
		    dst_rc.clear()
		}
	    }) ;

	let elapsed = copying.elapsed().as_secs_f64() ;
	let nnz = iterations * nrows * size ;
	let nbytes = nnz * (size_of::<u64>() + size_of::<Complex64>()) ;
	let f_nbytes = f64::value_from(nbytes).unwrap() ;
	let bytespersec = f_nbytes / elapsed ;

	if timings { println!("AFTER COPYING hack_chunks: nnz={}, {} bytes/sec {}",
			      number_(f64::value_from(nnz).unwrap()),
			      bibytes1(bytespersec),
			      seconds(elapsed)) ; }
    }
}
