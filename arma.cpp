#include <iostream>
#include <iomanip>
#include <chrono>
#define ARMA_USE_BLAS
#define ARMA_USE_LAPACK
#define ARMA_DONT_USE_OPENMP
#define ARMA_USE_CXX11
#include <armadillo>

int main() {
    std::chrono::time_point<std::chrono::system_clock> start, end; 

    int n, i;
    double *x, *y;
    double res;

    n = 100000000;
    x = (double *)calloc( n, sizeof( double ) );
    y = (double *)calloc( n, sizeof( double ) );

    for (i = 0; i < n; i++) {
        x[i] = 0.2;
        y[i] = 0.1;
    }

    arma::vec xv(x, n, false), yv(y, n, false);

    std::cout<< "Eigen: " << std::endl;
    start = std::chrono::system_clock::now(); 
    res = arma::dot(xv, yv);
    end = std::chrono::system_clock::now(); 
    std::chrono::duration<double> elapsed_seconds = end - start; 
    std::cout<< "res: " << std::setprecision(12) << res << std::endl;
    std::cout<< "printf: " << elapsed_seconds.count() << " seconds" << std::endl;

    free(x);
    free(y);
    return 0;
}
