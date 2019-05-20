call "C:/Program Files (x86)/IntelSWTools/compilers_and_libraries_2019/windows/bin/compilervars.bat" intel64 vs2017
set ARMA_INC=C:/library/armadillo-9.400.3/include
set EIGEN_INC=C:/library/eigen-3.3.7
icl /Qmkl /O3 /QxHost /Qopenmp /DNDEBUG /Qparallel /EHsc mkl.cpp
icl /Qmkl /O3 /QxHost /Qopenmp /DNDEBUG /Qparallel /EHsc -I"%ARMA_INC%" arma.cpp
icl /Qmkl /O3 /QxHost /Qopenmp /DNDEBUG /Qparallel /EHsc -I"%EIGEN_INC%" eigen.cpp
cd rust_dotproduct & cargo +nightly build --release
