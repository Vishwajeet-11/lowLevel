#include<stdio.h>

void addition(int *aA){
    (*aA)++;
}

int main(){
    int x = 10;
    addition(&x);
    printf("the incremented value is: %d\n", *&x);
}