#include <NTL/ZZ_p.h>
#include <NTL/ZZ.h>
#include <NTL/matrix.h>
#include <time.h> 


using namespace NTL;

class util {

  public:
    static Mat<ZZ_p> Transpose(Mat<ZZ_p> a);
    static ZZ_p InnerProduct(Vec<ZZ_p> a, Vec<ZZ_p> b);
    static Vec<ZZ_p> Add(Vec<ZZ_p> a, Vec<ZZ_p> b);
    static Mat<ZZ_p> Add(Mat<ZZ_p> a, Mat<ZZ_p> b);
    static Mat<ZZ_p> Mult(Mat<ZZ_p> a, Mat<ZZ_p> b);
    static Vec<ZZ_p> Mult(Mat<ZZ_p> a, Vec<ZZ_p> b);
    static ZZ_p Mod2(ZZ a);

};
