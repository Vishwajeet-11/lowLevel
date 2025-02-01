#include<stdio.h>

// pointers using for functions
void addition(int *aA, int *bB, int *Ssum){
     *Ssum = *aA + *bB;
}

int main(){
    int x = 10;
    int z = 12;
    int sum = 0;
    addition(&x, &z, &sum);
    printf("the sum of the values are: %d\n", *&sum);
}