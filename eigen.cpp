#include <iostream>
#include <iomanip>
#include <chrono>
#define EIGEN_USE_MKL_ALL
#include <Eigen/Dense>
#include <Eigen/Core>

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

    Eigen::Map<Eigen::VectorXd> xv(x, n), yv(y, n);

    std::cout<< "MKL: " << std::endl;
    start = std::chrono::system_clock::now(); 
    res = xv.adjoint()*yv;
    end = std::chrono::system_clock::now(); 
    std::chrono::duration<double> elapsed_seconds = end - start; 
    std::cout<< "res: " << std::setprecision(12) << res << std::endl;
    std::cout<< "printf: " << elapsed_seconds.count() << " seconds" << std::endl;

    free(x);
    free(y);
    return 0;
}

