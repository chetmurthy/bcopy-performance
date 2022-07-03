#include <chrono>
#include <algorithm>
#include <complex>
#include <vector>
#include <cstring>
#include <iostream>

using namespace std ;

typedef complex<double> complex64 ;

struct RowContents {
  vector<uint64_t> indices ;
  vector<complex64> data ;
  RowContents(uint64_t size) ;
  RowContents(uint64_t size, uint64_t colval, complex64& dataval) ;
  void clear() ;
  uint64_t append_via_insert(RowContents& other) ;
  uint64_t append_via_copy(RowContents& other) ;
  uint64_t append_via_memcpy(RowContents& other) ;
}  ;
RowContents::RowContents(uint64_t size) {
  indices.reserve(size) ;
  data.reserve(size) ;
}
RowContents::RowContents(uint64_t size, uint64_t colval, complex64& dataval) :
  indices(size, colval), data(size, dataval) { }

void RowContents::clear() {
  this->indices.resize(0) ;
  this->data.resize(0) ;
}

uint64_t RowContents::append_via_insert(RowContents& other) {
  this->indices.insert(this->indices.end(), other.indices.begin(), other.indices.end()) ;
  this->data.insert(this->data.end(), other.data.begin(), other.data.end()) ;
  return other.indices.size() * (sizeof(uint64_t) + sizeof(complex64)) ;
}

uint64_t RowContents::append_via_copy(RowContents& other) {
  std::copy(other.indices.begin(), other.indices.end(), std::back_inserter(this->indices));
  std::copy(other.data.begin(), other.data.end(), std::back_inserter(this->data));
  return other.indices.size() * (sizeof(uint64_t) + sizeof(complex64)) ;
}

uint64_t RowContents::append_via_memcpy(RowContents& other) {
  std::memcpy((void *)&(this->indices[this->indices.size()]), (void *)&(other.indices[0]), other.indices.size() * sizeof(uint64_t)) ;
  this->indices.resize(this->indices.size() + other.indices.size()) ;
  
  std::memcpy((void *)&(this->data[this->data.size()]), (void *)&(other.data[0]), other.data.size() * sizeof(complex64)) ;
  this->data.resize(this->data.size() + other.data.size()) ;
  
  return other.indices.size() * (sizeof(uint64_t) + sizeof(complex64)) ;
}

RowContents
hack_make_row(uint64_t size, uint64_t rowind) {
  double recoeff = rowind ;
  complex64 coeff = complex64(recoeff, 0.0) ;
  RowContents rc(size, rowind, coeff) ;
  return rc ;
}

uint64_t
hack_chunks(uint64_t count, uint64_t size) {
  uint64_t ncopied = 0 ;
  for (uint64_t i = 0; i < count ; i++) {
    RowContents rc = hack_make_row(size, i) ;
    RowContents dst_rc(size) ;
    ncopied += dst_rc.append_via_memcpy(rc) ;
  }
  return ncopied ;
}

uint64_t
hack_chunks1(uint64_t count, uint64_t size) {
  uint64_t ncopied = 0 ;
  RowContents rc = hack_make_row(size, count) ;
  RowContents dst_rc(size) ;
  for (uint64_t i = 0; i < count ; i++) {
    ncopied += dst_rc.append_via_memcpy(rc) ;
    dst_rc.clear() ;
  }
  return ncopied ;
}

int
main() {

  std::chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();
  uint64_t ncopied = hack_chunks1(1<<10, 1<<22) ;
  std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();

  /* Getting number of milliseconds as a double. */
  auto secs = std::chrono::duration_cast<std::chrono::seconds>(end - begin).count() ;

  cout << "ncopied: " << ncopied << std::endl ;
  double bytespersec = ncopied / secs ;
  std::cout << "persec: " << bytespersec << "bytes/sec\n";
  return(0) ;
}
