#include <util.h>
#include <math.h>

ZZ_p util::InnerProduct(Vec<ZZ_p> a, Vec<ZZ_p> b){
  int i;
  ZZ_p res = conv<ZZ_p>(conv<ZZ>(0));
  for(i=0;i<a.length();i++){
    res += a[i]*b[i];
  }
  return res;
}

Mat<ZZ_p> util::Transpose(Mat<ZZ_p> a){
  Mat<ZZ_p> res;
  int i,j;
  res.SetDims(a.NumCols(), a.NumRows());
  for(i=0;i<a.NumRows();i++){
    for(j=0;j<a.NumCols();j++){
      res[j][i] = a[i][j];
    }
  }
  return res;
}

Mat<ZZ_p> util::Add(Mat<ZZ_p> a, Mat<ZZ_p> b){
  int i, j;
  Mat<ZZ_p> res;
  res.SetDims(a.NumRows(), a.NumCols()); 
  for (i=0; i<a.NumRows(); i++) {
    for (j=0; j<a.NumCols(); j++) {
      res[i][j] = a[i][j] + b[i][j];
    }
  }
  return res;
}

Vec<ZZ_p> util::Add(Vec<ZZ_p> a, Vec<ZZ_p> b){
  int i;
  Vec<ZZ_p> res;
  res.SetLength(a.length());
  for(i=0;i<a.length();i++){
    res[i] = a[i] + b[i];
  }
  return res;
}

Vec<ZZ_p> util::Mult(Mat<ZZ_p> a, Vec<ZZ_p> b){
  int i,j,k;
  Vec<ZZ_p> res;
  res.SetLength(a.NumRows());
  for(i=0;i<a.NumRows();i++){
    res[i] = conv<ZZ_p>(conv<ZZ>(0));
    for(k=0;k<b.length();k++){
      res[i] += a[i][k]*b[k];
    }
  }
  return res;
}

Mat<ZZ_p> util::Mult(Mat<ZZ_p> a, Mat<ZZ_p> b){
  int i,j,k;
  Mat<ZZ_p> res;
  res.SetDims(a.NumRows(), b.NumCols());
  for(i=0;i<a.NumRows();i++){
    for(j=0;j<b.NumCols();j++){
      res[i][j] = conv<ZZ_p>(conv<ZZ>(0));
      for(k=0;k<b.NumRows();k++){
        res[i][j] += a[i][k]*b[k][j];
      }
    }
  }
  return res;
}

ZZ_p util::Mod2(ZZ a){
  ZZ_p res;
  res = conv<ZZ_p>(a%conv<ZZ>(2));
  return res;
}

